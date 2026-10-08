#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;

#[test]
fn tally_xml_content_type_classifiers_are_exact_and_case_insensitive() {
    assert!(has_tally_xml_utf16_content_type(
        b"POST / HTTP/1.1\r\ncontent-type: TEXT/XML; CHARSET=UTF-16\r\n\r\n<E />"
    ));
    assert!(!has_plain_tally_xml_content_type(
        b"POST / HTTP/1.1\r\ncontent-type: TEXT/XML; CHARSET=UTF-16\r\n\r\n<E />"
    ));
    assert!(has_plain_tally_xml_content_type(
        b"GET /status HTTP/1.1\r\ncontent-type: TEXT/XML\r\n\r\n"
    ));
    assert!(!has_tally_xml_utf16_content_type(
        b"GET /status HTTP/1.1\r\ncontent-type: TEXT/XML\r\n\r\n"
    ));
    assert!(!has_tally_xml_utf16_content_type(
        b"POST / HTTP/1.1\r\nContent-Type: application/xml; charset=utf-8\r\n\r\n<E />"
    ));
    assert!(!has_tally_xml_utf16_content_type(
        b"POST / HTTP/1.1\r\nContent-Type: text/xml; charset=utf-8\r\nContent-Type: application/xml\r\n\r\n<E />"
    ));
}

#[test]
fn read_request_enforces_deadline_while_a_peer_drip_feeds_bytes() {
    // The peer sends a byte every 15 ms, and the poll waits up to 1 s, far longer
    // than the 70 ms deadline and than any oversleep of the peer's 15 ms pause, so
    // only the deadline check on a byte that arrived can end the read in time. The
    // peer keeps sending until the reader is done, or for 10 s at most, so a read
    // that ignored the deadline would run until the peer gave up and closed. No
    // wall-clock ceiling is asserted (#998): the deadline is shown by the peer
    // still sending when the reader stopped.
    let listener = bind_loopback_listener().expect("bind loopback listener");
    let address = listener.local_addr().expect("read listener address");
    let reader_done = std::sync::Arc::new(AtomicBool::new(false));
    let peer_done = std::sync::Arc::clone(&reader_done);
    let writer = thread::spawn(move || {
        let mut stream = TcpStream::connect(address).expect("connect loopback listener");
        stream.set_nodelay(true).expect("disable Nagle buffering");
        let give_up = Instant::now() + Duration::from_secs(10);
        while Instant::now() < give_up {
            if peer_done.load(Ordering::Acquire) {
                return true;
            }
            if stream.write_all(b"x").is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(15));
        }
        false
    });
    let (mut stream, _) = listener.accept().expect("accept loopback peer");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("set read poll timeout");
    let cancelled = AtomicBool::new(false);
    let started = Instant::now();

    let error = read_request(&mut stream, &cancelled, Duration::from_millis(70))
        .expect_err("incomplete drip feed must not outlive its deadline");

    assert!(
        matches!(
            error,
            ReadError::Refused(RequestFault::Incomplete { expected: None, .. })
        ),
        "{error:?}"
    );
    // Not before the deadline: the reader's own clock starts after this one.
    assert!(started.elapsed() >= Duration::from_millis(70));
    reader_done.store(true, Ordering::Release);
    assert!(
        writer.join().expect("drip-feed writer does not panic"),
        "the read outlived the drip feed, so the deadline did not end it"
    );
}

