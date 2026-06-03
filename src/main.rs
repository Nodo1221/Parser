mod lexer;
mod parser;

use lexer::lex;
use parser::parse_expr;

fn main() {
    let in1 = "(2 + -3) * 4";
    let in2 = "2 * 3 * -4";
    let in3 = "2 + 3 * 4 + 1";

    let lex1 = lex(in1);
    let lex2 = lex(in2);
    let lex3 = lex(in3);

    let tree1 = parse_expr(&lex1);
    let tree2 = parse_expr(&lex2);
    let tree3 = parse_expr(&lex3);

    println!("1: {in1:<15} {:?}", lex1);
    println!("2: {in2:<15} {:?}", lex2);
    println!("3: {in3:<15} {:?}", lex3);
    println!();
    println!("1: {:?}", tree1);
    println!("2: {:?}", tree2);
    println!("3: {:?}", tree3);
}