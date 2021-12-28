/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 16:21:49
 * Last edited:
 *   28 Dec 2021, 12:59:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The parse-args library, which contains code to parse command line
 *   arguments.
**/

use std::collections::HashMap;
use opstring::OpString;


/***** CUSTOM TYPES *****/
/// Defines a shortcut for the Positional's HashMap in the ArgsDict.
type PositionalHashMap = HashMap<String, (usize, String)>;
/// Defines a shortcut for the Option's HashMap in the ArgsDict.
type OptionHashMap = HashMap<String, (String, String, Vec<String>)>;





/***** CONSTANTS *****/
/// The uid used for the help argument.
pub const HELP_UID: &str = "help";
/// The shortname used for the help argument.
pub const HELP_SHORTNAME: &str = "h";
/// The longname used for the help argument.
pub const HELP_LONGNAME: &str = "help";
/// The description used for the help argument.
pub const HELP_DESCRIPTION: &str = "Shows this list of arguments, then quits.";





/// Helper iterator over a string, that returns word-by-word instead of char-by-char.
/// 
/// Uses the graphene method to have intuitive characters.
/// 
/// **Template parameters**
///  * `'a`: The lifetime parameter for the WorldIterator, which should be itself.
struct WordIterator<'a> {
    /// The string we iterate over
    s    : OpString<'a>,
    /// The current position in the string
    i    : usize,
}

impl<'a> WordIterator<'a> {
    /// Constructor for the WordIterator class
    fn new(s: &'a str) -> WordIterator {
        // Return the new WordIterator
        return WordIterator {
            s    : OpString::new(s),
            i    : 0
        };
    }
}

impl<'a> Iterator for WordIterator<'a> {
    /// The type of each item returned by the iterator
    type Item = (&'a str, &'a str);

    /// Gets the next word/separator pair in the internal string.  
    /// A separator is what splits words, and can either be any whitespace (space, newline (`\n`), carriage return (`\r`) or tab (`\t`)) or a null-character (`\0`) in case of end-of-string.
    /// 
    /// **Returns**  
    /// An Option with, if we didn't reach the end yet, a tuple bearing the word (possibly empty in case of two consecutive separators) and the separator following after it.
    fn next(&mut self) -> std::option::Option<Self::Item> {
        // Continue with iterating where we were
        let start_i = self.i;
        loop {
            // Get the next char
            let c: &str;
            if self.i < self.s.len() { c = self.s[self.i]; }
            else { c = "\0"; }

            // See if it's a separator
            if c.eq(" ") || c.eq("\n") || c.eq("\t") || c.eq("\r") || c.eq("\0") {
                // It is; return the result + the separator
                let start_j = self.s.translate_opstr(start_i);
                let end_j   = self.s.translate_opstr(self.i);
                self.i += c.len();
                return Some((&self.s.parent()[start_j..end_j], c));
            }

            // Otherwise, move the internal i
            self.i += c.len();
        }
    }
}





/***** HELPER STRUCTS *****/
/// Intermediate representation for a Positional.
struct Positional {
    /// The uid for this positional.
    uid         : String,
    /// The index of this positional.
    index       : usize,
    /// The human-readable name for this positional. Used in the usage/help string.
    name        : String,
    /// The description for this positional.
    description : String,
}

/// Intermediate representation for an Option.
struct Option {
    /// The uid for this option.
    uid               : String,
    /// The shortname for this option. Will be the empty char (`\0`) if unused.
    shortname         : String,
    /// The longname for this option.
    longname          : String,
    /// The minimum number of values for this option.
    min_n_values      : usize,
    /// The maximum number of values for this option.
    max_n_values      : usize,
    /// The description of the parameters for this option.
    param_description : String,
    /// The description for this option.
    description       : String,
}





/***** ARGPARSER CLASS *****/
/// Defines a single instance for arguments.
pub struct ArgParser {
    /// Stores the defined positionals in the parser.
    positionals     : Vec<Positional>,
    /// Stores the defined options in the parser.
    options         : Vec<Option>,

