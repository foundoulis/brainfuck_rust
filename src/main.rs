
pub mod bfops;

fn main() {
    println!(
        "{:?}",
        bfops::AST::new_from_string(&mut "+++++[<<>>[<<]>>],.,.,[---]--".chars())
    );
}

fn run_ast(ast: &bfops::AST, prog: &mut Vec<i8>, loc: usize) {
    let array_size = 1024;
    let mut program = [i8; array_size];
    let mut location: usize = (array_size as usize) / 2;

    match ast {
        AST::MOVE_LEFT => location += 1;
        AST::MOVE_RIGHT => location -= 1;
        AST::INCR => program[location] += 1;
        AST::DECR => program[loaction] -= 1;
        AST::OUTPUT => println!("{}", program[location]);
        AST::INPUT => program[location] = 100;
        AST::LOOP(ref loop_to_execute) => run_ast(loop_to_execute, prog, loc);
    };
}
