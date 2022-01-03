/* TOKENIZER.rs
 *   by Lut99
 *
 * Created:
 *   03 Jan 2022, 10:27:03
 * Last edited:
 *   03 Jan 2022, 12:09:59
 * Auto updated?
 *   Yes
 *
 * Description:
 *   A Rust module that implements a tokenizer for the offsetcalculator.
**/

#[path = "./symbols.rs"]
mod symbols;

use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::Graphemes;


/***** HELPER MACROS *****/
/// Checks if the given 'char' equals a whitespace.
macro_rules! is_whitespace {
    ($c:expr) => {
        ($c.eq(" ") || $c.eq("\t") || $c.eq("\r") || $c.eq("\n"))
    };
}

/// Checks if the given 'char' equals a digit.
macro_rules! is_numeric {
    ($c:expr) => {
        ($c.eq("0") || $c.eq("1") || $c.eq("2") || $c.eq("3") || $c.eq("4") ||
         $c.eq("5") || $c.eq("6") || $c.eq("7") || $c.eq("8") || $c.eq("9"))
    };
}





/***** HELPER ENUMS *****/
enum TokenizerState {
    /// The start state
    Start,
    /// The state when we find a 'd' or 'D'
    D,
    /// We found a '0'
    Zero,
    /// The state for when we find a digit
    Digit,
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
    fn getc(&mut self) -> (&str, usize) {
        // If there's something in the buffer, return that instead
        if self.temp.len() > 0 {
            return self.temp.remove(self.temp.len() - 1);
        }

        // Otherwise, get it with its pos
        let c   = self.input.next();
        let pos = self.pos;

        // If we failed, return the end-of-string character
        if c == None { return ("\0", self.max_pos); }

        // Otherwise, increment the pos and return
        self.pos += 1;
        return (c.unwrap(), pos);
    }



    /// Tries to get the next token from the input stream.
    /// 
    /// **Returns**  
    /// The token wrapped in Ok() if true, or an Err() explaining why we couldn't get it.
    pub fn get(&mut self) -> symbols::Token {
        // Loop to emulate the destinaton
        let mut value_buffer: u64 = 0;
        let mut target = TokenizerState::Start;
        loop {
            // Match the target
            match target {
                TokenizerState::Start => {
                    // Get the next character
                    let (c, pos) = self.getc();

                    // Split on the character we see
                    if c.eq("0") {
                        // Digit _or_ start of hexadecimal
                        target = TokenizerState::Zero;
                        continue;
                    } if is_numeric!(c) {
                        // A number; might be decimal
                        value_buffer = c.chars().next().unwrap().to_digit(10).unwrap() as u64;
                        target = TokenizerState::Digit;
                        continue;
                    } else if c == "d" || c == "D" {
                        // Start of decimal?
                        target = TokenizerState::D;
                        continue;
                    } else if c == "+" {
                        // A plus sign!
                        return symbols::Token::new(symbols::TerminalKind::Plus, pos);
                    } else if is_whitespace!(c) {
                        // A whitespace; try again
                        continue;
                    } else if c.eq("\0") {
                        // End-of-string; 
                    } else {
                        // Unknown character
                        
                    }
                }

                TokenizerState::Zero => {
                    // Get the next character
                    let c = self.input.next().unwrap_or("\0");
                    if c != "\0" { self.pos += 1; }

                    
                }

                _ => {
                    // Undefined case
                    panic!("Reached unreachable part of token jumptable.");
                }
            }
        }
    }
}
