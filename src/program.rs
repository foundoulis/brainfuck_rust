use std::char;

type CellSize = u32;

pub struct Program {
    mem: Vec<CellSize>,
    loc: usize,
    in_bytes: Vec<CellSize>,
}

impl Program {
    pub fn new(in_bytes: Vec<CellSize>) -> Program {
        let mut new_mem = Vec::with_capacity(30000);
        for _ in 0..30000 {
            new_mem.push(0);
        }
        Program {
            mem: new_mem,
            loc: 15000usize,
            in_bytes: in_bytes,
        }
    }

    pub fn incr(&mut self) {
        self.mem[self.loc] += 1;
    }
    pub fn decr(&mut self) {
        self.mem[self.loc] -= 1;
    }

    pub fn mv_l(&mut self) {
        self.loc += 1;
    }
    pub fn mv_r(&mut self) {
        self.loc -= 1;
    }

    pub fn print(&self) {
        println!(
            "{}",
            char::from_u32(self.mem[self.loc] as u32).unwrap_or('!')
        );
    }
    pub fn set(&mut self) {
        self.mem[self.loc] = self.in_bytes.pop().unwrap();
    }
}