    /// Determines whether or not the double-dash argument is used
    use_double_dash : bool,
    /// Determines whether or not the help is given
    use_help        : bool,
}

/// Defines the ArgParser's methods
impl ArgParser {
    /// Constructor for the ArgParser, which is public.
    pub fn new() -> ArgParser {
        ArgParser {
            positionals     : Vec::new(),
            options         : Vec::new(),
            use_double_dash : false,
            use_help        : false
        }
    }

    

    /// Helper function that parses at most max_n values from the given list of arguments.
    /// 
    /// **Arguments**
    ///  * `args`: The list of arguments to parse from.
    ///  * `i`: Reference to the current position within args. Will be increment as we parse, and is left at the last-parsed argument.
    ///  * `max_n`: The maximum number of arguments to parse.
    ///  * `parse_opts`: Whether or not options are still allowed to be parsed. Might be adapted if we have use_double_dash set and we encounter it.
    ///  * `use_double_dash`: Whether or not the function should look out for the double dash, option-disabling arg.
    /// **Returns**  
    /// The popped arguments, of which there will be at most max_n.
    fn parse_values(args: &Vec<String>, i: &mut usize, max_n: usize, parse_opts: &mut bool, use_double_dash: bool) -> Vec<String> {
        // Increment i to skip the option itself
        *i += 1;
        let start_i = *i;

        // Try to pop
        let mut result: Vec<String> = Vec::new();
        while *i < args.len() && *i - start_i < max_n {
            // Get the argument
            let arg = &args[*i];
            let sarg = OpString::new(arg);
            if sarg.len() == 0 { continue; }

            // If it's an option, stop
            if *parse_opts && sarg[0].eq("-") {
                // Make sure its not the other one
                if use_double_dash && sarg.len() == 2 && sarg[1].eq("-") {
                    *parse_opts = false;
                    *i += 1;
                    continue;
                }
                break;
            }

            // Otherwise, add to the result
            result.push(arg.clone());

            // Increment i
            *i += 1;
        }

        // i is now at the first unparseable thing; fix this for the main increment
        *i -= 1;

        // Return the result struct
        return result;
    }

    /// Generates a string of n spaces.
    /// 
    /// **Arguments**
    ///  * `N`: The number of spaces to generate.
    /// 
    /// **Returns**  
    /// A string with N spaces.
    fn generate_spaces(n: usize) -> String {
        // Create a large enough string
        let mut result: String = String::new();
        result.reserve(n);

        // Write the spaces
        for _ in 0..n {
            result.push(' ');
        }

        // Done
        return result;
    }

