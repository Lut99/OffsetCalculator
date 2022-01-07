/* SESSION.rs
 *   by Lut99
 *
 * Created:
 *   07 Jan 2022, 10:09:06
 * Last edited:
 *   07 Jan 2022, 11:48:07
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains code for easily loading and writing a session from/to disk.
**/

use std::io::prelude::*;
use std::fs::File;

use rustyline::Editor;

use crate::ast::symbol_table::SymbolTable;
use crate::ast::parser::ValueKind;


/***** HELPER ENUMS *****/
/// Enum that defines the possible states of the reader.
#[derive(Copy, Clone, PartialEq)]
enum ReaderState {
    /// First state in the parser.
    Start,

    /// We're reading history lines.
    History,
    /// We're reading SymbolTable lines.
    SymbolTable,
}





/***** HELPER ERRORS *****/
/// Defines errors for unescaping strings.
#[derive(Debug)]
pub enum UnescapeError {
    MissingCharacterError{ pos: usize },
    IllegalCharacterError{ illegal_char: char, pos: usize },
    UnescapedCharacterError{ unescaped_char: char, pos: usize },
}

impl UnescapeError {
    /// Returns the column position of the error.
    pub fn pos(&self) -> usize {
        match self {
            UnescapeError::MissingCharacterError{ pos }                      => { *pos }
            UnescapeError::IllegalCharacterError{ illegal_char: _, pos }     => { *pos }
            UnescapeError::UnescapedCharacterError{ unescaped_char: _, pos } => { *pos }
        }
    }
}
impl std::fmt::Display for UnescapeError {
    /// Write the error message to some formatter
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnescapeError::MissingCharacterError{ pos: _ }                   => { write!(f, "Missing character after escape char '\\'") }
            UnescapeError::IllegalCharacterError{ illegal_char, pos: _ }     => { write!(f, "Illegal character '{}' after escape character '\\'", illegal_char) }
            UnescapeError::UnescapedCharacterError{ unescaped_char, pos: _ } => { write!(f, "Encountered unescaped character '{}'", unescaped_char) }
        }
    }
}
impl std::error::Error for UnescapeError {}





/***** HELPER FUNCTIONS *****/
/// Strips the whitespaces at the start and end of the given string.
/// 
/// **Arguments**
///  * `s`: The string to strip.
fn strip(s: &str) -> String {
    // Define the iterator
    let chars = s.chars().collect::<Vec<char>>();

    // Loop the first part
    let mut start_i = 0;
    while start_i < s.len() {
        if chars[start_i] != ' ' && chars[start_i] != '\t' && chars[start_i] != '\r' && chars[start_i] != '\n' {
            break;
        }
        start_i += 1;
    }

    // Loop the last part
    let mut end_i = s.len();
    while end_i > 0 {
        if chars[end_i - 1] != ' ' && chars[end_i - 1] != '\t' && chars[end_i - 1] != '\r' && chars[end_i - 1] != '\n' {
            break;
        }
        end_i -= 1;
    }

    // If start_i > end_i, it's only whitespaces
    if start_i > end_i { return String::new(); }

    // Return the slice
    return String::from(&s[start_i..end_i]);
}

/// Splits the given string on the given character.
/// 
/// **Arguments**
///  * `s`: The string to split.
///  * `c`: The character to split on.
/// 
/// **Result**  
/// A Vec<&str> containing the split parts.
fn split(s: &str, c: char) -> Vec<&str> {
    // Prepare the return list
    let mut res: Vec<&str> = Vec::new();
    res.reserve(3);

    // Iterate over the string
    let mut last_i = 0;
    let mut i = 0;
    for sc in s.chars() {
        if sc == c {
            // Add the split
            res.push(&s[last_i..i]);
            last_i = i + 1;
        }

        // Increment the i
        i += 1;
    }
    // Add the final bit
    res.push(&s[last_i..]);

    // Done, return
    return res;
}

/// Makes the given history string safe for storage by escaping the necessary characters.
/// 
/// **Arguments**
///  * `line`: The session line to escape.
/// 
/// **Returns**  
/// A new String that is the same but escaped.
fn escape(line: &str) -> String {
    let mut result = String::new();
    result.reserve(line.len());

    // Resolve the escaped characters in the line
    let mut iter = line.chars();
    let mut ic = iter.next();
    while ic != None {
        let c = ic.unwrap();
        if c == '\\' || c == '[' || c == ']' {
            // Write the escape char first
            result.push('\\');
        }

        // Write the char
        result.push(c);

        // Go to the next char
        ic = iter.next();
    }

    // Done, return
    result
}

