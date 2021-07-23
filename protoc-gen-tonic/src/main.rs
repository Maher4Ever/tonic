use std::io::{Read, Result, Write};

use prost::Message;
use prost_build::Config;
use prost_types::compiler::CodeGeneratorRequest;

fn main() {
    if let Err(e) = faillible_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn faillible_main() -> Result<()> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;

    let req = CodeGeneratorRequest::decode(buf.as_slice()).unwrap();
    let cfg = Config::new_from_opts(req.parameter(), false);
    let res = tonic_build::configure()
        .opts(req.parameter())
        .compile_request_with_config(cfg, req);

    buf.clear();
    res.encode(&mut buf).unwrap();
    std::io::stdout().write_all(&buf)?;

    Ok(())
}
