/* SYMBOLS.rs
 *   by Lut99
 *
 * Created:
 *   03 Jan 2022, 10:28:04
 * Last edited:
 *   07 Jan 2022, 12:14:52
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Small module that contains the definitions for symbols, terminals and
 *   non-terminals.
**/


/***** ENUMS *****/
/// Lists all the terminal types registered in the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalKind {
    /// Meta enum for when no kind is defined
    Undefined(String),
    /// Meta enum for when an error occurred
    Error(String),
    /// Meta enum for when the string has ended
    Eos,

    /// An identifier.
    ID(String),

    /// A decimal value.
    DEC(u64),
    /// A hexadecimal value.
    HEX(u64),
    /// A binary value.
    BIN(u64),

    /// The to-decimal token
    TODEC,
    /// The to-hexadecimal token
    TOHEX,
    /// The to-binary token
    TOBIN,

    // The equals-sign
    EQUALS,

    /// The plus-sign.
    PLUS,
    /// The minus-sign.
    MINUS,
    /// The multiply-sign.
    MULTIPLY,
    /// The divide-sign.
    DIVIDE,

    /// The left bracket
    LBRACKET,
    /// The right bracket
    RBRACKET,

    /// The Del token
    DEL,
    /// The DelAll token
    DELALL,
    /// The ShowVars token
    SHOWVARS,
    /// The ClearHist token
    CLEARHIST,
    /// The Help token
    HELP,
    /// The quit token
    EXIT,
}





/***** TRAITS *****/
/// Trait that allows the Symbol to be casted to an Any
pub trait SymbolToAny: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> SymbolToAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The Symbol trait is used for all symbols across the parser
pub trait Symbol: std::fmt::Debug + SymbolToAny {
    /// Returns whether or not the Token is terminal.
    fn is_terminal(&self) -> bool;

    /// Returns the positions of the symbol in the original string.
    fn pos(&self) -> (usize, usize);
    /// Updates the position of the symbol.
    /// 
    /// **Arguments**
    ///  * `pos1`: The new pos1.
    ///  * `pos2`: The new pos2.
    fn set_pos(&mut self, pos1: usize, pos2: usize);
}





/***** STRUCTS *****/
/// A Token actually represents a non-value terminal in the parser.
pub struct Token {
    /// The type of this Token, and thus possibly also carrying a value.
    pub kind : TerminalKind,
    /// The start position of this token in the input string.
    pub pos1 : usize,
    /// The end position (inclusive) of this token in the input string.
    pub pos2 : usize,
}

impl Token {
    /// Constructor for the Token
    /// 
    /// **Arguments**
    ///  * `kind`: The type of this Token as a TerminalKind.
    ///  * `pos1`: The start position of this Token in the input string.
    ///  * `pos2`: The end position (inclusive) of this Token in the input string.
    ///
    /// **Returns**  
    /// A newly constructed Token.
    pub fn new(kind: TerminalKind, pos1: usize, pos2: usize) -> Token {
        return Token {
            kind  : kind,
            pos1  : pos1,
            pos2  : pos2,
        };
    }
}

impl std::fmt::Debug for Token {
    /// Write a debug counterpart for this Token
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{}-{}]", self.kind, self.pos1, self.pos2)
    }
}

impl Symbol for Token {
    /// Returns whether or not this token is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { true }

    /// Returns the positions of the symbol in the original string.
    #[inline]
    fn pos(&self) -> (usize, usize) { (self.pos1, self.pos2) }
    /// Updates the position of the symbol.
    /// 
    /// **Arguments**
    ///  * `pos1`: The new pos1.
    ///  * `pos2`: The new pos2.
    #[inline]
    fn set_pos(&mut self, pos1: usize, pos2: usize) {
        self.pos1 = pos1;
        self.pos2 = pos2;
    }
}
