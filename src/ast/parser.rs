/* PARSER.rs
 *   by Lut99
 *
 * Created:
 *   04 Jan 2022, 12:00:03
 * Last edited:
 *   04 Jan 2022, 12:33:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   A module containing the offsetcalculator parser.
**/

use crate::ast::symbols::TerminalKind;
use crate::ast::symbols::Symbol;
use crate::ast::symbols::Token;
use crate::ast::tokenizer::Tokenizer;


/***** AST ENUMS *****/
/// Defines all constant types in the AST.
pub enum ConstantKind {
    /// Meta enum for no type defined.
    Undefined,

    /// The decimal constant type
    Decimal,
    /// The hexadecimal constant type
    Hexadecimal,
    /// The binary constant type
    Binary,
}

/// Defines all mono operators in the AST.
pub enum MonoOperator {
    /// Meta enum for no operator defined.
    Undefined,

    /// The to-dec operator
    Dec,
    /// The to-hex operator
    Hex,
    /// The to-bin operator
    Bin,
}

/// Defines all binary operators in the AST.
pub enum BinaryOperator {
    /// Meta enum for no operator defined.
    Undefined,

    /// The plus-operator
    Plus,
    /// The minus-operator
    Minus,
    /// The multiply-operator
    Multiply,
    /// The divide-operator
    Divide,
}





/***** AST TRAITS *****/
/// Defines an expression
pub trait Expression {
    
}





/***** AST NODES *****/
/// The ASTRoot node, which forms the root of the Abstract Syntax Tree.
pub struct ASTRoot {
    /// Since each AST only is one expression in our calculator, implement it as such
    pub expr : Box<dyn Expression>,
}
impl Symbol for ASTRoot {
    /// Returns whether or not this node is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { false }
}



/// The ASTBinOp node, which defines an operation with two operants in the Abstract Syntax Tree.
pub struct ASTBinOp {
    /// The operand of the BinOp
    pub operator : BinaryOperator,
    /// The lefthandside expression of the BinOp
    pub left     : Box<dyn Expression>,
    /// The righthand side expression of the BinOp
    pub right    : Box<dyn Expression>,
}
impl Symbol for ASTBinOp {
    /// Returns whether or not this node is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { false }
}
impl Expression for ASTBinOp {}

/// The ASTMonOp node, which defines an operator with only one operant in the Abstract Syntax Tree.
pub struct ASTMonOp {
    /// The operator of the MonOp
    pub operator : MonoOperator,
    /// The expression of the MonOp
    pub expr     : Box<dyn Expression>,
}
impl Symbol for ASTMonOp {
    /// Returns whether or not this node is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { false }
}
impl Expression for ASTMonOp {}

/// The ASTConst node, which defines a simply constant in the Abstract Syntax Tree.
pub struct ASTConst {
    /// The type of the constant
    pub kind  : ConstantKind,
    /// The value of the constant
    pub value : u64,
}
impl Symbol for ASTConst {
    /// Returns whether or not this node is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { false }
}
impl Expression for ASTConst {}





/***** PARSING FUNCTIONS *****/
/// Tries to reduce the given stack using the parser rules.
/// 
/// **Arguments**
///  * `stack`: The stack to reduce.
/// 
/// **Returns**  
/// The applied rule as a string, or 'an empty one if no rule is applied. Errors are automatically printed to stderr, and if they occur the rule 'error' is used.
fn reduce(stack: &mut Vec<Box<dyn Symbol>>) -> String {
    
}





/***** LIBRARY FUNCTIONS *****/
/// Parses the given string into an AST of the math it represents.
pub fn parse<'a>(input: &'a str) -> ASTRoot {
    // Prepare the tokenizer to use for input
    let mut tokenizer = Tokenizer::new(input);

    // Prepare the stack
    let mut stack: Vec<Box<dyn Symbol>> = Vec::new();

    // Start parsing
    loop {
        // Try to reduce the current stack
        let mut rule = reduce(&mut stack);

        // If we did anything, retry; otherwise, reduce to get more
        if rule.len() > 0 {
            continue;
        } else {
            // Get the next token
            let token = tokenizer.get();
            match token.kind {
                TerminalKind::Eos => {
                    // No more tokens; we're done parsing
                    break;
                }
                TerminalKind::Undefined((token, pos)) => {
                    // Encountered an unknown token; try to get more
                    eprintln!("{}: Encountered unknown token '{}'.", )
                    continue;
                }
            }
        }
    }

    // Done
}
