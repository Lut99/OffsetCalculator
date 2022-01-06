/* TOKENIZER.rs
 *   by Lut99
 *
 * Created:
 *   03 Jan 2022, 10:27:03
 * Last edited:
 *   06 Jan 2022, 16:56:20
 * Auto updated?
 *   Yes
 *
 * Description:
 *   A Rust module that implements a tokenizer for the offsetcalculator.
**/

use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::Graphemes;

use crate::ast::symbols::TerminalKind;
use crate::ast::symbols::Token;


/***** HELPER MACROS *****/
/// Checks if the given 'char' equals a whitespace.
macro_rules! is_whitespace {
    ($c:expr) => {
        ($c.eq(" ") || $c.eq("\t") || $c.eq("\r") || $c.eq("\n"))
    };
}

/// Checks if the given 'char' equals a separator: a whitespace, '\0', or some way we know a new token starts.
macro_rules! is_separator {
    ($c:expr) => {
        (is_whitespace!($c) || $c.eq("\0") || $c.eq("+") || $c.eq("-") || $c.eq("*") || $c.eq("/") || $c.eq("(") || $c.eq(")"))
    };
}

/// Checks if the given 'char' equals a valid ID-start character.
macro_rules! is_id_start {
    ($c:expr) => {
        ($c.eq("a") || $c.eq("b") || $c.eq("c") || $c.eq("d") || $c.eq("e") || $c.eq("f") || $c.eq("g") || $c.eq("h") || $c.eq("i") || $c.eq("j") || $c.eq("k") || $c.eq("l") || $c.eq("m") || $c.eq("n") || $c.eq("o") || $c.eq("p") || $c.eq("q") || $c.eq("r") || $c.eq("s") || $c.eq("t") || $c.eq("u") || $c.eq("v") || $c.eq("w") || $c.eq("x") || $c.eq("y") || $c.eq("z") ||
         $c.eq("A") || $c.eq("B") || $c.eq("C") || $c.eq("D") || $c.eq("E") || $c.eq("F") || $c.eq("G") || $c.eq("H") || $c.eq("I") || $c.eq("K") || $c.eq("K") || $c.eq("L") || $c.eq("M") || $c.eq("N") || $c.eq("O") || $c.eq("P") || $c.eq("Q") || $c.eq("R") || $c.eq("S") || $c.eq("T") || $c.eq("U") || $c.eq("V") || $c.eq("W") || $c.eq("X") || $c.eq("Y") || $c.eq("Z") ||
         $c.eq("_"))
    };
}
/// Checks if the given 'char' equals a valid ID character.
macro_rules! is_id {
    ($c:expr) => {
        (is_id_start!($c) || is_numeric!($c))
    };
}

/// Checks if the given 'char' equals a digit.
macro_rules! is_numeric {
    ($c:expr) => {
        ($c.eq("0") || $c.eq("1") || $c.eq("2") || $c.eq("3") || $c.eq("4") ||
         $c.eq("5") || $c.eq("6") || $c.eq("7") || $c.eq("8") || $c.eq("9"))
    };
}

/// Checks if the given 'char' equals a hexadecimal digit.
macro_rules! is_hex {
    ($c:expr) => {
        ($c.eq("0") || $c.eq("1") || $c.eq("2") || $c.eq("3") || $c.eq("4") ||
         $c.eq("5") || $c.eq("6") || $c.eq("7") || $c.eq("8") || $c.eq("9") ||
         $c.eq("A") || $c.eq("B") || $c.eq("C") || $c.eq("D") || $c.eq("E") || $c.eq("F") ||
         $c.eq("a") || $c.eq("b") || $c.eq("c") || $c.eq("d") || $c.eq("e") || $c.eq("f"))
    };
}

/// Checks if the given 'char' equals a binary digit.
macro_rules! is_binary {
    ($c:expr) => {
        ($c.eq("0") || $c.eq("1"))
    };
}





/***** HELPER ENUMS *****/
enum TokenizerState {
    /// The start state
    Start,

    /// We found an ID
    Id,

    /// We found a '0'
    Zero,
    /// The state for when we NEED a digit
    FirstDigit,
    /// The state for when we find a digit
    Digit,
    /// The state for when we NEED a hexadecimal digit
    FirstHexDigit,
    /// The state for when we find a hexadecimal digit
    HexDigit,
    /// The state for when we NEED a binary digit
    FirstBinaryDigit,
    /// The state for when we find a binary digit
    BinaryDigit,

    /// The state for when we encountered an unknown token and want to consume it
    UnknownToken,
}





/***** TOKENIZER STRUCT *****/
pub struct Tokenizer<'a> {
    /// The input string we try to tokenize. ALready split into graphenes.
    input   : Graphemes<'a>,
    /// Temporary buffer of characters that are put back
    temp    : Vec<(&'a str, usize)>,
    /// The current position in the string.
    pos     : usize,
    /// The length of the parent string.
    max_pos : usize,
}