#[test]
fn a_plan_that_no_request_reaches_is_named_by_its_place_in_the_sequence() {
    // The first request is queued on the listener before serving starts, as a
    // client's is when it connects before the worker runs (#1248). So only the
    // second plan can wait out the deadline, and a stall can only lengthen that.
    let listener = bind_loopback_listener().expect("bind loopback listener");
    listener
        .set_nonblocking(true)
        .expect("set the listener non-blocking");
    let address = listener.local_addr().expect("read listener address");
    let mut client = TcpStream::connect(address).expect("connect before serving starts");
    client
        .write_all(b"GET /status HTTP/1.1\r\n\r\n")
        .expect("queue the first request");
    let status = || {
        ScenarioPlan::new(crate::Fixture::ProductStatus(
            crate::ProductStatus::TallyPrime,
        ))
    };
    let received = Arc::new(AtomicUsize::new(0));

    let error = serve_sequence(
        listener,
        vec![status(), status()],
        Arc::new(AtomicBool::new(false)),
        Arc::clone(&received),
        Duration::from_secs(1),
    )
    .expect_err("no request reaches the second plan");

    assert_eq!(
        received.load(Ordering::Acquire),
        1,
        "the queued request is served"
    );
    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    assert_eq!(
        error
            .get_ref()
            .and_then(|inner| inner.downcast_ref::<NoRequestForPlan>()),
        Some(&NoRequestForPlan { plan: 1, plans: 2 })
    );
    drop(client);
}

/// Serves one plan to `raw`, queued on the listener before serving starts as
/// in the test above, and returns the result with the count of requests read
/// in full. The client keeps its stream open unless `end_stream`.
fn serve_queued(
    raw: &[u8],
    end_stream: bool,
    deadline: Duration,
) -> (io::Result<Vec<ObservedRequest>>, usize) {
    let listener = bind_loopback_listener().expect("bind loopback listener");
    listener
        .set_nonblocking(true)
        .expect("set the listener non-blocking");
    let address = listener.local_addr().expect("read listener address");
    let mut client = TcpStream::connect(address).expect("connect before serving starts");
    client.write_all(raw).expect("queue the request");
    if end_stream {
        client
            .shutdown(Shutdown::Write)
            .expect("end the request stream");
    }
    let received = Arc::new(AtomicUsize::new(0));
    let result = serve_sequence(
        listener,
        vec![ScenarioPlan::new(crate::Fixture::ExportStatusOne)],
        Arc::new(AtomicBool::new(false)),
        Arc::clone(&received),
        deadline,
    );
    drop(client);
    (result, received.load(Ordering::Acquire))
}

fn refusal(error: &io::Error) -> Option<RefusedRequest> {
    error
        .get_ref()
        .and_then(|inner| inner.downcast_ref::<RefusedRequest>())
        .copied()
}

#[test]
fn a_request_cut_short_is_refused_as_such_and_not_counted() {
    // Before #1148 the first case was dropped and reported as no request at
    // all, and the second was served as a whole request and counted.
    let head = "POST / HTTP/1.1\r\nContent-Length: 10\r\n\r\n";
    let raw = format!("{head}ab");
    for (end_stream, fault) in [
        (
            false,
            RequestFault::Incomplete {
                received: raw.len(),
                expected: Some(head.len() + 10),
            },
        ),
        (
            true,
            RequestFault::EndedEarly {
                received: raw.len(),
                expected: Some(head.len() + 10),
            },
        ),
    ] {
        let (result, received) = serve_queued(raw.as_bytes(), end_stream, Duration::from_secs(1));
        let error = result.expect_err("a request cut short is refused");
        assert_eq!(
            refusal(&error),
            Some(RefusedRequest {
                plan: 0,
                plans: 1,
                fault
            })
        );
        assert!(
            error
                .get_ref()
                .and_then(|inner| inner.downcast_ref::<NoRequestForPlan>())
                .is_none(),
            "a request did arrive"
        );
        assert_eq!(received, 0, "a refused request is not read in full");
    }
}

#[test]
fn bytes_past_the_declared_length_in_the_same_read_are_refused() {
    for (raw, fault) in [
        (
            "POST / HTTP/1.1\r\nContent-Length: 2\r\n\r\nhello",
            RequestFault::PastDeclaredLength {
                declared: 2,
                received: 5,
            },
        ),
        // A `GET` without `Content-Length` has no body (RFC 9112 §6.3).
        (
            "GET /status HTTP/1.1\r\n\r\nxyz",
            RequestFault::PastDeclaredLength {
                declared: 0,
                received: 3,
            },
        ),
    ] {
        let (result, received) = serve_queued(raw.as_bytes(), false, Duration::from_secs(1));
        let error = result.expect_err("bytes past the declared length are refused");
        assert_eq!(
            refusal(&error),
            Some(RefusedRequest {
                plan: 0,
                plans: 1,
                fault
            }),
            "{raw:?}"
        );
        assert_eq!(received, 0);
    }
}

