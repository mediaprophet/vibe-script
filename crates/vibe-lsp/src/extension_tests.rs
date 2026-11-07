use crate::server::{encode_message, read_message, LspServer};
use std::io::{BufReader, Cursor};
#[test]
fn discovery_request_returns_actual_manifest_without_executable_claims() {
    let input=encode_message(r#"{"jsonrpc":"2.0","id":9,"method":"vibe/extensionManifest"}"#);
    let mut output=Vec::new();
    LspServer::new(BufReader::new(Cursor::new(input)),&mut output).run().unwrap();
    let response=read_message(&mut BufReader::new(Cursor::new(output))).unwrap().unwrap();
    let v:serde_json::Value=serde_json::from_str(&response).unwrap();
    assert_eq!(v["id"],9);
    assert_eq!(v["result"],vibe_tooling::reference_preview().unwrap());
    assert_eq!(v["result"]["manifest"]["authorization"],"not_assessed");
}