impl<'a> Tokenizer<'a> {
    /// Constructor for the Tokenizer.
    /// 
    /// **Arguments**
    ///  * `input`: The string to tokenize.
    /// 
    /// **Returns**  
    /// A newly constructed Tokenizer instance.
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        return Tokenizer {
            input   : UnicodeSegmentation::graphemes(input, true),
            temp    : Vec::new(),
            pos     : 0,
            max_pos : input.len()
        };
    }



    /// Returns the next character on the stream.
    /// 
    /// **Returns**  
    /// The next character and its position in the string, or '\0' and the length of the string if there is none.
    fn getc(&mut self) -> (&'a str, usize) {
        // If there's something in the buffer, return that instead
        if self.temp.len() > 0 {
            return self.temp.remove(self.temp.len() - 1);
        }

        // Otherwise, try to get the char
        let c   = self.input.next();
        if c == None { return ("\0", self.max_pos + 1); }

        // Otherwise, increment the pos and return
        self.pos += 1;
        return (c.unwrap(), self.pos);
    }

    /// Puts the given character back on the stream.
    /// 
    /// Doesn't really do that, but instead puts it in a stack that characters are returned from first.
    /// 
    /// **Arguments**  
    ///  * `c`: The character to put back.
    ///  * `pos`: The position of the character in the input string.
    fn putc(&mut self, c: &'a str, pos: usize) {
        // Simply put on the stack
        self.temp.push((c, pos));
    }

    /// Parses the given char that is a digit as an integer and adds it to the given buffer.
    /// 
    /// **Arguments**
    ///  * `value`: The value to add the character to.
    ///  * `c`: The character that is a digit.
    ///  * `radix`: The radix value for the conversion. Can either be '2' for binary, '10' for decimal or '16' for hex.
    /// 
    /// **Returns**  
    /// Whether or not parsing was successful. If it was, returns None, or the error message otherwise.
    fn parse_const(value: &mut u64, c: &str, radix: u32) -> Option<String> {
        // Convert the digit to a value
        let v: u64 = c.chars().next().unwrap().to_digit(radix).unwrap() as u64;

        // Add it to the value while trying to prevent overflow
        let mut result = value.checked_mul(radix as u64);
        if result == None {
            return Some(String::from("overflow for u64"));
        }
        result = result.unwrap().checked_add(v);
        if result == None {
            return Some(String::from("overflow for u64"));
        }
        *value = result.unwrap();

        // Done
        return None;
    }



    /// Tries to get the next token from the input stream.
    /// 
    /// **Returns**  
    /// The token wrapped in Ok() if true, or an Err() explaining why we couldn't get it.
    pub fn get(&mut self) -> Token {
        // Loop to emulate the destinaton
        let mut start_pos: usize = usize::MAX;
        let mut parsed_buffer: String = String::new();
        let mut value_buffer: u64 = 0;
        let mut state = TokenizerState::Start;
        loop {
            // Match the state
            match state {
                TokenizerState::Start => {
                    // Get the next character
                    let (c, pos) = self.getc();
                    start_pos = pos;

                    // Split on the character we see
                    if is_id_start!(c) {
                        // Start of an ID!
                        parsed_buffer.push_str(c);
                        state = TokenizerState::Id;
                        continue;

                    } else if c.eq("0") {
                        // Digit _or_ start of hexadecimal
                        parsed_buffer.push_str(c);
                        state = TokenizerState::Zero;
                        continue;
                    } else if is_numeric!(c) {
                        // A number; might be decimal
                        Tokenizer::parse_const(&mut value_buffer, c, 10);
                        parsed_buffer.push_str(c);
                        state = TokenizerState::Digit;
                        continue;

                    } else if c == "=" {
                        // An equals sign!
                        return Token::new(TerminalKind::EQUALS, start_pos, pos);

                    } else if c == "+" {
                        // A plus sign!
                        return Token::new(TerminalKind::PLUS, start_pos, pos);
                    } else if c == "-" {
                        // A plus sign!
                        return Token::new(TerminalKind::MINUS, start_pos, pos);
                    } else if c == "*" {
                        // A plus sign!
                        return Token::new(TerminalKind::MULTIPLY, start_pos, pos);
                    } else if c == "/" {
                        // A plus sign!
                        return Token::new(TerminalKind::DIVIDE, start_pos, pos);
                    } else if c == "(" {
                        // A plus sign!
                        return Token::new(TerminalKind::LBRACKET, start_pos, pos);
                    } else if c == ")" {
                        // A plus sign!
                        return Token::new(TerminalKind::RBRACKET, start_pos, pos);

                    } else if is_whitespace!(c) {
                        // A whitespace; simply consume it, then try again
                        continue;
                    } else if c.eq("\0") {
                        // End-of-string; return that token
                        return Token::new(TerminalKind::Eos, start_pos, pos);

                    } else {
                        // Unknown token; consume it
                        parsed_buffer.push_str(c);
                        state = TokenizerState::UnknownToken;
                        continue;
                    }
                }

                TokenizerState::Id => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // Determine what to do
                    if is_id!(c) {
                        // While it's an ID, keep parsing
                        parsed_buffer.push_str(c);
                        continue;
                    } else if is_separator!(c) {
                        self.putc(c, pos);

                        // If we've parsed a keyword, return that instead
                        if parsed_buffer.eq("dec") {
                            return Token::new(TerminalKind::TODEC, start_pos, pos - 1);
                        } else if parsed_buffer.eq("hex") {
                            return Token::new(TerminalKind::TOHEX, start_pos, pos - 1);
                        } else if parsed_buffer.eq("bin") {
                            return Token::new(TerminalKind::TOBIN, start_pos, pos - 1);
                        } else if parsed_buffer.eq("show_vars") {
                            return Token::new(TerminalKind::SHOWVARS, start_pos, pos - 1);
                        } else if parsed_buffer.eq("help") {
                            return Token::new(TerminalKind::HELP, start_pos, pos - 1);
                        } else if parsed_buffer.eq("exit") {
                            return Token::new(TerminalKind::EXIT, start_pos, pos - 1);
                        }

                        // Return it
                        return Token::new(TerminalKind::ID(parsed_buffer), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }



                TokenizerState::Zero => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // Decide if it's hex or decimal
                    if c.eq("d") || c.eq("D") {
                        // It's decimal
                        parsed_buffer.push_str(c);
                        state = TokenizerState::FirstDigit;
                        continue;
                    } else if c.eq("x") || c.eq("X") {
                        // It's hexadecimal
                        parsed_buffer.push_str(c);
                        state = TokenizerState::FirstHexDigit;
                        continue;
                    } else if c.eq("b") || c.eq("B") {
                        // It's binary
                        parsed_buffer.push_str(c);
                        state = TokenizerState::FirstBinaryDigit;
                        continue;
                    } else if is_numeric!(c) {
                        // Also decimal
                        Tokenizer::parse_const(&mut value_buffer, c, 10);
                        parsed_buffer.push_str(c);
                        state = TokenizerState::Digit;
                        continue;
                    } else if is_separator!(c) {
                        // It's a zero
                        self.putc(c, pos);
                        return Token::new(TerminalKind::DEC(0), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::FirstDigit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a digit, keep parsing
                    if is_numeric!(c) {
                        parsed_buffer.push_str(c);
                        Tokenizer::parse_const(&mut value_buffer, c, 10);
                        state = TokenizerState::Digit;
                        continue;
                    } else if is_separator!(c) {
                        // We expected a digit!
                        self.putc(c, pos);
                        return Token::new(TerminalKind::Error(format!("Expected at least one digit after '0d'.")), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::Digit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a digit, keep parsing
                    if is_numeric!(c) {
                        parsed_buffer.push_str(c);
                        Tokenizer::parse_const(&mut value_buffer, c, 10);
                        continue;
                    } else if is_separator!(c) {
                        // Stop parsing and return
                        self.putc(c, pos);
                        return Token::new(TerminalKind::DEC(value_buffer), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::FirstHexDigit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a digit, keep parsing
                    if is_hex!(c) {
                        parsed_buffer.push_str(c);
                        Tokenizer::parse_const(&mut value_buffer, c, 16);
                        state = TokenizerState::HexDigit;
                        continue;
                    } else if is_separator!(c) {
                        // We expected a digit!
                        self.putc(c, pos);
                        return Token::new(TerminalKind::Error(format!("Expected at least one digit after '0x'.")), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::HexDigit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a hexdigit, keep parsing
                    if is_hex!(c) {
                        parsed_buffer.push_str(c);
                        Tokenizer::parse_const(&mut value_buffer, c, 16);
                        continue;
                    } else if is_separator!(c) {
                        // Stop parsing and return
                        self.putc(c, pos);
                        return Token::new(TerminalKind::HEX(value_buffer), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::FirstBinaryDigit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a digit, keep parsing
                    if is_binary!(c) {
                        parsed_buffer.push_str(c);
                        Tokenizer::parse_const(&mut value_buffer, c, 2);
                        state = TokenizerState::BinaryDigit;
                        continue;
                    } else if is_separator!(c) {
                        // We expected a digit!
                        self.putc(c, pos);
                        return Token::new(TerminalKind::Error(format!("Expected at least one digit after '0b'.")), start_pos, pos - 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }

                TokenizerState::BinaryDigit => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // As long as its a binarydigit, keep parsing
                    if is_binary!(c) {
                        Tokenizer::parse_const(&mut value_buffer, c, 2);
                        parsed_buffer.push_str(c);
                        continue;
                    } else if is_separator!(c) {
                        // Stop parsing and return
                        self.putc(c, pos);
                        return Token::new(TerminalKind::BIN(value_buffer), start_pos, pos + 1);
                    }

                    // Unknown token; consume it
                    self.putc(c, pos);
                    state = TokenizerState::UnknownToken;
                    continue;
                }



                TokenizerState::UnknownToken => {
                    // Get the next char
                    let (c, pos) = self.getc();

                    // If it's a whitespace, stop consuming
                    if is_separator!(c) {
                        self.putc(c, pos);
                        return Token::new(TerminalKind::Undefined(parsed_buffer), start_pos, pos - 1);
                    }

                    // Otherwise, keep putting on the buffer
                    parsed_buffer.push_str(c);
                    continue;
                }
            }
        }
    }
}
