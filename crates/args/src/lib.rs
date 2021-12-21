/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   21 Dec 2021, 16:21:49
 * Last edited:
 *   21 Dec 2021, 21:20:39
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The parse-args library, which contains code to parse command line
 *   arguments.
**/


pub mod parse_args {
    // Make the normal HashMap easily available
    use std::collections::HashMap;
    /// Defines a shortcut for the Positional's HashMap in the ArgsDict.
    type PositionalHashMap = HashMap<String, (u32, String)>;
    /// Defines a shortcut for the Option's HashMap in the ArgsDict.
    type OptionHashMap = HashMap<String, (char, String, Vec<String>)>;



    /// The uid used for the help argument.
    pub const HELP_UID: &str = "help";
    /// The shortname used for the help argument.
    pub const HELP_SHORTNAME: char = 'h';
    /// The longname used for the help argument.
    pub const HELP_LONGNAME: &str = "help";
    /// The description used for the help argument.
    pub const HELP_DESCRIPTION: &str = "Shows this list of arguments, then quits.";



    /// Intermediate representation for a Positional.
    struct Positional {
        /// The uid for this positional.
        uid         : String,
        /// The index of this positional.
        index       : u32,
        /// The human-readable name for this positional. Used in the usage/help string.
        name        : String,
        /// The description for this positional.
        description : String,
    }

