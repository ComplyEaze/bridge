//! A proxy set in the environment must not carry a Tally request (#920).
//!
//! This is the only test in its binary: it changes the process environment,
//! which other tests in the same process could otherwise read while it runs.

mod proxy_support;

#[tokio::test]
async fn a_proxy_in_the_environment_does_not_carry_a_tally_request() {
    // Every variable reqwest reads for an HTTP request, and none exempting
    // loopback, so only the transport's own setting can keep the request off
    // the proxy.
    let observed = proxy_support::send_with_a_proxy_listening(None, |proxy_url| {
        for name in ["HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"] {
            std::env::set_var(name, proxy_url);
        }
        for name in ["NO_PROXY", "no_proxy"] {
            std::env::remove_var(name);
        }
    })
    .await;
    assert_eq!(observed.proxy_received, 0, "the request went to the proxy");
    assert_eq!(observed.tally_received, 1, "the request did not reach Tally");
    assert!(observed.text.contains("<STATUS>1</STATUS>"));
}
