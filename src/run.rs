use crate::ast::{Instruction, Instructions, Runnable, RuntimeError};
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::io::Read;

impl Runnable for Instructions {
    fn run(&self, bracket_map: HashMap<usize, usize>) -> Result<(), RuntimeError> {
        let mut ctx = Context::new(self, &bracket_map);
        while ctx.has_next() {
            log::debug!(
                "{:?} - {:?} - {:?} - {:?}",
                ctx.get_inst(),
                ctx.memory_pointer,
                ctx.memory,
                ctx.start_stack
            );
            match ctx.get_inst() {
                Instruction::MOVELEFT => {
                    ctx.memory_pointer -= 1;
                    ctx.next_inst();
                }
                Instruction::MOVERIGHT => {
                    ctx.memory_pointer += 1;
                    ctx.next_inst();
                }
                Instruction::INCR => {
                    ctx.incr();
                    ctx.next_inst();
                }
                Instruction::DECR => {
                    ctx.decr();
                    ctx.next_inst();
                }
                Instruction::OUTPUT => {
                    log::info!("{}", ctx.get_byte() as char);
                    ctx.next_inst();
                }
                Instruction::INPUT => {
                    ctx.add_byte(
                        io::stdin()
                            .bytes()
                            .next()
                            .expect("Expected byte input.")
                            .ok()
                            .unwrap(),
                    );
                    ctx.next_inst();
                }
                Instruction::STARTLOOP => {
                    if ctx.get_byte() > 0 {
                        ctx.start_stack.push(ctx.program_pointer);
                    } else {
                        // go to the ending bracket.
                        ctx.program_pointer = *ctx.bracket_map.get(&ctx.program_pointer).unwrap();
                    }
                    ctx.next_inst();
                }
                Instruction::ENDLOOP => {
                    ctx.program_pointer = ctx.start_stack.pop().unwrap() - 1;
                    ctx.next_inst();
                }
            }
        }
        Ok(())
    }
}

struct Context<'a> {
    program_pointer: usize,
    program: &'a Instructions,
    memory_pointer: i32,
    memory: BTreeMap<i32, u8>,
    start_stack: Vec<usize>,
    bracket_map: &'a HashMap<usize, usize>,
}

impl<'a> Context<'a> {
    pub fn new(program: &'a Instructions, bracket_map: &'a HashMap<usize, usize>) -> Self {
        Self {
            program_pointer: 0,
            program,
            memory_pointer: 0,
            memory: BTreeMap::new(),
            start_stack: Vec::new(),
            bracket_map,
        }
    }
    pub fn incr(&mut self) {
        let (new_val, _) = self
            .memory
            .entry(self.memory_pointer)
            .or_insert(0)
            .overflowing_add(1);
        self.memory.insert(self.memory_pointer, new_val);
    }
    pub fn decr(&mut self) {
        let (new_val, _) = self
            .memory
            .entry(self.memory_pointer)
            .or_insert(0)
            .overflowing_sub(1);
        self.memory.insert(self.memory_pointer, new_val);
    }
    pub fn get_byte(&self) -> u8 {
        match self.memory.get(&self.memory_pointer) {
            Some(byte) => *byte,
            None => 0,
        }
    }
    pub fn add_byte(&mut self, byte: u8) {
        self.memory.insert(self.memory_pointer, byte);
    }
    pub fn next_inst(&mut self) {
        self.program_pointer += 1;
    }
    pub fn has_next(&self) -> bool {
        // if this fails, the program exits
        self.program_pointer < self.program.len()
    }
    pub fn get_inst(&self) -> Instruction {
        self.program[self.program_pointer]
    }
}
