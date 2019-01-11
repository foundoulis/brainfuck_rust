#[derive(Debug)]
pub enum AST {
    MOVE_LEFT,
    MOVE_RIGHT,

    INCR,
    DECR,

    INPUT,
    OUTPUT,

    LOOP(Box<Vec<AST>>),
}

impl AST {
    pub fn new_from_string(chars: &mut std::str::Chars) -> Vec<AST> {
        let mut ast: Vec<AST> = Vec::new();
        while let Some(ch) = chars.next() {
            match ch {
                '<' => ast.push(AST::MOVE_LEFT),
                '>' => ast.push(AST::MOVE_RIGHT),
                '+' => ast.push(AST::INCR),
                '-' => ast.push(AST::DECR),
                ',' => ast.push(AST::INPUT),
                '.' => ast.push(AST::OUTPUT),
                '[' => {
                    ast.push(AST::LOOP(Box::new(AST::new_from_string(chars))));
                }
                ']' => {
                    break;
                }
                _ => (),
            };
        }

        ast
    }
}
