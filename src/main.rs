/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 15:17:24
 * Last edited:
 *   06 Jan 2022, 17:55:33
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entry point to the OffsetCalculator tool.
**/

mod ast;
mod traversals;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use ast::parser::ValueKind;
use ast::parser::ASTNode;
use ast::symbol_table::SymbolTable;
use traversals::print_tree;
use traversals::trim;
use traversals::types;
use traversals::compute;
use traversals::symbol_table;


/***** CONSTANTS *****/
/// The default file to always load if it's a session
const DEFAULT_SESSION_PATH: &str = "./offsetcalculator.session";





/***** ENTRY POINT *****/
fn main() {
    /* PARSE ARGUMENTS */
    // Prepare parsing the arguments
    let mut parser = parse_args::ArgParser::new();
    parser.add_double_dash();
    parser.add_help();
    // Add the positionals
    // None
    // Add the options
    parser.add_opt("exec", "e", "execute", 1, 1, "<expression>", "If given, simply executes only this line and then quits (not entering the REPL).");
    parser.add_opt("session", "s", "session", 1, 1, "<path>", format!("If given, stores this session in the given file so you can resume later on. Note that, if present, the offsetcalculator always tries to load '{}'.", DEFAULT_SESSION_PATH));
    parser.add_opt("no_session", "S", "no-session", 0, 0, "", "If given, does not load the session file in the current directory.");

    // Parse the arguments
    let args_dict = parser.parse(&parse_args::get_args_from_env!());
    if args_dict.has_help() {
        return;
    }

    // If there are any errors or warnings, show them
    if args_dict.has_errors() {
        args_dict.print_errors();
        std::process::exit(-1);
    }
    if args_dict.has_warnings() {
        args_dict.print_warnings();
    }





    /* ONE-TIME COMMANDS */
    // If the user gave a flag that immediately returns, handle it
    if args_dict.has_opt("exec") {
        // Get the value
        let mut res: Option<ASTNode>;
        let value: &str = &args_dict.get_opt("exec").unwrap()[0];

        // Parse the value
        let oast = ast::parser::parse(&value);
        let mut ast: ast::parser::ASTNode;
        match oast {
            Some(node) => { ast = node; }
            None => { std::process::exit(-1); }
        }

        // Check if it's a command
        let cmd_res = ast::parser::get_command(&ast);
        if let Some(_) = cmd_res {
            eprintln!("You cannot execute commands from --execute, only expressions.");
            std::process::exit(-1);
        }

        // Prepare the symbol table
        let mut symtable = SymbolTable::new();
        symtable.insert(String::from("ans"), (ValueKind::Undefined, 0));

        // Trim it
        ast = trim::traverse(ast);
        // Resolve the symbol table
        res = symbol_table::traverse(ast, &mut symtable);
        if let Some(ast) = res {
            // Resolve the typing
            res = types::traverse(ast, &mut symtable);
            if let Some(ast) = res {
                // Compute the result!
                let mut value: u64 = 0;
                res = compute::traverse(ast, &mut value, &mut symtable);
                if let Some(ast) = res {
                    // Print the result in the correct format
                    let kind = ast::parser::get_kind(&ast);
                    match kind {
                        ValueKind::Decimal => { println!("{}", value); }
                        ValueKind::Hexadecimal => { println!("{:#x}", value); }
                        ValueKind::Binary => { println!("{:#b}", value); }
                        _ => {
                            panic!("Unknown ValueKind {:?} in AST's root node; this should never happen!", kind);
                        }
                    }
                }
            }
        }

        // Done
        return;
    }



    /* SHOW HEADER */
    // Show a bit of a header to let the user know what they're up to
    println!("\n*** OFFSETCALCULATOR ***\n");





    /* REPL LOOP */
    // Prepare the symbol table
    let mut symtable = SymbolTable::new();
    symtable.insert(String::from("ans"), (ValueKind::Undefined, 0));

    // Prepare the linereader
    let mut rl = Editor::<()>::new();

    // Load the history if needed
    if (!args_dict.has_opt("no_session") && std::path::Path::new(DEFAULT_SESSION_PATH).exists()) || args_dict.has_opt("session") {
        // Prepare the path
        let path: &str;
        if !args_dict.has_opt("no_session") && std::path::Path::new(DEFAULT_SESSION_PATH).exists() {
            path = DEFAULT_SESSION_PATH;
        }

        // First, load the last line of the file to get the ans
        let mut file = File::open(path);
        if !file.is_err() {
            let mut buffer = String::new();
            let res = file.unwrap().read_to_string(&mut buffer);
            if res.is_err() {
                // Split the buffer
                let ans_line = buffer.lines().rev().next();
                if ans_line.is_some() {
                    // Split it into type and int
                    // Now finally, try to parse the last one of those
                    let val = ans_line.unwrap().parse::<u64>();
                    if val.is_ok() {
                        // Store it
                        symtable.get_mut("ans").unwrap().1 = val.unwrap();
                    }
                    
                } else {
                    eprintln!("WARNING: Missing ans line in session file '{}'.", path);
                }
            } else {
                eprintln!("WARNING: Could not read session file '{}': {}.", path, res.err().unwrap());
            }
        } else {
            eprintln!("WARNING: Could not open session file '{}': {}.", path, file.err().unwrap());
        }

        // Load the file
        let res = rl.load_history(&path);
        if res.is_err() {
            eprintln!("WARNING: Could not load history from '{}': {}.", path, res.err().unwrap());
        }

        // Sneak our ans value from the history
        if rl.history().len() == 0 {
            eprintln!("WARNING: Loaded history file is empty.");
        } else {
            // Try to parse
            rl.history_mut().
        }
    }

    // Enter the REPL loop
    let mut res: Option<ASTNode>;
    loop {
        let readline = rl.readline(" > ");
        match readline {
            Ok(line) => {
                // Success in reading line; add it to the history, but only if it's different
                rl.add_history_entry(line.clone());

                // Throw it thru the parser
                let oast = ast::parser::parse(&line);
                let mut ast: ast::parser::ASTNode;
                match oast {
                    Some(node) => { ast = node; }
                    None => { continue; }
                }
                // println!("Parsed:");
                // ast = print_tree::traverse(ast);

                // Check if it's a command
                let cmd_res = ast::parser::get_command(&ast);
                if let Some(cmd) = cmd_res {
                    // Try to run it!
                    match cmd {
                        ASTNode::ShowVars { pos1: _, pos2: _ } => {
                            // Print the symbol table
                            println!("Currently defined variables:");
                            for (identifier, (kind, value)) in symtable.iter() {
                                println!(" - {}\t\t{:?}, {}", identifier, kind, value);
                            }
                            println!();
                            continue;
                        }
                        ASTNode::Help { pos1: _, pos2: _ } => {
                            // Print the help string
                            println!("You can simply write any calculation you like, which will then be evaluated.");
                            println!("There are a few special command keywords:");
                            println!(" - 'show_vars': Shows a list of currently loaded variables and their values.");
                            println!(" - 'help': Shows this menu.");
                            println!(" - 'exit': Exits the REPL.");
                            println!();
                            continue;
                        }
                        ASTNode::Exit { pos1: _, pos2: _ } => {
                            // Quit!
                            break;
                        }

                        // Not a command
                        _ => {
                            panic!("Got unknown command {:?}: this should never happen!", cmd);
                        }
                    }
                }

                // Trim it
                ast = trim::traverse(ast);
                // println!("Trimmed:");
                // ast = print_tree::traverse(ast);
                // Resolve the symbol table
                res = symbol_table::traverse(ast, &mut symtable);
                if let Some(ast) = res {
                    // Resolve the typing
                    // println!("Resolved:");
                    // let ast2 = print_tree::traverse(ast);
                    res = types::traverse(ast, &mut symtable);
                    // res = types::traverse(ast2, &mut symtable);
                    if let Some(ast) = res {
                        // Compute the result!
                        let mut value: u64 = 0;
                        // println!("Typed:");
                        // let ast2 = print_tree::traverse(ast);
                        res = compute::traverse(ast, &mut value, &mut symtable);
                        // res = compute::traverse(ast2, &mut value, &mut symtable);
                        if let Some(ast) = res {
                            // Print the result in the correct format
                            // println!("Computed:");
                            // let ast2 = print_tree::traverse(ast);
                            let kind = ast::parser::get_kind(&ast);
                            // let kind = ast::parser::get_kind(&ast2);
                            match kind {
                                ValueKind::Decimal => { println!(" = {}", value); }
                                ValueKind::Hexadecimal => { println!(" = {:#x}", value); }
                                ValueKind::Binary => { println!(" = {:#b}", value); }
                                _ => {
                                    panic!("Unknown ValueKind {:?} in AST's root node; this should never happen!", kind);
                                }
                            }

                            // Store the ans in the symbol table
                            symtable.get_mut("ans").unwrap().0 = kind;
                            symtable.get_mut("ans").unwrap().1 = value;

                            // Print an extra newline to close off
                            println!();
                        }
                    }
                }

            },
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C was pressed
                std::process::exit(signal_hook::consts::SIGINT);
            },
            Err(ReadlineError::Eof) => {
                // Ctrl+D was pressed
                break;
            },
            Err(err) => {
                // Other error
                eprintln!("Couldn't read line: {:?}", err);
                std::process::exit(-1);
            }
        }
    }

    // Save the history
    let res = rl.save_history(&histpath);
    if res.is_err() {
        let err = res.err().unwrap();
        println!("WARNING: Could not save history to '{}': {}.", histpath, err);
    }

    // Done
    println!("Bye.\n");
    return;
}
