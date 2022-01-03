/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 15:17:24
 * Last edited:
 *   03 Jan 2022, 11:06:12
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entry point to the OffsetCalculator tool.
**/

mod ast;

use ast::symbols;


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

        // Try to convert to hexadecimal or back
        let t: symbols::Token = symbols::Token::new(symbols::TerminalKind::Undefined);
    }

    

    /* SHOW HEADER */
    
}
