use jsonrpc_core::{IoHandler, Params, Value};
use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    let mut handler = IoHandler::new();
    handler.add_sync_method("say_hello", |_: Params| Ok(Value::String("Hello World!".to_owned())));

    for req in io::stdin().lock().lines() {
        if let Some(rsp) = handler.handle_request_sync(&req?) {
            println!("{}", rsp);
        }
    }
    Ok(())
}
