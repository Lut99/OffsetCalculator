/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 15:17:24
 * Last edited:
 *   05 Jan 2022, 16:39:40
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entry point to the OffsetCalculator tool.
**/

mod ast;
mod traversals;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use ast::parser;
use traversals::print;


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
    parser.add_opt("convert", "c", "convert", 1, 1, "<value>", "If given, converts the given decimal value to hexadecimal or the given hexadecimal value to decimal. Then the program quits.");

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
    if args_dict.has_opt("convert") {
        // Get the value
        let value: &str = &args_dict.get_opt("convert").unwrap()[0];

        // Do later
        panic!("The '--convert' flag behaviour hasn't been implemented yet.");
    }

    

    /* SHOW HEADER */
    // Show a bit of a header to let the user know what they're up to
    println!("\n*** OFFSETCALCULATOR ***\n");





    /* REPL LOOP */
    // Prepare the linereader
    let mut rl = Editor::<()>::new();

    // Enter the REPL loop
    loop {
        let readline = rl.readline(" > ");
        match readline {
            Ok(line) => {
                // Success in reading line
                
                // Throw it thru the parser
                let ast = parser::parse(&line);
                match ast {
                    Some(node) => {
                         // Traverse the AST
                        print::traverse(node);
                    }
                    None => {
                        // Skip this line
                        continue;
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

    // Done
    println!("Bye.\n");
    return;
}