    /// Helper function that adds the given description linewrapped to the given string.
    /// 
    /// **Arguments**
    ///  * `result`: The string to append the result to.
    ///  * `x`: The current column position on the line. Will be updated as we write.
    ///  * `description`: The description to write.
    ///  * `indent_width`: The width before each new line.
    ///  * `line_width`: The line width to break on.
    fn print_description(result: &mut String, x: &mut usize, description: &str, indent_width: usize, line_width: usize) {
        // Make sure indent_width and line_width aren't conflicting
        if indent_width >= line_width {
            panic!("Cannot have an indent width larger than or equal to a line width: {} >= {}", indent_width, line_width);
        }

        // Generate the indent spaces
        let indent = ArgParser::generate_spaces(indent_width);

        // Go through the description word-by-word
        for (word, separator) in WordIterator::new(description) {
            // Only do stuff if the parsed word has at least one char
            if word.len() > 0 {
                // See if we need to go to the next line
                if *x != indent_width && *x + word.len() + 1 >= line_width {
                    // Add a new line plus the indent
                    result.reserve(1 + indent_width);
                    result.push('\n');
                    result.push_str(indent.as_str());

                    // Reset the x
                    *x = indent_width;
                }

                // Now loop through the word to write it, possibly linewrapped
                result.reserve(word.len() + word.len() / (line_width - indent_width));
                for c in unicode_segmentation::UnicodeSegmentation::graphemes(word, true) {
                    // Split if needed
                    if *x >= line_width {
                        // Add a new line plus the indent
                        result.reserve(1 + indent_width);
                        result.push('\n');
                        result.push_str(indent.as_str());

                        // Reset the x
                        *x = indent_width;
                    }

                    // Write the letter
                    result.push_str(c);
                    *x += 1;
                }
            }

            // After this word, print the needed stuff
            if separator.eq(" ") && *x + 1 + 1 < line_width {
                // Simply print the space
                result.push(' ');
                *x += 1;
            } else if separator.eq("\n") {
                // Print a newline
                result.reserve(1 + indent_width);
                result.push('\n');
                result.push_str(indent.as_str());

                // Reset the x
                *x = indent_width;
            } else if separator.eq("\r") {
                // Ignore
                continue;
            } else if separator.eq("\t") {
                // Print enough spaces to get to the next boundry of four
                let target_x = *x - (*x % 4) + 4;
                // Don't do it if we go too large
                if target_x + 1 >= line_width { continue; }
                // Print the spaces
                result.push_str(ArgParser::generate_spaces(target_x - *x).as_str());
                *x = target_x;
            } else if separator.eq("\0") {
                // Stop
                break;
            }
        }
    }



    /// Registers a new positional argument.
    /// 
    /// **Arguments**
    ///  * `uid`: Unique identifier for this argument. Doesn't share the names with options, so go nuts.
    ///  * `name`: Readable name for use in the usage/help string.
    ///  * `description`: A string description of the positional.
    pub fn add_pos(&mut self, uid: &str, name: &str, description: &str) {
        // Check if the uid conflicts
        for pos in self.positionals.iter() {
            if pos.uid == uid {
                panic!("A positional with uid '{}' already exists in this ArgParser instance.", uid);
            }
        }

        // Create a new Positional argument
        let result = Positional {
            uid: String::from(uid),
            index: self.positionals.len(),
            name: String::from(name),
            description: String::from(description)
        };

        // Store the positional internally
        self.positionals.push(result);
    }

    /// Registers a new option.
    /// 
    /// ** Arguments **
    ///  * `uid`: Unique identifier for this argument. Doesn't share the names with positionals, so go nuts.
    ///  * `shortname`: A single character, optional identifier for the option. Must be unique across all options. If you don't want to use it, pass a new/empty string.
    ///  * `longname`: A multi-character identifier for the option. Must be unique across all options.
    ///  * `min_n_values`: The minimum number of values for this option. If it's a flag, pass no argument (0).
    ///  * `max_n_values`: The maximum number of values for this option. If it's a flag, pass no argument (0). Cannot be smaller than `min_n_values`.
    ///  * `param_description`: A string description of the parameters of this option. Will most likely be a list of types or something.
    ///  * `description`: A string description of the option.
    pub fn add_opt(&mut self, uid: &str, shortname: &str, longname: &str, min_n_values: usize, max_n_values: usize, param_description: &str, description: &str) {
        // Check if the shortname is valid
        if unicode_segmentation::UnicodeSegmentation::graphemes(shortname, true).collect::<Vec<&str>>().len() > 1 {
            panic!("A shortlabel cannot have more than one character: {} > 1.", shortname.len());
        }

        // Check if the uid, shortname or longnames are in conflict
        for opt in self.options.iter() {
            if opt.uid.eq(uid) {
                panic!("An option with uid '{}' already exists in this ArgParser instance.", uid);
            }
            if shortname.len() > 0 && opt.shortname.eq(shortname) {
                panic!("An option with shortlabel '{}' already exists in this ArgParser instance.", shortname);
            }
            if opt.longname.eq(longname) {
                panic!("An option with longname '{}' already exists in this ArgParser instance.", longname);
            }
        }

        // Make sure the max_n_values isn't smaller
        if max_n_values < min_n_values {
            panic!("max_n_values has to be equal to or larger than min_n_values; {} > {}", max_n_values, min_n_values);
        }

        // Create a new Option
        let result = Option {
            uid               : String::from(uid),
            shortname         : String::from(shortname),
            longname          : String::from(longname),
            min_n_values,
            max_n_values,
            param_description : String::from(param_description),
            description       : String::from(description)
        };

        // Store the option intenally
        self.options.push(result);
    }

