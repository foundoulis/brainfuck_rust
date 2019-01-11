
pub mod bfops;

fn main() {
    println!(
        "{:?}",
        bfops::AST::new_from_string(&mut "+++++[<<>>[<<]>>],.,.,[---]--".chars())
    );
}
