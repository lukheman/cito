mod lexer;
use std::fs;
use std::process::Command;

use lexer::{Lexer, Token, TokenType};

fn code_generation(tokens: Vec<Token>) {
    let mut output = String::new();
    output.push_str("section .data\n");

    if tokens[0].tokentype == TokenType::Print
        && tokens[1].tokentype == TokenType::Int
        && tokens[2].tokentype == TokenType::Semicolon
    {
        output.push_str(
            format!(
                "    string db '{}', 0xa\n",
                tokens[1].value.clone().unwrap()
            )
            .as_str(),
        );
        output.push_str("    stringLen equ $ - string\n");

        output.push_str("section .text\n");
        output.push_str("    global _start\n");
        output.push_str("_start:\n");
        output.push_str("    mov eax, 4\n");
        output.push_str("    mov ebx, 1\n");
        output.push_str("    mov ecx, string\n");
        output.push_str("    mov edx, stringLen\n");
        output.push_str("    int 0x80\n");

        output.push_str("    mov eax, 1\n");
        output.push_str("    int 0x80\n");
    } else {
        panic!("invalid syntax");
    }

    fs::write("output.asm", output).unwrap();
}

fn main() {
    let source = fs::read_to_string("hello.ct").unwrap();

    let mut lexer = Lexer::new(source);
    lexer.tokenization();

    code_generation(lexer.tokens);

    Command::new("nasm")
        .args(["-f", "elf32", "output.asm", "-o", "output.o"])
        .output();

    Command::new("ld")
        .args(["-m", "elf_i386", "-s", "output.o", "-o", "output"])
        .output();

    Command::new("./output").spawn();
}
