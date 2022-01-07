pub mod ast;
pub mod logger;
pub mod run;

use ast::{Parseable, Runnable, RuntimeError};

fn main() -> Result<(), RuntimeError> {
    logger::setup_logger().unwrap();
    let program_string = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-]<.>+++++++++++[<++++++++>-]<-.--------.+++.------.--------.[-]>++++++++[<++++>-]<+.[-]++++++++++.";
    log::debug!("{:?}", program_string);
    match program_string.parse_to_bf() {
        Ok((instruct, bracket_map)) => {
            log::debug!("{:?}", instruct);
            instruct.run(bracket_map)?;
        }
        Err(err) => {
            log::error!("Parsing error!");
            log::error!("{:?}", err);
        }
    };
    Ok(())
}
