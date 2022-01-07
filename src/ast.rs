use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    fn parse_to_bf(&self) -> (Instructions, HashMap<usize, usize>) {
        let instuctions = self
            .chars()
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
            .collect::<Instructions>();

        let mut map = HashMap::new();

        let mut opening = Vec::new();
        for (i, c) in instuctions.iter().enumerate() {
            match c {
                Instruction::STARTLOOP => {
                    opening.push(i);
                }
                Instruction::ENDLOOP => {
                    let begin = opening.pop().unwrap();
                    map.insert(begin, i);
                }
                _ => {}
            }
        }
        if !opening.is_empty() {}

        (instuctions, map)
    }
}

pub trait Parseable<T> {
    fn parse_to_bf(&self) -> (Instructions, HashMap<usize, usize>);
}

#[derive(Debug)]
pub struct RuntimeError;
pub trait Runnable {
    fn run(&self, bracket_map: HashMap<usize, usize>) -> Result<(), RuntimeError>;
}
