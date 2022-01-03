/* SYMBOLS.rs
 *   by Lut99
 *
 * Created:
 *   03 Jan 2022, 10:28:04
 * Last edited:
 *   03 Jan 2022, 12:10:31
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Small module that contains the definitions for symbols, terminals and
 *   non-terminals.
**/


/***** ENUMS *****/
/// Lists all the terminal types registered in the parser.
#[derive(Copy, Clone)]
pub enum TerminalKind {
    /// Meta enum for when no kind is defined
    Undefined,

    /// A decimal value.
    Decimal(u64),
    /// A hexadecimal value.
    Hex(u64),

    /// The to-decimal token
    ToDecimal,
    /// The to-hexadecimal token
    ToHex,

    /// The plus-sign.
    Plus,
    /// The minus-sign.
    Minus,
    /// The multiply-sign.
    Multiply,
    /// The divide-sign.
    Divide,

    /// The left bracket
    LBracket,
    /// The right bracket
    RBracket,
}





/***** TRAITS *****/
/// The Symbol trait is used for all symbols across the parser
pub trait Symbol {
    /// Returns whether or not the Token is terminal.
    fn is_terminal() -> bool;
}



/// The NonTerminal trait is used to define semantics in the parser, i.e., reducable symbols.
pub trait NonTerminal: Symbol {
    
}





/***** STRUCTS *****/
/// A Token actually represents a non-value terminal in the parser.
pub struct Token {
    /// The type of this Token, and thus possibly also carrying a value.
    pub kind : TerminalKind,
    /// The position of this token in the input string.
    pub pos  : usize,
}

impl Token {
    /// Constructor for the Token
    /// 
    /// **Arguments**
    ///  * `kind`: The type of this Token as a TerminalKind.
    ///  * `pos`: The position of this Token in the input string.
    ///
    /// **Returns**  
    /// A newly constructed Token.
    pub fn new(kind: TerminalKind, pos: usize) -> Token {
        return Token {
            kind  : kind,
            pos   : pos,
        };
    }
}

impl Symbol for Token {
    /// Returns whether or not this token is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal() -> bool { true }
}
