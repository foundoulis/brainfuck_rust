
pub mod ast;
pub mod program;

use crate::ast::AST;
use crate::program::Program;

fn main() {
    let mut instruct = ast::AST::new_from_string(&mut "+[[->]-[-<]>-]>.>>>>.<<<<-.>>-.>.<<.>>>>-.<<<<<++.>>++.".chars());
    run_ast(&mut instruct, &mut Program::new(Vec::new()));
}

fn run_ast(ast: &mut Vec<AST>, prog: &mut Program) {
    for inst in ast {
        match inst {
            AST::MOVE_LEFT => prog.mv_l(),
            AST::MOVE_RIGHT => prog.mv_r(),
            AST::INCR => prog.incr(),
            AST::DECR => prog.decr(),
            AST::OUTPUT => prog.print(),
            AST::INPUT => prog.set(),
            AST::LOOP(ref mut loop_to_execute) => run_ast(&mut **loop_to_execute, prog),
        };
    }
}