    /// Registers the double-dash that can be used to disable options
    pub fn add_double_dash(&mut self) {
        // Simply set that we use it
        self.use_double_dash = true;
    }

    /// Registers a help-flag as '-h' and '--help'.
    /// 
    /// To check if it was specified, call 'dict.has_opt(parse_args::HELP_UID)' on the resulting dict after the parse() call.
    /// 
    /// If run, reserves the '-h' and '--help' flags for standard help usage. Doing it this way automatically enables parsing help before anything else is parsed.
    pub fn add_help(&mut self) {
        // Check if the uid, shortname or longnames are in conflict
        for opt in self.options.iter() {
            if opt.uid.eq(HELP_UID) {
                panic!("Cannot add help, as an option with uid '{}' already exists in this ArgParser instance.", HELP_UID);
            }
            if HELP_SHORTNAME.len() > 0 && opt.shortname.eq(HELP_SHORTNAME) {
                panic!("Cannot add help, as an option with shortlabel '{}' already exists in this ArgParser instance.", HELP_SHORTNAME);
            }
            if opt.longname.eq(HELP_LONGNAME) {
                panic!("Cannot add help, as an option with longname '{}' already exists in this ArgParser instance.", HELP_LONGNAME);
            }
        }

        // Create the option
        let result = Option {
            uid               : String::from(HELP_UID),
            shortname         : String::from(HELP_SHORTNAME),
            longname          : String::from(HELP_LONGNAME),
            min_n_values      : 0,
            max_n_values      : 0,
            param_description : String::new(),
            description       : String::from(HELP_DESCRIPTION)
        };

        // Store the option, but at the start of the vector
        self.options.push(result);

        // Also note the help is defined as special
        self.use_help = true;
    }



    /// Generates the usage string for this argument instance.
    /// 
    /// Note that this string is not terminated by a newline.
    /// 
    /// **Arguments**
    ///  * `exec_name`: The name of the executable.
    /// **Returns**  
    /// A string with the usage for this instance.
    pub fn get_usage(&self, exec_name: &str) -> String {
        // Create a new string
        let mut result: String = String::new();

        // Add the exectable name
        result.push_str("Usage: ");
        result.push_str(exec_name);

        // Add the options placeholder
        if self.options.len() > 0 { result.push_str(" [options]"); }

        // Add the positionals
        for pos in self.positionals.iter() {
            result.push_str(format!(" <{}>", pos.name).as_str());
        }

        // Return it!
        return result;
    }

