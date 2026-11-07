fn main() {
    let output = vibe_tooling::reference_preview().and_then(|v| vibe_tooling::manifest_bytes(&v));
    match output {
        Ok(bytes) => println!("{}",String::from_utf8(bytes).expect("UTF-8 JSON")),
        Err(e) => { eprintln!("Descriptor export failed: {e:?}"); std::process::exit(1); }
    }
}
