use jsonrpc_core::{IoHandler, Params, Value};
use std::{env, fs::File, io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    let is_debug = env::args().any(|s| s == "--debug");
    let mut dbgout: Box<dyn Write> = if is_debug {
        // println!("is debug");
        match File::create("/sandbox/autorust.log") {
            Ok(file) => Box::new(file),
            Err(e) => {
                println!("Could not create file: {}", e);
                Box::new(io::sink())
            }
        }
    } else {
        Box::new(io::sink())
    };

    let mut handler = IoHandler::new();
    handler.add_sync_method("say_hello", |_: Params| Ok(Value::String("Hello World!".to_owned())));

    handler.add_sync_method("GetPluginNames", |_: Params| {
        Ok(Value::Array(vec![Value::String("autorust".to_owned())]))
    });

    // let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        // Language Server Protocol - Base Protocol
        // "The base protocol consists of a header and a content part (comparable to HTTP). The header and content part are separated by a '\r\n'."
        // https://github.com/microsoft/language-server-protocol/blob/gh-pages/_specifications/specification-3-15.md#-base-protocol-
        
        // read headers until an empty line
        for hdr in &mut lines {
            let hdr = hdr?;
            writeln!(&mut dbgout, "hdr {}", hdr)?;
            if hdr.is_empty() {
                break;
            }
        }

        // read a single line containing the json
        for req in &mut lines {
            let req = req?;
            writeln!(&mut dbgout, "req {}", req)?;
            if let Some(rsp) = handler.handle_request_sync(&req) {
                writeln!(&mut dbgout, "rsp {}", rsp)?;
                // writeln!(&mut stdout, "{}", rsp)?;
                println!("{}", rsp);
            }
            break;
        }
    }
    // Ok(())
}
