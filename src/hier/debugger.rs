use std::io;
use std::io::Write;
use crate::exit_handler;
use crate::hier::environment::Environment;

pub(crate) fn debug(environment: &mut Environment, break_function: &String) {
    if break_function != "" && !environment.is_a_step {
        println!("Breakpoint {} at {}:{} in {}",
                 break_function,
                 environment.current_interpreting_location.line_number,
                 environment.current_interpreting_location.offset,
                 environment.current_interpreting_location.module
        );
    }

    if environment.is_a_step {
        environment.is_a_step = false;
        environment.should_step_into = false;
    }

    loop {
        print!("HDB > ");
        std::io::stdout().flush().expect("Failed to flush stdout.");

        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            eprintln!("Failed to read line: {}.", error);
            exit_handler();
        };

        line = line.trim().to_string();

        let mut parts: Vec<&str> = line.split(' ').collect();
        let command = parts[0];

        parts.remove(0);
        let argument = parts.join(" ");

        match command {
            "h" | "help" => print_help(),
            "b" | "break" => {
                environment.breakpoints.push(argument);
            },
            "rb" | "rebreak" => {
               let br_id = argument.parse::<usize>();

                match br_id {
                    Ok(id) => {
                        if environment.breakpoints.len() <= id {
                            println!("Error: Breakpoint with id {} doesn't exist.", id);
                        } else {
                            environment.breakpoints.remove(id);
                        }
                    },
                    Err(_) => {
                        let mut idx: i32 = -1;
                        let mut i = 0;

                        for br in environment.breakpoints.clone() {
                            if br == argument {
                                idx = i;
                                break;
                            }

                            i += 1;
                        }

                        if idx == -1 {
                            println!("Error: Breakpoint {} doesn't exist.", argument);
                        } else {
                            environment.breakpoints.remove(idx as usize);
                        }
                    }
                }
            },
            "lib" | "libreak" => {
                let mut i = 0;
                for br in environment.breakpoints.clone() {
                    println!("{} - {}", i, br);
                    i += 1;
                }
            },
            "c" | "continue" => {
                return;
            },
            "s" | "step" => {
                environment.is_a_step = true;
                return;
            },
            "l" | "location" => {
                println!("{} at {}:{} in {}",
                         break_function,
                         environment.current_interpreting_location.line_number,
                         environment.current_interpreting_location.offset,
                         environment.current_interpreting_location.module
                );
            },
            "e" | "expression" => {
                // FIX: Sometimes prints wrong expression
                println!("{}", environment.current_interpreting_expression.get_representation());
            },
            "p" | "print" => {
                println!("{:?}", environment.get(argument));
            },
            "x" | "exit" => {
                (environment.exit_handler)();
            },
            _ =>  println!("Unknown command: {}", line)
        }
    }
}

fn print_help() {
    println!("== HDB help ==");
    println!("Notation:");
    println!("Each command has a long and short form which can be used interchangeably. They will be described like this: short/long <argument>. Some of them accept an argument. The argument is everything that is after the command name.");
    println!("Commands:");
    println!("h/help - print this information.");
    println!("b/break <function identifier> - start debugger before the function is ran.");
    println!("rb/rbreak <function identifier>/<breakpoint id> - remove breakpoint by function identifier or index, if it exists.");
    println!("lib/libreak - print breakpoints");
    println!("c/continue - continue execution until the end of the program or next breakpoint.");
    println!("s/step - continue to next function. Doesn't enter imported functions.");
    println!("e/expression - print current expression (inaccurate).");
    println!("p/print <variable identifier> - print value of the variable.");
    println!("l/location - print current location.");
    println!("x/exit - stops running the program and exits.");
}