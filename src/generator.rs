use crate::parser::NodeExpr;
use crate::parser::{NodeProgram, NodeStmt};
use std::fs;

pub struct Generator {
    program: NodeProgram,
    output: String,
}

impl Generator {
    pub fn new(program: NodeProgram) -> Self {
        Self {
            program: program,
            output: String::new(),
        }
    }

    pub fn generate(&mut self) {
        self.output.push_str("section .data\n");
        for stmt in &self.program.statements {
            match stmt {
                NodeStmt::NodePrint { expr } => {
                    match expr {
                        NodeExpr::NodeExprStringLiteral { token } => {
                            self.output.push_str(
                                format!("    string db '{}', 0xa\n", token.value.clone().unwrap())
                                    .as_str(),
                            );
                        }
                        NodeExpr::NodeExprIntLiteral { token } => {
                            self.output.push_str(
                                format!("    string db '{}', 0xa\n", token.value.clone().unwrap())
                                    .as_str(),
                            );
                        }
                    }
                    self.output.push_str("    stringLen equ $ - string\n");
                    self.output.push_str("section .text\n");
                    self.output.push_str("    global _start\n");
                    self.output.push_str("_start:\n");
                    self.output.push_str("    mov eax, 4\n");
                    self.output.push_str("    mov ebx, 1\n");
                    self.output.push_str("    mov ecx, string\n");
                    self.output.push_str("    mov edx, stringLen\n");
                    self.output.push_str("    int 0x80\n");
                    self.output.push_str("    mov eax, 1\n");
                    self.output.push_str("    int 0x80\n");
                }
            }
        }
        fs::write("output.asm", self.output.as_str()).unwrap();
    }
}
