//! Local playground: `127.0.0.1:7420`. Diagnose, format, and eval on LocalHost.
//! Browser WASM is optional; this is the honest desktop Check/Eval server.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use vibe::projectional::{project_program, ProjectOptions};
use vibe::{
    check_program, diagnose, parse_program, Budget, Engine, Env, LocalHost,
};

fn main() {
    let addr = std::env::var("VIBE_PLAY_ADDR").unwrap_or_else(|_| "127.0.0.1:7420".into());
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("vibe-play: bind {addr}: {e}");
        std::process::exit(1);
    });
    eprintln!("vibe-play running at http://{addr}");
    eprintln!("  Playground: http://{addr}/");
    eprintln!("  Showcase:   http://{addr}/docs/showcase.html");
    eprintln!("  API Docs:   http://{addr}/docs/dev-docs.html");
    eprintln!("  Endpoints:  POST /api/diagnose | POST /api/eval | POST /api/format");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                if let Err(e) = handle(s) {
                    eprintln!("vibe-play: {e}");
                }
            }
            Err(e) => eprintln!("vibe-play accept: {e}"),
        }
    }
}

fn project_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or(manifest)
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "json" => "application/json",
        "js" | "mjs" => "application/javascript",
        "css" => "text/css; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "ebnf" | "gbnf" | "vibe" | "md" | "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn serve_static(root: &Path, rel_path: &str) -> Option<(&'static str, Vec<u8>)> {
    let clean = rel_path.trim_start_matches('/');
    // Prevent directory traversal
    if clean.contains("..") {
        return None;
    }
    let target = root.join(clean);
    if !target.exists() || !target.is_file() {
        return None;
    }
    let ctype = content_type_for_path(&target);
    let bytes = std::fs::read(&target).ok()?;
    Some((ctype, bytes))
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(8192);
    let mut temp = [0u8; 4096];

    // Read headers
    let mut header_end = None;
    while header_end.is_none() {
        let n = stream.read(&mut temp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            header_end = Some(pos);
            break;
        }
        if buf.len() > 1_048_576 {
            // Header too large
            return respond_status(&mut stream, "413 Payload Too Large", b"Header too large");
        }
    }

    let header_pos = match header_end {
        Some(p) => p,
        None => return Ok(()),
    };

    let header_str = String::from_utf8_lossy(&buf[..header_pos]);
    let mut lines = header_str.lines();
    let request_line = lines.next().unwrap_or("");
    let mut req_parts = request_line.split_whitespace();
    let method = req_parts.next().unwrap_or("GET").to_uppercase();
    let raw_path = req_parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or("/");

    let mut content_length: usize = 0;
    for line in lines {
        if let Some(val) = line.to_lowercase().strip_prefix("content-length:") {
            if let Ok(len) = val.trim().parse::<usize>() {
                content_length = len;
            }
        }
    }

    let body_start = header_pos + 4;
    let mut body_bytes = buf[body_start..].to_vec();
    while body_bytes.len() < content_length {
        let n = stream.read(&mut temp)?;
        if n == 0 {
            break;
        }
        body_bytes.extend_from_slice(&temp[..n]);
    }
    if body_bytes.len() > content_length && content_length > 0 {
        body_bytes.truncate(content_length);
    }
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    let root = project_root();

    match (method.as_str(), path) {
        ("OPTIONS", _) => {
            let resp = b"HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            stream.write_all(resp)?;
            Ok(())
        }
        ("GET", "/") | ("GET", "/index.html") | ("GET", "/playground") | ("GET", "/playground/") => {
            if let Some((ctype, bytes)) = serve_static(&root, "playground/index.html") {
                respond(&mut stream, ctype, &bytes)
            } else {
                let html = b"<!DOCTYPE html><html><body><h1>VibeScript Playground</h1><p>missing playground/index.html</p></body></html>";
                respond(&mut stream, "text/html; charset=utf-8", html)
            }
        }
        ("GET", p) if p.starts_with("/docs/") || p.starts_with("/playground/") || p.starts_with("/grammar/") || p.starts_with("/skills/") => {
            let rel = p.trim_start_matches('/');
            if let Some((ctype, bytes)) = serve_static(&root, rel) {
                respond(&mut stream, ctype, &bytes)
            } else {
                respond_status(&mut stream, "404 Not Found", b"File not found")
            }
        }
        ("POST", "/api/diagnose") => {
            let json = diagnose_src(&body);
            respond(
                &mut stream,
                "application/json",
                json.to_string().as_bytes(),
            )
        }
        ("POST", "/api/eval") => {
            let json = eval_src(&body);
            respond(
                &mut stream,
                "application/json",
                json.to_string().as_bytes(),
            )
        }
        ("POST", "/api/format") => {
            let json = format_src(&body);
            respond(
                &mut stream,
                "application/json",
                json.to_string().as_bytes(),
            )
        }
        _ => respond_status(&mut stream, "404 Not Found", b"Not found"),
    }
}

