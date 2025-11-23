use core::panic;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;

/*
*  BRAINFUCK LOGIC:
*
*  30,000 cell array
*
*  <, > -> moves to left/right cell
*
*  +/- -> increments/decrements the value in that cell
*
*  [ -> if current cell is zero skip to matcing ]
*
*  ] -> if current cell is NOT zero jump back to starting [
*
*  . -> writes current cell to stdout
*
*  , -> reads user input and stores it to current cell
*
*  and that's it!
*/

fn generate_base(path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    let base = "\
.intel_syntax noprefix

.bss
arr:
        .space 30000
input:
        .space 1

.section .text
        .global _start

_start:
        lea r12, [arr]
        add r12, 14999
        mov rdx, 1
        mov byte ptr [r12], 0
";

    file.write(base.as_bytes())?;
    Ok(())
}

fn compile_brainfuck(bf_file: &String, path: &String) -> Result<(), std::io::Error> {
    let mut file = File::open(bf_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut output = OpenOptions::new().append(true).open(path).unwrap();
    let mut loop_counter: usize;
    let mut stack_top: usize;
    let mut loop_stack: Vec<usize> = vec![];
    let mut closed_loops: Vec<usize> = vec![];

    for char in contents.chars() {
        match char {
            '>' => writeln!(output, "        add r12, 1")?,
            '<' => writeln!(output, "        sub r12, 1")?,
            '+' => writeln!(output, "        add byte ptr [r12], 1")?,
            '-' => writeln!(output, "        sub byte ptr [r12], 1")?,
            '[' => {
                loop_counter = 0;
                loop {
                    if !loop_stack.contains(&loop_counter) && !closed_loops.contains(&loop_counter)
                    {
                        break;
                    }
                    loop_counter += 1;
                }
                loop_stack.push(loop_counter);
                writeln!(
                    output,
                    "l{}:\n        movzx ecx, byte ptr [r12]\n        cmp ecx, 0\n        je le{}",
                    loop_counter, loop_counter
                )?
            }
            ']' => {
                let stack_length = loop_stack.len();
                if stack_length > 0 {
                    stack_top = loop_stack[stack_length - 1];
                    closed_loops.push(stack_top);
                    loop_stack.pop();
                } else {
                    panic!("Syntax error! You cannot close a loop which has not been opened.");
                }
                writeln!(
                    output,
                    "le{}:\n        movzx ecx, byte ptr [r12]\n        cmp ecx, 0\n        jne l{}",
                    stack_top, stack_top
                )?
            }
            '.' => writeln!(
                output,
                "        mov rax, 1\n        mov rdi, 1\n        lea rsi, [r12]\n        syscall"
            )?,
            ',' => writeln!(
                output,
                "        mov rax, 0\n        mov rdi, 0\n        lea rsi, [input]\n        syscall\n        mov cl, [input]\n        mov [r12], cl"
            )?,
            _ => continue,
        };
    }

    if loop_stack.len() > 0 {
        panic!("Syntax error! You have declared a loop which is never closed.")
    }
    writeln!(
        output,
        "        mov rax, 60\n        mov rdi, 0\n        syscall"
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let brainfuck_file = args.iter().nth(1);
    let output_file = args.iter().last();

    if args.len() < 3 {
        panic!("\nUSAGE: bfc [PATH TO BRAINFUCK FILE].bf [PATH TO OUTPUT FILE].s\n")
    }

    let now = Instant::now();

    match output_file {
        Some(op_file) => {
            println!("{}", op_file);
            match generate_base(op_file) {
                Ok(_) => {}
                Err(e) => {
                    panic!(
                        "Unexpected error occured: {}\n\nUSAGE: bfc [PATH TO BRAINFUCK FILE].bf [PATH TO OUTPUT FILE].s\n",
                        e
                    );
                }
            }
            match brainfuck_file {
                Some(bf_file) => match compile_brainfuck(bf_file, op_file) {
                    Ok(_) => {
                        println!(
                            "Program compiled in {} miliseconds.",
                            now.elapsed().as_millis()
                        );
                    }
                    Err(e) => {
                        panic!(
                            "Unexpected error occured: {}\n\nUSAGE: bfc [PATH TO BRAINFUCK FILE].bf [PATH TO OUTPUT FILE].s\n",
                            e
                        );
                    }
                },
                None => {
                    panic!("Brainfuck file not specified.");
                }
            };
        }
        None => {
            panic!("Output file not specified.");
        }
    }
}
