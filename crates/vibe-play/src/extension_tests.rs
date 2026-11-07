use super::*;
#[test]
fn extension_route_returns_shared_descriptor_export() {
    let listener=TcpListener::bind("127.0.0.1:0").unwrap();
    let address=listener.local_addr().unwrap();
    let server=std::thread::spawn(move||handle(listener.accept().unwrap().0).unwrap());
    let mut client=TcpStream::connect(address).unwrap();
    client.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    client.write_all(b"GET /api/extensions HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
    let mut response=String::new();client.read_to_string(&mut response).unwrap();server.join().unwrap();
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    let json:serde_json::Value=serde_json::from_str(response.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(json,vibe_tooling::reference_preview().unwrap());
}

#[test]
fn descriptor_preview_is_valid_utf8() {
    let (_, bytes) = serve_static(&project_root(), "playground/extensions.html").unwrap();
    let page = std::str::from_utf8(&bytes).expect("preview declares UTF-8");
    assert!(page.contains("/api/extensions"));
    assert!(page.contains("Discovery does not grant authority"));
}