    /// Helper function that prints the given positional to the given string, neatly formatted and line-wrapped.  
    /// Note that the string will be assuming it is written after a newline, and will terminate itself with newlines too.
    /// 
    /// Note that this function will panic! is the given uid doesn't exists.
    ///
    /// **Arguments**
    ///  * `result`: The resulting string to write to.
    ///  * `uid': The uid of the positional to write its help string for.
    ///  * `indent_width`: The prefix width of each new line. Also the space positionals have before they interrupt the description column.
    ///  * `line_width`: The total line width of each line.
    pub fn print_pos_help(&self, result: &mut String, uid: &str, indent_width: usize, line_width: usize) {
        // Try to find the positional
        let mut opt_pos: std::option::Option<&Positional> = None;
        for p in self.positionals.iter() {
            if p.uid.eq(uid) {
                opt_pos = Some(p);
                break;
            }
        }
        if let None = opt_pos { panic!("Unknown positional '{}'.", uid); }
        let pos = opt_pos.unwrap();

        // Prepare the argument string and write it
        let pos_name = format!("  <{}>", pos.name);
        result.push_str(pos_name.as_str());

        // Either pad the string until the description column, or add a newline
        if 2 + pos_name.len() >= indent_width {
            // Add a new line plus the indent
            result.reserve(1 + indent_width);
            result.push('\n');
            result.push_str(ArgParser::generate_spaces(indent_width).as_str());
        } else {
            result.push_str(ArgParser::generate_spaces(indent_width - pos_name.len()).as_str());
        }

        // Start writing the lines, linewrapped
        let mut x: usize = indent_width;
        ArgParser::print_description(result, &mut x, pos.description.as_str(), indent_width, line_width);

        // Write a final newline character and we're done
        result.push('\n');
    }

    /// Helper function that prints the given option to the given string, neatly formatted and line-wrapped.  
    /// Note that the string will be assuming it is written after a newline, and will terminate itself with newlines too.
    /// 
    /// Note that this function will panic! is the given uid doesn't exists.
    ///
    /// **Arguments**
    ///  * `result`: The resulting string to write to.
    ///  * `uid': The uid of the option to write its help string for.
    ///  * `indent_width`: The prefix width of each new line. Also the space options have before they interrupt the description column.
    ///  * `line_width`: The total line width of each line.
    pub fn print_opt_help(&self, result: &mut String, uid: &str, indent_width: usize, line_width: usize) {
        // Try to find the positional
        let mut opt_opt: std::option::Option<&Option> = None;
        for o in self.options.iter() {
            if o.uid.eq(uid) {
                opt_opt = Some(o);
                break;
            }
        }
        if let None = opt_opt { panic!("Unknown option '{}'.", uid); }
        let opt = opt_opt.unwrap();

        // Prepare the argument string and write it
        let opt_name = format!("  {}--{}{}", if opt.shortname.len() > 0 { format!("-{},", opt.shortname) } else { String::new() }, opt.longname, if opt.param_description.len() > 0 { format!(" {}", opt.param_description) } else { String::new() });
        result.push_str(opt_name.as_str());

        // Either pad the string until the description column, or add a newline
        if 2 + opt_name.len() >= indent_width {
            // Add a new line plus the indent
            result.reserve(1 + indent_width);
            result.push('\n');
            result.push_str(ArgParser::generate_spaces(indent_width).as_str());
        } else {
            result.push_str(ArgParser::generate_spaces(indent_width - opt_name.len()).as_str());
        }

        // Start writing the lines, linewrapped
        let mut x: usize = indent_width;
        ArgParser::print_description(result, &mut x, opt.description.as_str(), indent_width, line_width);

        // Write a final newline character and we're done
        result.push('\n');
    }

    /// Generates the help string for this argument instance.
    /// 
    /// Formatted to be copy/pasted immediately to stdout or something.
    /// 
    /// **Arguments**
    ///  * `exec_name`: The name of the executable.
    ///  * `indent_width`: The prefix width of each new line. Also the space options have before they interrupt the description column. A good default is `20`.
    ///  * `line_width`: The total line width of each line. A good default is 80.
    /// **Returns**  
    /// A string with the help for this instance.
    pub fn get_help(&self, exec_name: &str, indent_width: usize, line_width: usize) -> String {
        // Create a new string
        let mut result: String = String::new();

        // Print the usage string
        result.push_str("\n");
        result.push_str(format!("{}\n", self.get_usage(exec_name).as_str()).as_str());
        result.push_str("\n\n");

        // Print the positionals
        result.push_str("Positionals:\n");
        for p in self.positionals.iter() {
            // Print it
            self.print_pos_help(&mut result, &p.uid, indent_width, line_width);
        }

        // Print the options
        result.push_str("\nOptions:\n");
        for o in self.options.iter() {
            // Print it
            self.print_opt_help(&mut result, &o.uid, indent_width, line_width);
        }
        result.push('\n');

        // Done!
        return result;
    }



