pub mod parser;
pub mod syntax;
pub mod tokenizer;

use crate::parser::Parser;
use crate::syntax::{SyntaxElement, SyntaxKind};
use crate::tokenizer::Tokenizer;
use rowan::NodeOrToken;

fn print(indent: usize, element: &SyntaxElement) {
    let kind: SyntaxKind = element.kind().into();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, &child);
            }
        }

        NodeOrToken::Token(token) => {
            println!("- {:?} {:?}", token.text(), kind)
        }
    }
}

fn main() {
    let tokenizer = Tokenizer::new("1 + 2 * 3 + 4");
    let parser = Parser::new(tokenizer.peekable());
    let ast = parser.parse();

    print(0, &ast.into());
}