/// Makes the given stored history string safe for usage by unescaping the necessary characters.
/// 
/// **Arguments**
///  * `line`: The session line to unescape.
///  * `l`: The line number to write.
///  * `line`: The session line to unescape.
/// 
/// **Returns**  
/// A new String that is the same but unescaped, or else an UnescapeError explaining what happened.
fn unescape(line: &str) -> Result<String, UnescapeError> {
    let mut result = String::new();
    result.reserve(line.len());

    // Resolve the escaped characters in the line
    let mut i = 1;
    let mut iter = line.chars();
    let mut ic = iter.next();
    while ic != None {
        let c = ic.unwrap();
        if c == '\\' {
            // Get the next character to know what it is
            ic = iter.next();
            if ic == None {
                return Err(UnescapeError::MissingCharacterError{ pos: i });
            }
            let c = ic.unwrap();

            // Match it
            if c == '[' || c == ']' || c == '\\' {
                // Add this one
                result.push(c);
            } else {
                return Err(UnescapeError::IllegalCharacterError{ illegal_char: c, pos: i });
            }
        } else if c == '[' || c == ']' {
            return Err(UnescapeError::UnescapedCharacterError{ unescaped_char: c, pos: i });
        } else {
            // Simply write it to the buffer
            result.push(c);
        }

        // Go to the next char
        ic = iter.next();
        i += 1;
    }

    // Done, return
    Ok(result)
}





/***** LIBRARY ERRORS *****/
/// Defines errors for loading or writing a session.
#[derive(Debug)]
pub enum SessionError {
    /// Error for when the session file couldn't be opened.
    OpenFileError{ path: String, error: std::io::Error },
    /// Error for when the session file couldn't be read.
    ReadLineError{ path: String, line: usize, error: std::io::Error },
    /// Error for when the session file couldn't be written to.
    WriteError{ path: String, error: std::io::Error },
}

impl SessionError {
    /// Returns the column position of the error.
    pub fn path(&self) -> &str {
        match self {
            SessionError::OpenFileError{ path, error: _ }          => { &path }
            SessionError::ReadLineError{ path, line: _, error: _ } => { &path }
            SessionError::WriteError{ path, error: _ }             => { &path }
        }
    }
}
impl std::fmt::Display for SessionError {
    /// Write the error message to some formatter
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::OpenFileError{ path: _, error }       => { write!(f, "Could not open session file: {}", error) }
            SessionError::ReadLineError{ path: _, line, error } => { write!(f, "Could not read line {} from session file: {}", line, error) }
            SessionError::WriteError{ path: _, error }          => { write!(f, "Could not write to session file: {}", error) }
        }
    }
}
impl std::error::Error for SessionError {}