#[test]
fn bytes_after_a_whole_request_are_never_read() {
    // The body arrives after the head, with three bytes more than declared.
    // The read asks only for the two declared bytes, so the request is whole
    // and its recorded body is exactly the declared length.
    let listener = bind_loopback_listener().expect("bind loopback listener");
    listener
        .set_nonblocking(true)
        .expect("set the listener non-blocking");
    let address = listener.local_addr().expect("read listener address");
    let mut client = TcpStream::connect(address).expect("connect before serving starts");
    client
        .write_all(b"POST / HTTP/1.1\r\nContent-Length: 2\r\n\r\n")
        .expect("queue the head");
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        client.write_all(b"hello").map(|()| client)
    });

    let observed = serve_sequence(
        listener,
        vec![ScenarioPlan::new(crate::Fixture::ExportStatusOne)],
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicUsize::new(0)),
        Duration::from_secs(5),
    )
    .expect("the declared request is served");

    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0].request_body_bytes, 2);
    assert_eq!(
        observed[0].request_body_sha256,
        hex::encode(Sha256::digest(b"he"))
    );
    assert!(observed[0].request_processed);
    drop(writer.join().expect("the body writer does not panic"));
}

#[test]
fn a_whole_request_is_whole_even_when_the_deadline_has_passed() {
    // With a zero deadline every read is late. The request is whole on its
    // first read, and completeness is decided before the deadline.
    let listener = bind_loopback_listener().expect("bind loopback listener");
    let address = listener.local_addr().expect("read listener address");
    let raw = b"GET /status HTTP/1.1\r\n\r\n";
    let mut client = TcpStream::connect(address).expect("connect loopback listener");
    client.write_all(raw).expect("write the request");
    let (mut stream, _) = listener.accept().expect("accept loopback peer");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("set read poll timeout");

    let read = read_request(&mut stream, &AtomicBool::new(false), Duration::ZERO)
        .expect("a whole request is read");

    assert!(
        matches!(&read, ReadOutcome::Whole(request) if request == raw),
        "{read:?}"
    );
    drop(client);
}

#[test]
fn the_declared_frame_is_parsed_once_and_fails_closed() {
    let get = b"GET /status HTTP/1.1\r\nContent-Type: text/xml\r\n\r\n";
    assert_eq!(parse_head(&get[..get.len() - 1]), Ok(None));
    assert_eq!(
        parse_head(get),
        Ok(Some(Frame {
            head: get.len(),
            total: get.len()
        }))
    );
    let post = b"POST / HTTP/1.1\r\ncontent-length:\t4 \r\n\r\nbody";
    assert_eq!(
        parse_head(post),
        Ok(Some(Frame {
            head: post.len() - 4,
            total: post.len()
        }))
    );
    for (raw, fault) in [
        (
            "POST / HTTP/1.1\r\n\r\n".to_owned(),
            RequestFault::MissingContentLength,
        ),
        (
            "POST / HTTP/1.1\r\nContent-Length: 4 2\r\n\r\n".to_owned(),
            RequestFault::UnparseableContentLength,
        ),
        (
            "POST / HTTP/1.1\r\nContent-Length: 2\r\nContent-Length: 3\r\n\r\n".to_owned(),
            RequestFault::RepeatedContentLength,
        ),
        // The value fits in `usize`, so only the checked add can refuse it.
        (
            format!("GET / HTTP/1.1\r\nContent-Length: {}\r\n\r\n", usize::MAX),
            RequestFault::LengthOverflow,
        ),
        (
            format!(
                "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
                MAX_REQUEST_BYTES
            ),
            RequestFault::TooLarge {
                bytes: "POST / HTTP/1.1\r\nContent-Length: \r\n\r\n".len()
                    + MAX_REQUEST_BYTES.to_string().len()
                    + MAX_REQUEST_BYTES,
            },
        ),
    ] {
        assert_eq!(parse_head(raw.as_bytes()), Err(fault), "{raw:?}");
    }
}
