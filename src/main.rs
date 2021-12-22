/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 15:17:24
 * Last edited:
 *   22 Dec 2021, 17:21:07
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entry point to the OffsetCalculator tool.
**/

use std::env;
use parse_args;


/***** ENTRY POINT *****/
fn main() {
    // Prepare parsing the arguments
    let mut parser = parse_args::ArgParser::new();
    parser.add_double_dash();
    parser.add_help();

    // Add the test positional
    parser.add_pos("test", "test", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do\n\tb\na\tb\neiusmod tempor incididunt ut labore et dolore magna aliqua. ashdgfjhsdgfhajgsdfajhgsdfhjagsdfjhasgdfkhagsdfkahdsgfakhdsgfakhgdsfkjhsgdfkhgsdfkahgdfkahdgfakhdgfkahdfgakhgf Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
    // // Add the test option
    // parser.add_opt("test", 't', "test", 0, 1, "[<boolean>]", "A test option that optionally carries a boolean value.");

    // Parse the arguments
    let args: Vec<String> = env::args().collect();
    let args_dict = parser.parse(&args);
    if args_dict.has_opt("help") {
        print!("{}", parser.get_help(args[0].as_str(), 20, 80));
        return;
    } else {
        print!("No help found.\n");
    }

    // If there are any errors or warnings, show them
    if args_dict.has_errors() {
        args_dict.print_errors();
        std::process::exit(-1);
    }
    if args_dict.has_warnings() {
        args_dict.print_warnings();
    }

    // Show help

}
