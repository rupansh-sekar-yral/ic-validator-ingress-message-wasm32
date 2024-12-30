use ic_types::{messages::{HttpCallContent, HttpCanisterUpdate, HttpRequest, HttpRequestEnvelope}, CanisterId};
use ic_validator_ingress_message::{HttpRequestVerifier, IngressMessageVerifier};
use worker::*;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    let verifier = IngressMessageVerifier::default();
    let envelope = HttpRequestEnvelope {
        content: HttpCallContent::Call {
            update: HttpCanisterUpdate {
                canister_id: CanisterId::ic_00().into(),
                method_name: "test".into(),
                arg: vec![].into(),
                sender: CanisterId::ic_00().into(),
                ingress_expiry: u64::MAX,
                nonce: None,
            },
        },
        sender_pubkey: Some(vec![0].into()),
        sender_sig: Some(vec![0].into()),
        sender_delegation: None,
    };
    let req: HttpRequest<_> = envelope.try_into().unwrap();
    verifier.validate_request(&req).unwrap();
    Response::ok("Hello World!")
}