    /// Intermediate representation for an Option.
    struct Option {
        /// The uid for this option.
        uid          : String,
        /// The shortname for this option. Will be the empty char (`\0`) if unused.
        shortname    : char,
        /// The longname for this option.
        longname     : String,
        /// The minimum number of values for this option.
        min_n_values : u32,
        /// The maximum number of values for this option.
        max_n_values : u32,
        /// The description for this option.
        description  : String,
    }

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
        ///  * `parse_opts`: Whether or not options are still allowed to be parsed.
        /// **Returns**
        /// The popped arguments, of which there will be at most max_n.
        fn parse_values(args: &Vec<String>, i: &mut usize, max_n: u32, parse_opts: bool) -> Vec<String> {
            // Increment i to skip the option itself
            *i += 1;

            // Try to pop
            let mut result: Vec<String> = Vec::new();
            while *i < args.len() && *i < max_n as usize {
                // Get the argument
                let arg = &args[*i];

                // If it's empty, skip; otherwise, get the first char
                if arg.len() == 0 { continue; }
                let first_char: char = arg.chars().next().unwrap();

                // If it's an option, stop
                if parse_opts && first_char == '-' {
                    break;
                }

                // Otherwise, add to the result
                result.push(arg.clone());

                // Increment i
                *i += 1;
            }

            // i is now at the first unparseable thing; fix this
            *i -= 1;

            // Return the result struct
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
        fn print_description(result: &mut String, x: &mut u32, description: &str, indent_width: u32, line_width: u32) {
            // Go through the description word-by-word
            let mut word_s: usize = 0;
            let mut word_e: usize = 0;
            loop {
                // Get the next word
                while word_e < description.len() {
                    // If it's at a space, stop
                    let c = description.chars().next().unwrap();
                    if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                        break;
                    }

                    // Increment word_e
                    word_e += 1
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
                index: self.positionals.len() as u32,
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
        ///  * `shortname`: A single character, optional identifier for the option. Must be unique across all options. If you don't want to use it, pass a null-character (`\0`).
        ///  * `longname`: A multi-character identifier for the option. Must be unique across all options.
        ///  * `min_n_values`: The minimum number of values for this option. If it's a flag, pass no argument (0).
        ///  * `max_n_values`: The maximum number of values for this option. If it's a flag, pass no argument (0). Cannot be smaller than `min_n_values`.
        ///  * `description`: A string description of the option.
        pub fn add_opt(&mut self, uid: &str, shortname: char, longname: &str, min_n_values: u32, max_n_values: u32, description: &str) {
            // Check if the uid, shortname or longnames are in conflict
            for opt in self.options.iter() {
                if opt.uid.eq(uid) {
                    panic!("An option with uid '{}' already exists in this ArgParser instance.", uid);
                }
                if shortname != '\0' && opt.shortname == shortname {
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
                uid: String::from(uid),
                shortname,
                longname: String::from(longname),
                min_n_values,
                max_n_values,
                description: String::from(description)
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
                if HELP_SHORTNAME != '\0' && opt.shortname == HELP_SHORTNAME {
                    panic!("Cannot add help, as an option with shortlabel '{}' already exists in this ArgParser instance.", HELP_SHORTNAME);
                }
                if opt.longname.eq(HELP_LONGNAME) {
                    panic!("Cannot add help, as an option with longname '{}' already exists in this ArgParser instance.", HELP_LONGNAME);
                }
            }

            // Create the option
            let result = Option {
                uid: String::from(HELP_UID),
                shortname: HELP_SHORTNAME,
                longname: String::from(HELP_LONGNAME),
                min_n_values: 0,
                max_n_values: 0,
                description: String::from(HELP_DESCRIPTION)
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
                result.push_str(format!(" <{}>", pos.name));
            }

            // Return it!
            return result;
        }

        /// Generates the help string for this argument instance.
        /// 
        /// Formatted to be copy/pasted immediately to stdout or something.
        /// 
        /// **Arguments**
        ///  * `exec_name`: The name of the executable.
        /// **Returns**
        /// A string with the help for this instance.
        pub fn get_help(&self, exec_name: &str) -> String {
            // Create a new string
            let mut result: String = String::new();

            // Print the usage string
            result.push_str("\n");
            result.push_str(format!("{}\n", self.get_usage(exec_name).as_str()));
            result.push_str("\n\n");

            // Print the positionals
            result.push_str("Positionals:\n");
            
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
                // Get the argument, and make sure it isn't empty
                let arg = &args[i];
                if arg.len() == 0 { continue; }

                // Get the first character
                let first_char: char = arg.chars().next().unwrap();

                // If the argument starts with a dash, parse it as an option (as long as that's allowed)
                if parse_options && first_char == '-' {
                    // Throw errors if nothing follows
                    if arg.len() == 1 {
                        result.errors.push(String::from("Missing option name after '-'."));
                        i += 1;
                        continue;
                    }

                    // Get the second character
                    let second_char: char = arg.chars().next().unwrap();

                    // If it's the dash and we treat is specially, then treat is special
                    if self.use_double_dash && second_char == '-' {
                        parse_options = false;
                        i += 1;
                        continue;
                    }

                    // Otherwise, split into shortname or longname search
                    if second_char != '-' {
                        // Shortname

                        // Search through the options to find a match
                        let mut status = 0;
                        for opt in self.options.iter() {
                            if opt.shortname == second_char {
                                // First, get the current list of values (if any)
                                let mut new_values = Vec::new();
                                let opt_values: &mut Vec<String>;
                                if result.options.contains_key(&opt.uid) {
                                    opt_values = &mut result.options.get_mut(&opt.uid).unwrap().2;
                                } else {
                                    opt_values = &mut new_values;
                                }

                                // It matches; see if we need to parse values
                                if arg.len() > 2 {
                                    // It could be directly following the option itself
                                    if opt.max_n_values == 1 {
                                        // Make sure we didn't already see one
                                        if opt_values.len() + 1 > opt.max_n_values as usize {
                                            result.errors.push(String::from(format!("Too many values given for '-{}': expected {}, got {}.", opt.shortname, opt.max_n_values, opt_values.len() + 1)));
                                            status = -1;
                                            break;
                                        }

                                        // It is; add the value
                                        opt_values.push(String::from(&arg[2..]));
                                    } else {
                                        // Wtf is this
                                        result.errors.push(String::from(format!("Got value directly after '-{}', which is only supported for options with at most 1 value.", opt.shortname)));
                                        status = -1;
                                        break;
                                    }
                                } else if opt.max_n_values > 0 {
                                    // Search for the values and add them to the list
                                    let mut values = ArgParser::parse_values(&args, &mut i, opt.max_n_values - opt_values.len() as u32, parse_options);
                                    opt_values.append(&mut values);
                                }

                                // Insert it with the new values if we hadn't already
                                if !result.options.contains_key(&opt.uid) {
                                    result.options.insert(opt.uid.clone(), (opt.shortname, opt.longname.clone(), new_values));
                                }

                                // Quit searching
                                status = 1;
                                break;
                            }
                        }
                        if status != 1 {
                            if status == 0 { result.errors.push(String::from(format!("Unknown option '{}'{}", arg, if self.use_help { format!("; type '--{}' to see a list of options.", HELP_LONGNAME) } else { String::new() }))); }
                            i += 1;
                            continue;
                        }
                    } else {
                        // Longname

                        // Search through the options to find a match
                        let mut status = 0;
                        for opt in self.options.iter() {
                            if arg[2..2 + opt.longname.len()].eq(&opt.longname) && (arg.len() <= 2 + opt.longname.len() || arg.chars().nth(2 + opt.longname.len()).unwrap() == '=') {
                                // First, get the current list of values (if any)
                                let mut new_values = Vec::new();
                                let opt_values: &mut Vec<String>;
                                if result.options.contains_key(&opt.uid) {
                                    opt_values = &mut result.options.get_mut(&opt.uid).unwrap().2;
                                } else {
                                    opt_values = &mut new_values;
                                }

                                // It matches; see if we need to parse values
                                if arg.len() > 2 + opt.longname.len() + 1 {
                                    // It could be following the option itself with an equal sign
                                    if opt.max_n_values == 1 {
                                        // Make sure we didn't already see one
                                        if opt_values.len() + 1 > opt.max_n_values as usize {
                                            result.errors.push(String::from(format!("Too many values given for '--{}': expected {}, got {}.", opt.longname, opt.max_n_values, opt_values.len() + 1)));
                                            status = -1;
                                            break;
                                        }

                                        // It is; add the value
                                        opt_values.push(String::from(&arg[2 + opt.longname.len() + 1..]));
                                    } else {
                                        // Wtf is this
                                        result.errors.push(String::from(format!("Got value after '--{}', which is only supported for options with at most 1 value.", opt.longname)));
                                        status = -1;
                                        break;
                                    }
                                } else if opt.max_n_values > 0 {
                                    // Search for the values and add them to the list
                                    let mut values = ArgParser::parse_values(&args, &mut i, opt.max_n_values - opt_values.len() as u32, parse_options);
                                    opt_values.append(&mut values);
                                }

                                // Insert it with the new values if we hadn't already
                                if !result.options.contains_key(&opt.uid) {
                                    result.options.insert(opt.uid.clone(), (opt.shortname, opt.longname.clone(), new_values));
                                }

                                // Quit searching
                                status = 1;
                                break;
                            }
                        }
                        if status != 1 {
                            if status == 0 { result.errors.push(String::from(format!("Unknown option '{}'{}", arg, if self.use_help { format!("; type '--{}' to see a list of options.", HELP_LONGNAME) } else { String::new() }))); }
                            i += 1;
                            continue;
                        }
                    }

                } else {
                    // Check if we have a positional for this index
                    if positional_i >= self.positionals.len() {
                        // Ignore it; add the warning
                        result.warnings.push(String::from(format!("Ignoring positional '{}' at index {}", arg, positional_i)));
                        positional_i += 1;
                        i += 1;
                        continue;
                    }

                    // Store it
                    let pos: &Positional = &self.positionals[positional_i];
                    result.positionals.insert(
                        pos.uid.clone(), (pos.index, arg.clone())
                    );

                    // Increment the positional i
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
                result.options.retain(|key, _| !key.eq(HELP_UID) );
            } else if result.errors.len() > 0 {
                result.positionals.clear();
                result.options.clear();
            }
            
            // Done! Return the result
            return result;
        }

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



        /// Checks if a positional with the given uid is given by the user.
        /// 
        /// **Arguments**
        ///  * `uid`: The uid of the positional to check.
        /// 
        /// ** Returns **
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
        /// ** Returns **
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
        pub fn get_pos_index(&self, uid: &str) -> std::option::Option<u32> {
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
        pub fn get_opt_shortname(&self, uid: &str) -> std::option::Option<char> {
            if self.has_opt(uid) {
                return Some(self.options.get(uid).unwrap().0);
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

}