/***** LIBRARY FUNCTIONS *****/
/// Loads the session from the given path.
/// 
/// **Arguments**
///  * `path`: The path to load from.
///  * `symbol_table`: The SymbolTable to populate with the saved variables.
///  * `rl`: The RustyLine editor that will be populated with the history lines.
/// 
/// **Returns**  
/// Returns Ok() when everything went right, or err() with the reason when it didn't.
pub fn load(path: &str, symbol_table: &mut SymbolTable, rl: &mut Editor<()>) -> Result<(), SessionError> {
    // Start by trying to open the file
    let openres = File::open(path);
    if let Err(reason) = openres {
        // We failed
        return Err(SessionError::OpenFileError{ path: String::from(path), error: reason });
    }
    let file = openres.ok().unwrap();

    // Otherwise, start reading the lines
    let mut l = 1;
    let mut state = ReaderState::Start;
    for inline in std::io::BufReader::new(file).lines() {
        if let Err(reason) = inline {
            return Err(SessionError::ReadLineError{ path: String::from(path), line: l, error: reason });
        }
        let line = inline.ok().unwrap();

        // If it starts with a comment, ignore
        match state {
            ReaderState::Start => {
                // Strip the line
                let line = strip(&line);
                if line.len() == 0 {
                    // Skip line
                    l += 1;
                    continue;
                }

                // We only accept '[history]' and '[symtable]' lines
                if line.to_lowercase().eq("[history]") {
                    state = ReaderState::History;
                } else if line.to_lowercase().eq("[symtable]") {
                    state = ReaderState::SymbolTable;
                }
            }

            ReaderState::History => {
                // Strip the line
                let line = strip(&line);
                if line.len() == 0 {
                    // Skip line
                    l += 1;
                    continue;
                }

                // Handle special lines
                if line.to_lowercase().eq("[history]") {
                    // Ignore, as we're already in history
                    l += 1;
                    continue;
                } else if line.to_lowercase().eq("[symtable]") {
                    // Move to parsing symbol tables
                    state = ReaderState::SymbolTable;
                    l += 1;
                    continue;
                }

                // Unescape the history string
                let iunescaped = unescape(&line);
                if iunescaped.is_err() {
                    let e = iunescaped.err().unwrap();
                    eprintln!("{}:{}:{}: WARNING: {}; skipping line.", path, l, e.pos(), e);
                    l += 1;
                    continue;
                }
                let unescaped = iunescaped.ok().unwrap();

                // Add the line to the rl history
                rl.add_history_entry(unescaped);
            }

            ReaderState::SymbolTable => {
                // Strip the line
                let line = strip(&line);
                if line.len() == 0 {
                    // Skip line
                    l += 1;
                    continue;
                }

                // Handle special lines
                if line.to_lowercase().eq("[history]") {
                    // Move to parsing history
                    state = ReaderState::History;
                    l += 1;
                    continue;
                } else if line.to_lowercase().eq("[symtable]") {
                    // Ignore, as we're already in symtable
                    l += 1;
                    continue;
                }

                // Lines are in the format 'ID=KIND,VALUE', so split on that
                let eq_parts = split(&line, '=');
                if eq_parts.len() != 2 {
                    eprintln!("WARNING: Expected one equal sign, got {} on line {} of session file '{}'; skipping line.", eq_parts.len(), l, path);
                    l += 1;
                    continue;
                }

                // The first is the ID, so split the second on the comma
                let comma_parts = split(eq_parts[1], ',');
                if comma_parts.len() != 2 {
                    eprintln!("WARNING: Expected one comma after equal sign, got {} on line {} of session file '{}'; skipping line.", comma_parts.len(), l, path);
                    l += 1;
                    continue;
                }

                // Next, try to parse the first one as a kind
                let rkind = strip(comma_parts[0]);
                let ikind = rkind.parse::<ValueKind>();
                if ikind.is_err() {
                    eprintln!("WARNING: Unknown kind '{}' on line {} of session file '{}'; skipping line.", rkind, l, path);
                    l += 1;
                    continue;
                }
                let kind = ikind.ok().unwrap();

                // Then, parse the other as a value
                let rvalue = strip(comma_parts[1]);
                let ivalue = rvalue.parse::<u64>();
                if ivalue.is_err() {
                    eprintln!("WARNING: Cannot parse '{}' as u64 on line {} of session file '{}': {}.", rvalue, l, path, ivalue.err().unwrap());
                    eprintln!("Skipping line.");
                    l += 1;
                    continue;
                }
                let value = ivalue.ok().unwrap();

                // We did it! Add the symbol table entry
                symbol_table.insert(strip(eq_parts[0]), (kind, value));
            }
        }

        // Increment the line number
        l += 1;
    }

    // Done
    return Ok(());
}

/// Saves the session to the given path.
/// 
/// **Arguments**
///  * `path`: The path to save to.
///  * `symbol_table`: The SymbolTable to save.
///  * `rl`: The RustyLine editor with the history to save.
/// 
/// **Returns**  
/// Returns Ok() when everything went right, or err() with the reason when it didn't.
pub fn save(path: &str, symbol_table: &SymbolTable, rl: &Editor<()>) -> Result<(), SessionError> {
    // Start by trying to create the file
    let createres = File::create(path);
    if createres.is_err() {
        // We failed
        return Err(SessionError::OpenFileError{ path: String::from(path), error: createres.err().unwrap() });
    }
    let mut file = createres.ok().unwrap();

    // First, write a header
    let writeres = write!(file, "SESSION FILE for OFFSETCALCULATOR\n   Generated by the OffsetCalculator\n\nThe file is split into two sections:\n - [history]: Stores all lines of the history in a session\n - [symtable]: Stores are variables.\nBefore a section is defined, the parses ignores anything, hence we can write this prelude!\n\n");
    if let Err(reason) = writeres {
        return Err(SessionError::WriteError{ path: String::from(path), error: reason })
    }

    // First, we write the history files
    if let Err(reason) = write!(file, "[history]\n") {
        return Err(SessionError::WriteError{ path: String::from(path), error: reason })
    }
    for i in 0..rl.history().len() {
        // Escape the line
        let escaped = escape(rl.history().get(i).unwrap());
        
        // Write it to the file
        if let Err(reason) = write!(file, "{}\n", escaped) {
            return Err(SessionError::WriteError{ path: String::from(path), error: reason })
        }
    }

    // Next, write the variables
    if let Err(reason) = write!(file, "\n[symtable]\n") {
        return Err(SessionError::WriteError{ path: String::from(path), error: reason })
    }
    for (identifier, (kind, value)) in symbol_table.iter() {
        // Write it to the file
        if let Err(reason) = write!(file, "{} = {:?}, {}\n", identifier, kind, value) {
            return Err(SessionError::WriteError{ path: String::from(path), error: reason })
        }
    }

    // Done
    return Ok(());
}