    /// Tries to parse the internally defined positionals and arguments according to the given list of arguments.
    /// 
    /// ** Arguments **
    ///  * `args`: The list of arguments, as a vector of str's.
    /// 
    /// ** Returns **
    /// An ArgDict with the results. If any errors occurred, parses no errors and adds the relevant errors to the dict. If help is given and the user gave it too, only that option is present in the ArgDict.
    pub fn parse(&self, args: &Vec<String>) -> ArgDict {
        // Prepare the resulting dict of arguments
        let mut result = ArgDict::new();

        // Now go through the arguments to parse them
        let mut positional_i = 0;
        let mut parse_options = true;
        let mut i: usize = 1;
        while i < args.len() {
            // Get the argument and its iterator
            let arg = &args[i];
            let sarg = OpString::new(arg);
            if sarg.len() == 0 { continue; }

            // First, split on option or not
            if parse_options && sarg[0].eq("-") {
                // It's an option
                if sarg.len() == 1 {
                    result.errors.push(String::from("Missing character after '-'."));
                    i += 1;
                    continue;
                }

                // If it's the double dash case, then stop parsing double values
                if self.use_double_dash && sarg.len() == 2 && sarg[1].eq("-") {
                    parse_options = false;
                    i += 1;
                    continue;
                }

                // Check if single dash or double dash
                if !sarg[1].eq("-") || (!self.use_double_dash && sarg.len() == 2) {
                    // Single dash; shortoption
                    let mut found = false;
                    let mut error = false;
                    for o in self.options.iter() {
                        if o.shortname.eq(sarg[1]) {
                            // It's a match!

                            // Make sure it's legal
                            if sarg.len() > 2 {
                                if o.max_n_values == 0 {
                                    // No values at all supported
                                    result.errors.push(format!("Option '-{}' cannot accept values (is passed '{}').", o.shortname, &arg[sarg.translate_opstr(2)..]));
                                    error = true;
                                    break;
                                } else if o.max_n_values > 1 {
                                    // More values supported
                                    result.errors.push(format!("Passing a value immediately after an option is only supported for options with at most 1 value ('-{}' has at most {}).", o.shortname, o.max_n_values));
                                    error = true;
                                    break;
                                }
                            }

                            // Now make sure the option is defined
                            if !result.options.contains_key(&o.uid) {
                                result.options.insert(o.uid.clone(), (o.shortname.clone(), o.longname.clone(), Vec::new()));
                            }
                            let values = &mut result.options.get_mut(&o.uid).unwrap().2;
                            
                            // Add the values as needed
                            if sarg.len() > 2 {
                                // We know that the number of arguments make sense, so add the rest as a value
                                values.push(String::from(&arg[sarg.translate_opstr(2)..]));

                            } else if o.max_n_values > 0 {
                                // Parse the rest of the arguments as values
                                let mut new_values = ArgParser::parse_values(args, &mut i, o.max_n_values, &mut parse_options, self.use_double_dash);
                                values.append(&mut new_values);

                            }

                            // We're done
                            found = true;
                            break;
                        }
                    }

                    // If not found, throw an error
                    if !found {
                        if !error { result.errors.push(format!("Unknown option '{}'{}", arg, if self.use_help { "; use '--help' to see an overview of accepted options." } else { "" })); }
                        i += 1;
                        continue;
                    }

                } else {
                    // Double dash; use longoption
                    let mut found = false;
                    let mut error = false;
                    let larg = &arg[sarg.translate_opstr(2)..];
                    for o in self.options.iter() {
                        if o.longname.eq(&larg[..o.longname.len()]) {
                            // It's a match!

                            // Make sure its legal
                            if larg.len() > o.longname.len() {
                                if !sarg[2 + o.longname.len()].eq("=") {
                                    // Not yet the end; continue instead
                                    continue;
                                } else if o.max_n_values == 0 {
                                    // No values at all supported
                                    result.errors.push(format!("Option '--{}' cannot accept values (is passed '{}').", o.longname, &arg[2 + o.longname.len() + 1..]));
                                    error = true;
                                    break;
                                } else if o.max_n_values > 1 {
                                    // More values supported
                                    result.errors.push(format!("Passing a value immediately after an option is only supported for options with at most 1 value ('--{}' has at most {}).", o.longname, o.max_n_values));
                                    error = true;
                                    break;
                                }
                            }

                            // Otherwise, make sure the option is defined
                            if !result.options.contains_key(&o.uid) {
                                result.options.insert(o.uid.clone(), (o.shortname.clone(), o.longname.clone(), Vec::new()));
                            }
                            let values = &mut result.options.get_mut(&o.uid).unwrap().2;

                            // Add the values as needed
                            if larg.len() > o.longname.len() {
                                // We know that the equal sign and number of arguments make sense, so add the rest as a value
                                values.push(String::from(&arg[2 + o.longname.len() + 1..]));

                            } else if o.max_n_values > 0 {
                                // Parse the rest of the arguments as values
                                let mut new_values = ArgParser::parse_values(args, &mut i, o.max_n_values, &mut parse_options, self.use_double_dash);
                                values.append(&mut new_values);

                            }

                            // We're done
                            found = true;
                            break;
                        }
                    }

                    // If not found, throw an error
                    if !found {
                        if !error { result.errors.push(format!("Unknown option '{}'{}", arg, if self.use_help { "; use '--help' to see an overview of accepted options." } else { "" })); }
                        i += 1;
                        continue;
                    }
                }

            } else {
                // It's a positional; check if we have any registered
                if positional_i >= self.positionals.len() {
                    result.warnings.push(format!("Skipping positional '{}' (index {})...", sarg, positional_i));
                    i += 1;
                    positional_i += 1;
                    continue;
                }

                // We have, so add it
                result.positionals.insert(self.positionals[positional_i].uid.clone(), (self.positionals[positional_i].index, arg.clone()));
                positional_i += 1;

            }

            // Done, increment i
            i += 1;
        }

        // Check if each option has enough values
        for opt in self.options.iter() {
            // See if this one appears in the output
            if result.options.contains_key(&opt.uid) {
                let values = &result.options.get(&opt.uid).unwrap().2;
                if values.len() < opt.min_n_values as usize {
                    result.errors.push(format!("Not enough values for '--{}': expected {}, got {}.", opt.longname, opt.min_n_values, values.len()));
                }
            }
        }

        // Clear the values if help is given (leaving help in that case) or, if not, there are errors
        if self.use_help && result.options.contains_key(HELP_UID) {
            // Clear the positionals
            result.positionals.clear();
            // Clear the options, so that's everything except help
            result.options.retain(|key, _| key.eq(HELP_UID) );
        } else if result.errors.len() > 0 {
            // Clear everything
            result.positionals.clear();
            result.options.clear();
        }
        
        // Done! Return the result
        return result;
    }

}





