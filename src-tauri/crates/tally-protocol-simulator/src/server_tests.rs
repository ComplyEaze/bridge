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

    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    // Not before the deadline: the reader's own clock starts after this one.
    assert!(started.elapsed() >= Duration::from_millis(70));
    reader_done.store(true, Ordering::Release);
    assert!(
        writer.join().expect("drip-feed writer does not panic"),
        "the read outlived the drip feed, so the deadline did not end it"
    );
}