fn respond(stream: &mut TcpStream, ctype: &str, body: &[u8]) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}

fn respond_status(stream: &mut TcpStream, status: &str, body: &[u8]) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}

pub fn diagnose_src(src: &str) -> serde_json::Value {
    let report = diagnose(src);
    let errors: Vec<serde_json::Value> = report
        .errors
        .iter()
        .map(|e| {
            serde_json::json!({
                "code": e.code.as_str(),
                "message": e.message,
                "span": [e.span.start, e.span.end],
            })
        })
        .collect();
    serde_json::json!({
        "ok": report.valid,
        "valid": report.valid,
        "kind": report.kind,
        "errors": errors,
    })
}

pub fn format_src(src: &str) -> serde_json::Value {
    match parse_program(src) {
        Ok(prog) => {
            let formatted = project_program(&prog, &ProjectOptions::default());
            serde_json::json!({
                "ok": true,
                "formatted": formatted,
            })
        }
        Err(e) => serde_json::json!({
            "ok": false,
            "error": {
                "code": e.code.as_str(),
                "message": e.message,
                "span": [e.span.start, e.span.end],
            }
        }),
    }
}

pub fn eval_src(src: &str) -> serde_json::Value {
    match parse_program(src).and_then(|p| check_program(&p).map(|_| p)) {
        Ok(prog) => {
            let mut host = LocalHost::default();
            let mut env = Env::default();
            let mut engine = Engine::with_program(&mut host, Budget::default(), &prog);
            let first = match engine.eval_program(&prog, &mut env) {
                Ok(v) => v,
                Err(e) => {
                    return serde_json::json!({
                        "ok": false,
                        "error": { "code": e.code.as_str(), "message": e.message }
                    });
                }
            };
            let has_main = prog.items.iter().any(|i| matches!(i, vibe::Item::Function(f) if f.name == "main"));
            let value = if has_main {
                match engine.call_function(&prog, "main", Vec::new(), &mut env) {
                    Ok(v) => v,
                    Err(e) => {
                        return serde_json::json!({
                            "ok": false,
                            "error": { "code": e.code.as_str(), "message": e.message }
                        });
                    }
                }
            } else {
                first
            };
            serde_json::json!({ "ok": true, "value": value.to_string() })
        }
        Err(e) => serde_json::json!({
            "ok": false,
            "error": { "code": e.code.as_str(), "message": e.message }
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_calls_main() {
        let v = eval_src("fn main() { return 1 + 2; }");
        assert_eq!(v.get("ok"), Some(&serde_json::json!(true)));
        assert_eq!(v.get("value").and_then(|x| x.as_str()), Some("3"));
    }

    #[test]
    fn eval_handles_arithmetic_error() {
        let v = eval_src("fn main() { return 1 / 0; }");
        assert_eq!(v.get("ok"), Some(&serde_json::json!(false)));
        assert!(v.get("error").is_some());
    }

    #[test]
    fn diagnose_valid_code() {
        let v = diagnose_src("fn main() -> i64 { return 42; }");
        assert_eq!(v.get("ok"), Some(&serde_json::json!(true)));
        assert_eq!(v.get("valid"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn diagnose_invalid_syntax() {
        let v = diagnose_src("fn main( { return 42; }");
        assert_eq!(v.get("ok"), Some(&serde_json::json!(false)));
        let errors = v.get("errors").and_then(|e| e.as_array()).unwrap();
        assert!(!errors.is_empty());
    }

    #[test]
    fn format_valid_program() {
        let v = format_src("fn main() -> i64 { return 42; }");
        assert_eq!(v.get("ok"), Some(&serde_json::json!(true)));
        let formatted = v.get("formatted").and_then(|s| s.as_str()).unwrap();
        assert!(formatted.contains("fn main()"));
    }

    #[test]
    fn serve_static_finds_playground() {
        let root = project_root();
        let res = serve_static(&root, "playground/index.html");
        assert!(res.is_some());
        let (ctype, bytes) = res.unwrap();
        assert_eq!(ctype, "text/html; charset=utf-8");
        assert!(bytes.len() > 100);
    }

    #[test]
    fn serve_static_blocks_path_traversal() {
        let root = project_root();
        let res = serve_static(&root, "../../../Cargo.toml");
        assert!(res.is_none());
    }
}