/***** ARGDICT CLASS *****/
/// Defines a dictionary that is returned by the ArgParser, and can be used to lookup parsed positionals and options.
pub struct ArgDict {
    /// Stores the parsed positionals. Each positional is mapped to its uid, and contains its index and string value.
    positionals : PositionalHashMap,
    /// Stores the parsed options. Each option is mapped to its uid.
    options     : OptionHashMap,

    /// Stores any warnings encountered during parsing.
    warnings    : Vec<String>,
    /// Stores any errors encountered during parsing. If this is non-empty, then there won't be any positionals or options either.
    errors      : Vec<String>,
}

/// Defines the ArgDict's methods
impl ArgDict {
    /// Private constructor for the ArgDict
    fn new() -> ArgDict {
        ArgDict {
            positionals : PositionalHashMap::new(),
            options     : OptionHashMap::new(),
            warnings    : Vec::new(),
            errors      : Vec::new()
        }
    }



    /// Checks if any warnings occurred during parsing.
    /// 
    /// **Returns**  
    /// `true` if warnings occurred, or `false` if they didn't.
    #[inline]
    pub fn has_warnings(&self) -> bool {
        self.warnings.len() > 0
    }

    /// If warnings occurred, prints them one-by-one to stderr.  
    /// If there are no warnings, does nothing.
    pub fn print_warnings(&self) {
        // Simply print them all on the next line
        for w in self.warnings.iter() {
            eprintln!("{}", w);
        }
    }



