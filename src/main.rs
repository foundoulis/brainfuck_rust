pub mod logger;
use ast::Parseable;
pub mod ast;
// pub mod program;

fn main() {
    logger::setup_logger().unwrap();
    let program_string = "+[[->]-[-<]>-]>.>>awefasdf>>.<<<<-.>>-.>.<<.>>>>-.<<<<<++.>>++.";
    log::debug!("{}", program_string);
    let instruct = program_string.parse_to_bf();
    log::debug!("{:?}", instruct);
}
