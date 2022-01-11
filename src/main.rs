/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 15:17:24
 * Last edited:
 *   11 Jan 2022, 14:12:56
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entry point to the OffsetCalculator tool.
**/

mod ast;
mod traversals;
mod session;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use ast::parser::ValueKind;
use ast::parser::ASTNode;
use ast::symbol_table::SymbolTable;
#[allow(unused_imports)]
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
    parser.add_opt("session", "s", "session", 1, 1, "<path>", &format!("If given, stores this session in the given file so you can resume later on. Note that, if present, the offsetcalculator always tries to load '{}'.", DEFAULT_SESSION_PATH));
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
                        ValueKind::Hexadecimal => { println!("0x{:X}", value); }
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

    // Load the session if needed
    if (!args_dict.has_opt("no_session") && std::path::Path::new(DEFAULT_SESSION_PATH).exists()) || args_dict.has_opt("session") {
        // Resolve the path
        let mut path = DEFAULT_SESSION_PATH;
        if args_dict.has_opt("session") {
            path = &args_dict.get_opt("session").unwrap()[0];
        }

        // Try to load the session
        if let Err(reason) = session::load(path, &mut symtable, &mut rl) {
            eprintln!("{}: WARNING: {}: Not loading session file.", reason.path(), reason);
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
                        ASTNode::Del { ref identifier, pos1: _, pos2: _ } => {
                            // Try to find the identifier
                            if symtable.contains_key(identifier) {
                                // Remove it
                                symtable.remove(identifier);
                                println!("   Deleted variable '{}'.\n", identifier);
                            } else {
                                eprintln!("   Unknown identifier '{}'; cannot delete it.", identifier);
                            }
                            continue;
                        }
                        ASTNode::DelAll { pos1: _, pos2: _ } => {
                            // Clear the symbol table
                            symtable.clear();
                            // Reinstate ans
                            symtable.insert(String::from("ans"), (ValueKind::Undefined, 0));
                            println!("   Cleared all variables.\n");
                            continue;
                        }
                        ASTNode::ShowVars { pos1: _, pos2: _ } => {
                            // Print the symbol table
                            println!("   Currently defined variables:");
                            for (identifier, (kind, value)) in symtable.iter() {
                                println!(" - {}\t\t{:?}, {}", identifier, kind, value);
                            }
                            println!();
                            continue;
                        }
                        ASTNode::ClearHist { pos1: _, pos2: _ } => {
                            // Clear the history
                            rl.clear_history();
                            println!("   Cleared history.\n");
                            continue;
                        }
                        ASTNode::Help { pos1: _, pos2: _ } => {
                            // Print the help string
                            println!("   See help below for either writing expressions or running commands.");
                            println!();
                            println!("   Expressions:");
                            println!("     Expressions in the calculator are written as normal programming language math");
                            println!("     expressions.");
                            println!("     You can use the following constants:");
                            println!("      - A decimal constant (e.g., '42') or a constant prefixed by '0d' (e.g.,");
                            println!("        '0d42')");
                            println!("      - A hexadecimal constant prefixed by '0x' (e.g., '0x2A')");
                            println!("      - A binary constant prefixed by '0b' (e.g., '0b101010')");
                            println!("     Furthermore, you can also use the following operators (in order of");
                            println!("     precedence):");
                            println!("      - <id> = <expr>: Creates a variable with the given ID and sets its value to");
                            println!("        the given expression.");
                            println!("      - dec <expr>: Converts the representation of the given expression to");
                            println!("        decimal.");
                            println!("      - hex <expr>: Converts the representation of the given expression to");
                            println!("        hexadecimal.");
                            println!("      - bin <expr>: Converts the representation of the given expression to");
                            println!("        binary.");
                            println!("      - <expr> * <expr>: Multiplication on the given two expressions.");
                            println!("      - <expr> / <expr>: Division on the given two expressions.");
                            println!("      - <expr> + <expr>: Addition on the given two expressions.");
                            println!("      - <expr> - <expr>: Subtraction on the given two expressions.");
                            println!();
                            println!("   Commands:");
                            println!("     There are a few special command keywords:");
                            println!("      - 'del <id>': Deletes the variable with the given identifier.");
                            println!("      - 'delall': Deletes all variables, even 'ans' (resetting it to undefined).");
                            println!("      - 'show_vars': Shows a list of currently loaded variables and their values.");
                            println!("      - 'clear_hist': Clear the history of the REPL up to that point.");
                            println!("      - 'help': Shows an in-calculator help menu for expressions and commands.");
                            println!("      - 'exit': Exits the REPL.");
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
                                ValueKind::Hexadecimal => { println!(" = 0x{:X}", value); }
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
                eprintln!("   Couldn't read line: {:?}", err);
                std::process::exit(-1);
            }
        }
    }

    // Save the session, if needed
    if (!args_dict.has_opt("no_session") && std::path::Path::new(DEFAULT_SESSION_PATH).exists()) || args_dict.has_opt("session") {
        // Resolve the path
        let mut path = DEFAULT_SESSION_PATH;
        if args_dict.has_opt("session") {
            path = &args_dict.get_opt("session").unwrap()[0];
        }

        // Save the session!
        if let Err(reason) = session::save(path, &symtable, &rl) {
            eprintln!("{}: WARNING: {}: Not saving session file.", reason.path(), reason);
        }
    }

    // Done
    println!("Bye.\n");
    return;
}
