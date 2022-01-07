pub mod ast;
pub mod logger;
pub mod run;

use ast::{Parseable, Runnable, RuntimeError};

fn main() -> Result<(), RuntimeError> {
    logger::setup_logger().unwrap();
    let program_string = "+++++[-]";
    log::debug!("{}", program_string);
    let instruct = program_string.parse_to_bf();
    log::debug!("{:?}", instruct);
    instruct.0.run(instruct.1)?;
    Ok(())
}