    /// Checks if any errors occurred during parsing.
    /// 
    /// **Returns**  
    /// `true` if errors occurred, or `false` if they didn't.
    #[inline]
    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    /// If errors occurred, prints them one-by-one to stderr.  
    /// If there are no errors, does nothing.
    pub fn print_errors(&self) {
        // Simply print them all on the next line
        for e in self.errors.iter() {
            eprint!("{}\n", e);
        }
    }



    /// Checks if a positional with the given uid is given by the user.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the positional to check.
    /// 
    /// **Returns**  
    /// Whether or not the positional is given, as a boolean.
    #[inline]
    pub fn has_pos(&self, uid: &str) -> bool {
        self.positionals.contains_key(uid)
    }

    /// Checks if an option with the given uid is given by the user.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the option to check.
    /// 
    /// **Returns**  
    /// Whether or not the option is given, as a boolean.
    #[inline]
    pub fn has_opt(&self, uid: &str) -> bool {
        self.options.contains_key(uid)
    }



    /// Returns the index of the given positional.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the positional whos index we want to get.
    /// 
    /// **Returns**  
    /// An Option with either the index of the given positional or 'none'.
    pub fn get_pos_index(&self, uid: &str) -> std::option::Option<usize> {
        if self.has_pos(uid) {
            return Some(self.positionals.get(uid).unwrap().0);
        } else {
            return None;
        }
    }

    /// Returns the value of the positional with the given uid.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the positional to get.
    /// 
    /// **Returns**  
    /// An Option that is either the value of the positional or 'none'.
    pub fn get_pos(&self, uid: &str) -> std::option::Option<&String> {
        if self.has_pos(uid) {
            return Some(&self.positionals.get(uid).unwrap().1);
        } else {
            return None;
        }
    }


    
    /// Returns the shortname of the option with the given uid.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the option to get.
    /// 
    /// **Returns**  
    /// An Option that is either the shortname of the option or 'none'.
    pub fn get_opt_shortname(&self, uid: &str) -> std::option::Option<&str> {
        if self.has_opt(uid) {
            return Some(self.options.get(uid).unwrap().0.as_str());
        } else {
            return None;
        }
    }
    
    /// Returns the longname of the option with the given uid.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the option to get.
    /// 
    /// **Returns**  
    /// An Option that is either the longname of the option or 'none'.
    pub fn get_opt_longname(&self, uid: &str) -> std::option::Option<&String> {
        if self.has_opt(uid) {
            return Some(&self.options.get(uid).unwrap().1);
        } else {
            return None;
        }
    }

    /// Returns the value(s) of the option with the given uid.
    /// 
    /// If the Option has no value, returns an empty list.
    /// 
    /// **Arguments**
    ///  * `uid`: The uid of the option to get.
    /// 
    /// **Returns**  
    /// An Option that is either the values of the option as a list of Strings or 'none'.
    pub fn get_opt(&self, uid: &str) -> std::option::Option<&Vec<String>> {
        if self.has_opt(uid) {
            return Some(&self.options.get(uid).unwrap().2);
        } else {
            return None;
        }
    }

}
