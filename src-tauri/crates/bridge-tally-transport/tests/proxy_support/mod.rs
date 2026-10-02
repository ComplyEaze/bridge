//! Shared by the two proxy tests (#920): one Tally request sent while a proxy
//! stand-in is listening, and which of the two stand-ins received it.

use bridge_tally_transport::{
    TallyEndpointConfig, TallyHttpTransport, TallyTransportError, TransportPolicy,
};
use tally_protocol_simulator::{Fixture, ScenarioPlan, SequenceSimulator, WireEncoding};

/// What one exchange observed.
pub struct Observed {
    pub proxy_received: usize,
    pub tally_received: usize,
    pub text: String,
}

/// Sends one request to a loopback Tally stand-in while a proxy stand-in
/// listens. `before_send` and `builder` receive the proxy's URL
/// (`http://<address>`); with no `builder` the transport builds its own client.
pub async fn send_with_a_proxy_listening(
    builder: Option<fn(&str) -> reqwest::ClientBuilder>,
    before_send: fn(&str),
) -> Observed {
    const MAX_HOST_ABORT_ATTEMPTS: usize = 3;
    for attempt in 1..=MAX_HOST_ABORT_ATTEMPTS {
        let tally = SequenceSimulator::spawn(vec![
            ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le),
        ])
        .expect("spawn the loopback Tally stand-in");
        let proxy = SequenceSimulator::spawn(vec![ScenarioPlan::new(Fixture::SyntheticXml(
            "<ENVELOPE>proxy</ENVELOPE>".to_owned(),
        ))
        .with_encoding(WireEncoding::Utf16Le)])
        .expect("spawn the loopback proxy stand-in");
        let proxy_url = format!("http://{}", proxy.address());
        before_send(&proxy_url);
        let endpoint = TallyEndpointConfig {
            host: tally.address().ip().to_string(),
            port: tally.address().port(),
        };
        let transport = match builder {
            Some(builder) => TallyHttpTransport::with_builder(
                endpoint,
                TransportPolicy::default(),
                builder(&proxy_url),
            ),
            None => TallyHttpTransport::new(endpoint),
        }
        .expect("build the transport");
        let result = transport.post_xml("<ENVELOPE />".to_owned()).await;
        let observed = (proxy.received(), tally.received());
        proxy.cancel();
        tally.cancel();
        // Windows endpoint-security software can abort a new loopback flow
        // before any response, as in http_transport.rs. Only that failure is
        // retried, on fresh stand-ins. A request carried by the proxy is
        // answered by it, so it is never retried here.
        if attempt < MAX_HOST_ABORT_ATTEMPTS
            && matches!(&result, Err(TallyTransportError::RequestFailed))
            && observed == (0, 0)
        {
            continue;
        }
        let response = result.expect("one stand-in answers");
        return Observed {
            proxy_received: observed.0,
            tally_received: observed.1,
            text: response.text().to_owned(),
        };
    }
    unreachable!("bounded attempts always return")
}
