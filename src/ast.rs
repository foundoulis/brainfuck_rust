use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub enum Instruction {
    MOVELEFT,
    MOVERIGHT,
    INCR,
    DECR,
    INPUT,
    OUTPUT,
    STARTLOOP,
    ENDLOOP,
}

pub type Instructions = Vec<Instruction>;

impl Parseable<Instructions> for &str {
    fn parse_to_bf(&self) -> Instructions {
        self.chars()
            .filter_map(|c| match c {
                '<' => Some(Instruction::MOVELEFT),
                '>' => Some(Instruction::MOVERIGHT),
                '+' => Some(Instruction::INCR),
                '-' => Some(Instruction::DECR),
                ',' => Some(Instruction::INPUT),
                '.' => Some(Instruction::OUTPUT),
                '[' => Some(Instruction::STARTLOOP),
                ']' => Some(Instruction::ENDLOOP),
                _ => {
                    log::warn!("Invalid instruction found: {}", c);
                    log::warn!("Ignoring...");
                    None
                }
            })
            .collect::<Instructions>()
    }
}

pub struct RuntimeError;
pub struct ParsingError;

pub trait Parseable<T> {
    fn parse_to_bf(&self) -> Instructions;
}

pub trait Runnable {
    fn run(&self) -> Result<(), RuntimeError>;
}
