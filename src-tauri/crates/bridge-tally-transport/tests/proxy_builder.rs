//! A proxy configured on the client builder must not carry a Tally request
//! (#920): the transport applies its own proxy setting after the builder's.

mod proxy_support;

#[tokio::test]
async fn a_proxy_on_the_client_builder_does_not_carry_a_tally_request() {
    let observed = proxy_support::send_with_a_proxy_listening(
        Some(|proxy_url| {
            reqwest::Client::builder().proxy(reqwest::Proxy::all(proxy_url).expect("a proxy URL"))
        }),
        |_| {},
    )
    .await;
    assert_eq!(observed.proxy_received, 0, "the request went to the proxy");
    assert_eq!(
        observed.tally_received, 1,
        "the request did not reach Tally"
    );
    assert!(observed.text.contains("<STATUS>1</STATUS>"));
}
