mod generator;
mod lexer;
mod parser;

use std::fs;
use std::process::Command;

use generator::Generator;
use lexer::{Lexer, Token, TokenType};
use parser::Parser;

fn main() {
    let source = fs::read_to_string("hello.ct").unwrap();

    let mut lexer = Lexer::new(source);
    lexer.tokenization();

    let mut parser = Parser::new(lexer.tokens);

    let nodeprogram = parser.parse().unwrap();

    let mut generator = Generator::new(nodeprogram);

    generator.generate();

    // dbg!(lexer.tokens);

    // code_generation(lexer.tokens);

    Command::new("nasm")
        .args(["-f", "elf32", "output.asm", "-o", "output.o"])
        .output();

    Command::new("ld")
        .args(["-m", "elf_i386", "-s", "output.o", "-o", "output"])
        .output();

    Command::new("./output").spawn();
}
