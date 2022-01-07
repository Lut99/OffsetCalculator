/* PARSER.rs
 *   by Lut99
 *
 * Created:
 *   04 Jan 2022, 12:00:03
 * Last edited:
 *   07 Jan 2022, 12:16:37
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
/// Defines the possible states for the parser, to implement a jump table.
#[allow(non_camel_case_types)]
enum ParserState {
    /// The start state
    Start,

    /// We've seen one Id
    Id,

    /// We've seen one Expr
    Expr,
    /// We've seen one Expr and an equals
    Expr_Equals,

    /// We've seen one Term
    Term,
    /// We've seen a Term followed by a plus OR a minus
    Term_PlusORMinus,

    /// We've seen one Factor
    Factor,
    /// We've seen a Factor followed by a multiplication OR a division
    Factor_MultiplyORDivide,

    /// We've seen the right bracket
    RBracket,
    /// We've seen the right bracket followed by an expression
    RBracket_Expr,
}

/// Defines all constant types in the AST.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueKind {
    /// Meta value for when no kind is defined yet
    Undefined,

    /// The decimal constant type
    Decimal,
    /// The hexadecimal constant type
    Hexadecimal,
    /// The binary constant type
    Binary,
}

impl From<TerminalKind> for ValueKind {
    fn from(val: TerminalKind) -> Self {
        match val {
            TerminalKind::DEC(_) | TerminalKind::TODEC => { ValueKind::Decimal }
            TerminalKind::HEX(_) | TerminalKind::TOHEX => { ValueKind::Hexadecimal }
            TerminalKind::BIN(_) | TerminalKind::TOBIN => { ValueKind::Binary }
            _                                          => { ValueKind::Undefined }
        }
    }
}
impl std::str::FromStr for ValueKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to read
        if s.to_lowercase().eq("decimal") {
            return Ok(ValueKind::Decimal);
        } else if s.to_lowercase().eq("hexadecimal") {
            return Ok(ValueKind::Hexadecimal);
        } else if s.to_lowercase().eq("binary") {
            return Ok(ValueKind::Binary);
        } else if s.to_lowercase().eq("undefined") {
            return Ok(ValueKind::Undefined);
        }

        // Otherwise, return err
        return Err(());
    }
}
impl From<ValueKind> for String {
    fn from(val: ValueKind) -> Self {
        // Return the value
        return format!("{:?}", val);
    }
}

/// Defines all binary operators in the AST.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LowBinaryOperator {
    /// Meta value for when no operator is defined
    Undefined,

    /// The plus-operator
    Plus,
    /// The minus-operator
    Minus,
}

impl From<TerminalKind> for LowBinaryOperator {
    fn from(val: TerminalKind) -> Self {
        match val {
            TerminalKind::PLUS     => { LowBinaryOperator::Plus }
            TerminalKind::MINUS    => { LowBinaryOperator::Minus }
            _                      => { LowBinaryOperator::Undefined }
        }
    }
}

/// Defines the high-precedence binary operators in the AST.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HighBinaryOperator {
    /// Meta value for when no operator is defined
    Undefined,

    /// The multiply-operator
    Multiply,
    /// The divide-operator
    Divide,
}

impl From<TerminalKind> for HighBinaryOperator {
    fn from(val: TerminalKind) -> Self {
        match val {
            TerminalKind::MULTIPLY => { HighBinaryOperator::Multiply }
            TerminalKind::DIVIDE   => { HighBinaryOperator::Divide }
            _                      => { HighBinaryOperator::Undefined }
        }
    }
}





/***** AST NODES *****/
/// Enum that defines the AST nodes
#[derive(Clone)]
pub enum ASTNode {
    /// A simple node that can be used as a placeholder
    Undefined,

    /// Defines a runnable command that is NOT an expression.
    Cmd { cmd: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines the 'del' node.
    Del { identifier: String, pos1: usize, pos2: usize },
    /// Defines the 'delall' node.
    DelAll { pos1: usize, pos2: usize },
    /// Defines the 'show_vars' node.
    ShowVars { pos1: usize, pos2: usize },
    /// Defines the 'clear_hist' node.
    ClearHist { pos1: usize, pos2: usize },
    /// Defines the 'help' node.
    Help { pos1: usize, pos2: usize },
    /// Defines the 'exit' node.
    Exit { pos1: usize, pos2: usize },

    /// Defines an expression in the AST
    Expr { override_kind: bool, kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a strong expression in the AST, which is an expression but for operators of slightly higher precedence
    StrongExpr { kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a term in the AST, which is an expression but for operators of higher precedence
    Term { kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a factor in the AST, which is an expression but for operators of more higher precedence
    Factor { kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a smallfactor in the AST, which is an expression but for operators of the highest precedence
    SmallFactor { kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },

    /// Defines an assignment of an identifier
    Assign { override_kind: bool, kind: ValueKind, identifier: String, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a binary operator for lower precedence operators in the AST
    BinOpLow { override_kind: bool, kind: ValueKind, operator: LowBinaryOperator, left: Box<ASTNode>, right: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a binary operator for higher precedence operators in the AST
    BinOpHigh { override_kind: bool, kind: ValueKind, operator: HighBinaryOperator, left: Box<ASTNode>, right: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a monary operator in the AST
    MonOp { kind: ValueKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    
    /// Defines an identifier in the AST
    Id { identifier: String, pos1: usize, pos2: usize },
    /// Defines a constant in the AST
    Const { kind: ValueKind, value: u64, pos1: usize, pos2: usize },
}

impl std::fmt::Debug for ASTNode {
    /// Write a debug counterpart for this Token
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Undefined => { write!(f, "Undefined") }

            ASTNode::Cmd{ cmd, pos1: _, pos2: _ }            => { write!(f, "Cmd({:?})", cmd) }
            ASTNode::Del{ ref identifier, pos1: _, pos2: _ } => { write!(f, "Del({})", identifier) }
            ASTNode::DelAll{ pos1: _, pos2: _ }              => { write!(f, "DelAll") }
            ASTNode::ShowVars{ pos1: _, pos2: _ }            => { write!(f, "ShowVars") }
            ASTNode::ClearHist{ pos1: _, pos2: _ }            => { write!(f, "ClearHist") }
            ASTNode::Help{ pos1: _, pos2: _ }                => { write!(f, "Help") }
            ASTNode::Exit{ pos1: _, pos2: _ }                => { write!(f, "Exit") }
            
            ASTNode::Expr{ override_kind: _, kind, expr, pos1: _, pos2: _ } => { write!(f, "Expr<{:?}>({:?})", kind, expr) }
            ASTNode::StrongExpr{ kind, expr, pos1: _, pos2: _ }             => { write!(f, "StrongExpr<{:?}>({:?})", kind, expr) }
            ASTNode::Term{ kind, expr, pos1: _, pos2: _ }                   => { write!(f, "Term<{:?}>({:?})", kind, expr) }
            ASTNode::Factor{ kind, expr, pos1: _, pos2: _ }                 => { write!(f, "Factor<{:?}>({:?})", kind, expr) }
            ASTNode::SmallFactor{ kind, expr, pos1: _, pos2: _ }            => { write!(f, "SmallFactor<{:?}>({:?})", kind, expr) }

            ASTNode::Assign{ override_kind, kind, ref identifier, expr, pos1: _, pos2: _ }     => { write!(f, "Assign<{} {:?}>({} = {:?})", override_kind, kind, identifier, expr) }
            ASTNode::BinOpLow{ override_kind, kind, operator, left, right, pos1: _, pos2: _ }  => { write!(f, "BinOpL<{} {:?}>({:?} {:?} {:?})", override_kind, kind, left, operator, right) }
            ASTNode::BinOpHigh{ override_kind, kind, operator, left, right, pos1: _, pos2: _ } => { write!(f, "BinOpH<{} {:?}>({:?} {:?} {:?})", override_kind, kind, left, operator, right) }
            ASTNode::MonOp{ kind, expr, pos1: _, pos2: _ }                                     => { write!(f, "MonOp<{:?}>({:?})", kind, expr) }

            ASTNode::Id{ ref identifier, pos1: _, pos2: _ } => {write!(f, "Id({})", identifier) }
            ASTNode::Const{ kind, value, pos1: _, pos2: _ } => { write!(f, "{}<{:?}>", value, kind) }
        }
    }
}

impl Symbol for ASTNode {
    /// Returns whether or not this node is terminal.
    /// 
    /// **Returns**  
    /// True is a terminal, false otherwise.
    #[inline]
    fn is_terminal(&self) -> bool { false }

    /// Returns the positions of the symbol in the original string.
    #[inline]
    fn pos(&self) -> (usize, usize) {
        // Switch first
        match self {
            ASTNode::Undefined => { (usize::MAX, usize::MAX) }

            ASTNode::Cmd{ cmd: _, pos1, pos2 }        => { (*pos1, *pos2) }
            ASTNode::Del{ identifier: _, pos1, pos2 } => { (*pos1, *pos2) }
            ASTNode::DelAll{ pos1, pos2 }             => { (*pos1, *pos2) }
            ASTNode::ShowVars{ pos1, pos2 }           => { (*pos1, *pos2) }
            ASTNode::ClearHist{ pos1, pos2 }          => { (*pos1, *pos2) }
            ASTNode::Help{ pos1, pos2 }               => { (*pos1, *pos2) }
            ASTNode::Exit{ pos1, pos2 }               => { (*pos1, *pos2) }
            
            ASTNode::Expr{ override_kind: _, kind: _, expr: _, pos1, pos2 } => { (*pos1, *pos2) }
            ASTNode::StrongExpr{ kind: _, expr: _, pos1, pos2 }             => { (*pos1, *pos2) }
            ASTNode::Term{ kind: _, expr: _, pos1, pos2 }                   => { (*pos1, *pos2) }
            ASTNode::Factor{ kind: _, expr: _, pos1, pos2 }                 => { (*pos1, *pos2) }
            ASTNode::SmallFactor{ kind: _, expr: _, pos1, pos2 }            => { (*pos1, *pos2) }

            ASTNode::Assign{ override_kind: _, kind: _, identifier: _, expr: _, pos1, pos2 }            => { (*pos1, *pos2) }
            ASTNode::BinOpLow{ override_kind: _, kind: _, operator: _, left: _, right: _, pos1, pos2 }  => { (*pos1, *pos2) }
            ASTNode::BinOpHigh{ override_kind: _, kind: _, operator: _, left: _, right: _, pos1, pos2 } => { (*pos1, *pos2) }
            ASTNode::MonOp{ kind: _, expr: _, pos1, pos2 }                                              => { (*pos1, *pos2) }

            ASTNode::Id{ identifier: _, pos1, pos2 }        => { (*pos1, *pos2) }
            ASTNode::Const{ kind: _, value: _, pos1, pos2 } => { (*pos1, *pos2) }
        }
    }
    /// Updates the position of the symbol.
    /// 
    /// **Arguments**
    ///  * `pos1`: The new pos1.
    ///  * `pos2`: The new pos2.
    #[inline]
    fn set_pos(&mut self, new_pos1: usize, new_pos2: usize) {
        // Switch first
        match self {
            ASTNode::Undefined => {}

            ASTNode::Cmd{ cmd: _, ref mut pos1, ref mut pos2 }        => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Del{ identifier: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::DelAll{ ref mut pos1, ref mut pos2 }             => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::ShowVars{ ref mut pos1, ref mut pos2 }           => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::ClearHist{ ref mut pos1, ref mut pos2 }          => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Help{ ref mut pos1, ref mut pos2 }               => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Exit{ ref mut pos1, ref mut pos2 }               => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::Expr{ override_kind: _, kind: _, expr: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::StrongExpr{ kind: _, expr: _, ref mut pos1, ref mut pos2 }             => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Term{ kind: _, expr: _, ref mut pos1, ref mut pos2 }                   => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Factor{ kind: _, expr: _, ref mut pos1, ref mut pos2 }                 => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::SmallFactor{ kind: _, expr: _, ref mut pos1, ref mut pos2 }            => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::Assign{ override_kind: _, kind: _, identifier: _, expr: _, ref mut pos1, ref mut pos2 }            => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::BinOpLow{ override_kind: _, kind: _, operator: _, left: _, right: _, ref mut pos1, ref mut pos2 }  => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::BinOpHigh{ override_kind: _, kind: _, operator: _, left: _, right: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::MonOp{ kind: _, expr: _, ref mut pos1, ref mut pos2 }                                              => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::Id{ identifier: _, ref mut pos1, ref mut pos2 }         => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Const{ kind: _,  value: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
        }
    }
}





/***** PARSING FUNCTIONS *****/
/// Tries to reduce the given stack using the parser rules.
/// 
/// **Arguments**
///  * `stack`: The stack to reduce.
/// 
/// **Returns**  
/// The applied rule as a string, or 'an empty one if no rule is applied. Errors are automatically printed to stderr, and if they occur the rule 'error' is used.
fn reduce(input: &str, stack: &mut Vec<Box<dyn Symbol>>, lookahead: &Token) -> String {
    // Define some temporary variables
    let mut last_node: &ASTNode = &ASTNode::Undefined;
    let mut last_token: &Token = &Token::new(TerminalKind::Undefined(String::new()), usize::MAX, usize::MAX);

    // Do the jumptable
    let mut i: usize = stack.len();
    let mut state = ParserState::Start;
    loop {
        // Match the state
        match state {
            ParserState::Start => {
                // Get the first symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Downcast
                    let token = s.as_any().downcast_ref::<Token>().unwrap();

                    // Switch on its kind
                    match &token.kind {
                        TerminalKind::DELALL => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::DelAll {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("delall");
                        }
                        TerminalKind::SHOWVARS => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::ShowVars {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("show_vars");
                        }
                        TerminalKind::CLEARHIST => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::ClearHist {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("clear_hist");
                        }
                        TerminalKind::HELP => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::Help {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("help");
                        }
                        TerminalKind::EXIT => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::Exit {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("exit");
                        }

                        TerminalKind::RBRACKET => {
                            // Start consuming the next expression
                            last_token = token;
                            state = ParserState::RBracket;
                            continue;
                        }

                        TerminalKind::ID(_) => {
                            // Go to the id state
                            last_token = token;
                            state = ParserState::Id;
                            continue;
                        }
                        TerminalKind::DEC(val) => {
                            // Replace on the stack with a const
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ValueKind::Decimal,
                                value: *val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("const_dec");
                        }
                        TerminalKind::HEX(val) => {
                            // Replace on the stack with a const
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ValueKind::Hexadecimal,
                                value: *val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("const_hex");
                        }
                        TerminalKind::BIN(val) => {
                            // Replace on the stack with a const
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ValueKind::Binary,
                                value: *val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("const_bin");
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on its kind
                    match node {
                        ASTNode::Del{ identifier: _, pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_del");
                        }
                        ASTNode::DelAll{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_delall");
                        }
                        ASTNode::ShowVars{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_showvars");
                        }
                        ASTNode::ClearHist{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_clearhist");
                        }
                        ASTNode::Help{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_help");
                        }
                        ASTNode::Exit{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd_exit");
                        }

                        ASTNode::Assign{ override_kind: _, kind: _, identifier: _, expr: _, pos1, pos2 } => {
                            // Cast to an expression
                            stack[i] = Box::new(ASTNode::Expr{
                                override_kind: false,
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("expr_assign");
                        }

                        ASTNode::BinOpLow{ override_kind: _, kind: _, operator: _, left: _, right: _, pos1, pos2 } => {
                            // Cast to a strongexpression
                            stack[i] = Box::new(ASTNode::StrongExpr{
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("strongexpr_binoplow");
                        }

                        ASTNode::BinOpHigh{ override_kind: _, kind: _, operator: _, left: _, right: _, pos1, pos2 } => {
                            // Cast to an expression
                            stack[i] = Box::new(ASTNode::Term{
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("term_binophigh");
                        }

                        ASTNode::MonOp{ kind: _, expr: _, pos1, pos2 } |
                        ASTNode::SmallFactor{ kind: _, expr: _, pos1, pos2 } => {
                            // Cast to a factor
                            stack[i] = Box::new(ASTNode::Factor{
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("factor_monop_smallfactor");
                        }

                        ASTNode::Id{ identifier: _, pos1, pos2 } => {
                            // Cast to a smallfactor
                            stack[i] = Box::new(ASTNode::SmallFactor{
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("smallfactor_id");
                        }
                        ASTNode::Const{ kind: _, value: _, pos1, pos2 } => {
                            // Cast to a smallfactor
                            stack[i] = Box::new(ASTNode::SmallFactor{
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("smallfactor_const");
                        }

                        ASTNode::Term{ kind: _, expr: _, pos1: _, pos2: _ } => {
                            // Go to the start of possibly a binoplow
                            last_node = node;
                            state = ParserState::Term;
                            continue;
                        }

                        ASTNode::Factor{ kind: _, expr: _, pos1: _, pos2: _ } => {
                            // Go to the start of possibly a binoplow
                            last_node = node;
                            state = ParserState::Factor;
                            continue;
                        }

                        ASTNode::StrongExpr{ kind: _, expr: _, pos1: _, pos2: _ } => {
                            // Make sure there is no plus or minus on the lookahead
                            match lookahead.kind {
                                TerminalKind::PLUS |
                                TerminalKind::MINUS => {
                                    // Skip replacing
                                    return String::new();
                                }

                                _ => {}
                            }

                            // Otherwise, cast to an expression
                            stack[i] = Box::new(ASTNode::Expr{
                                override_kind: false,
                                kind: ValueKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: node.pos().0, pos2: node.pos().1
                            });
                            return String::from("expr_strongexpr");
                        }

                        ASTNode::Expr{ override_kind: _, kind: _, expr: _, pos1: _, pos2: _ } => {
                            // Go to the start of possibly a binoplow
                            last_node = node;
                            state = ParserState::Expr;
                            continue;
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }
                }
            }



            ParserState::Id => {
                // Get the id from the previous token
                let id;
                match &last_token.kind {
                    TerminalKind::ID(tid) => {
                        id = tid;
                    }
                    _ => { panic!("We've seen an ID, but the last token wasn't an ID; this should never happen!"); }
                }

                // Get the next symbol
                if i > 0 {
                    i -= 1;
                    let s = &stack[i];

                    // Switch on terminal VS nonterminal
                    if s.is_terminal() {
                        // Downcast
                        let token = s.as_any().downcast_ref::<Token>().unwrap();

                        // Switch on its kind
                        match token.kind {
                            TerminalKind::DEL => {
                                // Join them in a Del node.
                                let ns = Box::new(ASTNode::Del{
                                    identifier: id.clone(),
                                    pos1: token.pos1, pos2: token.pos2
                                });

                                // Replace on the stack
                                stack.remove(stack.len() - 1);
                                stack[i] = ns;

                                // Done
                                return String::from("del");
                            }
                            _ => {}
                        }
                    }
                }

                // Do not do it if there's an EQUALS coming up
                match lookahead.kind {
                    TerminalKind::EQUALS => {
                        // Skip replacing
                        return String::new();
                    }
                    _ => {}
                }

                // Replace the ID we parsed on the stack with an id
                let stack_len = stack.len();
                stack[stack_len - 1] = Box::new(ASTNode::Id{
                    identifier: id.clone(),
                    pos1: last_token.pos1, pos2: last_token.pos2
                });
                return String::from("id");
            }



            ParserState::Expr => {
                // Get the next symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Downcast
                    let token = s.as_any().downcast_ref::<Token>().unwrap();

                    // Switch on its kind
                    match token.kind {
                        // Could be a monary op
                        TerminalKind::TODEC |
                        TerminalKind::TOHEX |
                        TerminalKind::TOBIN => {
                            // It is; generate the new symbol
                            let ns = Box::new(ASTNode::MonOp{
                                kind: ValueKind::from(token.kind.clone()),
                                expr: Box::new(last_node.clone()),
                                pos1: token.pos1, pos2: last_node.pos().1 }
                            );
                            
                            // Insert it in the stack instead of the top two ones
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Done
                            return String::from("monop");
                        }

                        // Could be an assign
                        TerminalKind::EQUALS => {
                            // It is; go to the equals state
                            last_token = token;
                            state = ParserState::Expr_Equals;
                            continue;
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }

                } else {
                    // Ignore all nonterminals too
                    return String::new();
                }
            }

            ParserState::Expr_Equals => {
                // Get the next symbol
                if i > 0 {
                    i -= 1;
                    let s = &stack[i];

                    // Switch on terminal VS nonterminal
                    if s.is_terminal() {
                        // Downcast
                        let token = s.as_any().downcast_ref::<Token>().unwrap();

                        // Switch on its kind
                        match token.kind {
                            // Could be an assign
                            TerminalKind::ID(ref id) => {
                                // Create the new node
                                let ns = Box::new(ASTNode::Assign{
                                    override_kind: false,
                                    kind: ValueKind::Undefined,
                                    identifier: id.clone(),
                                    expr: Box::new(last_node.clone()),
                                    pos1: token.pos1, pos2: token.pos2
                                });

                                // Replace it on the stack
                                stack.remove(stack.len() - 1);
                                stack.remove(stack.len() - 1);
                                stack[i] = ns;

                                // Done
                                return String::from("assign");
                            }

                            // Used a keyword
                            TerminalKind::TODEC |
                            TerminalKind::TOHEX |
                            TerminalKind::TOBIN |
                            TerminalKind::DEL |
                            TerminalKind::DELALL |
                            TerminalKind::SHOWVARS |
                            TerminalKind::HELP |
                            TerminalKind::EXIT => {
                                // Tell the user what happened
                                eprintln!("{}: Expected identifier, got keyword {}.", token.pos().0, &input[token.pos1 - 1..token.pos2]);
                                stack.remove(stack.len() - 1);
                                stack.remove(stack.len() - 1);
                                return String::from("error");
                            }

                            _ => {}
                        }

                    }
                }

                // Not what we expected!
                eprintln!("{}: Missing identifier before assign.", last_token.pos().0);
                stack.remove(stack.len() - 1);
                stack.remove(stack.len() - 1);
                return String::from("error");
            }



            ParserState::Term => {
                // If the lookahead is a multiply or a divide, let the term be
                match lookahead.kind {
                    TerminalKind::MULTIPLY |
                    TerminalKind::DIVIDE => {
                        // Skip replacing
                        return String::new();
                    }

                    _ => {}
                }

                // Get the next symbol
                if i > 0 {
                    i -= 1;
                    let s = &stack[i];

                    // Switch on terminal VS nonterminal
                    if s.is_terminal() {
                        // Downcast
                        let token = s.as_any().downcast_ref::<Token>().unwrap();

                        // Switch on its kind
                        match token.kind {
                            TerminalKind::PLUS |
                            TerminalKind::MINUS => {
                                // Go to the last step of the binoplow
                                last_token = token;
                                state = ParserState::Term_PlusORMinus;
                                continue;
                            }

                            _ => {}
                        }
                    }
                }

                // Replace the original term by an expression
                let ns = Box::new(ASTNode::StrongExpr{
                    kind: ValueKind::Undefined,
                    expr: Box::new(last_node.clone()),
                    pos1: last_node.pos().0, pos2: last_node.pos().1
                });

                // Replace with the top one on the stack
                let i2 = stack.len() - 1;
                stack[i2] = ns;

                // Done
                return String::from("strongexpr_term");
            }

            ParserState::Term_PlusORMinus => {
                // Get the next symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Get the operator
                let op = LowBinaryOperator::from(last_token.kind.clone());

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Downcast
                    let token = s.as_any().downcast_ref::<Token>().unwrap();

                    // Show that this isn't what we mean
                    eprintln!("{}: Missing value before {}.", token.pos1, if op == LowBinaryOperator::Plus { "addition" } else { "subtraction" });
                    stack.remove(stack.len() - 1);
                    stack.remove(stack.len() - 1);
                    return String::from("error");

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on the type
                    match node {
                        ASTNode::StrongExpr{ kind: _, expr: _, pos1, pos2 } => {
                            // Construct the binoplow!
                            let ns = Box::new(ASTNode::BinOpLow{
                                override_kind: false,
                                kind: ValueKind::Undefined,
                                operator: op,
                                left: Box::new(node.clone()),
                                right: Box::new(last_node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });

                            // Replace on the stack
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Return!
                            return String::from("binoplow");
                        }
                        
                        // For the rest, throw an error too
                        _ => {
                            eprintln!("{}: Incompatible symbol '{}' before {}.", node.pos().0, &input[node.pos().0 - 1..node.pos().1], if op == LowBinaryOperator::Plus { "addition" } else { "subtraction" });
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            return String::from("error");
                        }
                    }
                }
            }



            ParserState::Factor => {
                // Get the next symbol
                if i > 0 {
                    i -= 1;
                    let s = &stack[i];

                    // Switch on terminal VS nonterminal
                    if s.is_terminal() {
                        // Downcast
                        let token = s.as_any().downcast_ref::<Token>().unwrap();

                        // Switch on its kind
                        match token.kind {
                            TerminalKind::MULTIPLY |
                            TerminalKind::DIVIDE => {
                                // Go to the last step of the binophigh
                                last_token = token;
                                state = ParserState::Factor_MultiplyORDivide;
                                continue;
                            }

                            _ => {}
                        }
                    }
                }

                // Replace the original term by a term
                let ns = Box::new(ASTNode::Term{
                    kind: ValueKind::Undefined,
                    expr: Box::new(last_node.clone()),
                    pos1: last_node.pos().0, pos2: last_node.pos().1
                });

                // Replace with the top one on the stack
                let i2 = stack.len() - 1;
                stack[i2] = ns;

                // Done
                return String::from("term_factor");
            }

            ParserState::Factor_MultiplyORDivide => {
                // Get the next symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Get the operator
                let op = HighBinaryOperator::from(last_token.kind.clone());

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Downcast
                    let token = s.as_any().downcast_ref::<Token>().unwrap();

                    // Show that this isn't what we mean
                    eprintln!("{}: Missing value before {}.", token.pos1, if op == HighBinaryOperator::Multiply { "multiplication" } else { "division" });
                    stack.remove(stack.len() - 1);
                    stack.remove(stack.len() - 1);
                    return String::from("error");

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on the type
                    match node {
                        ASTNode::Term{ kind: _, expr: _, pos1, pos2 } => {
                            // Construct the binophigh!
                            let ns = Box::new(ASTNode::BinOpHigh{
                                override_kind: false,
                                kind: ValueKind::Undefined,
                                operator: op,
                                left: Box::new(node.clone()),
                                right: Box::new(last_node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });

                            // Replace on the stack
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Return!
                            return String::from("binophigh");
                        }
                        
                        // For the rest, throw an error too
                        _ => {
                            eprintln!("{}: Incompatible symbol '{}' before {}.", node.pos().0, &input[node.pos().0 - 1..node.pos().1], if op == HighBinaryOperator::Multiply { "multiplication" } else { "division" });
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            return String::from("error");
                        }
                    }
                }
            }



            ParserState::RBracket => {
                // Get the first symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Simply ignore; any bracket errors are treated during the post-analysis
                    return String::new();

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on its kind
                    match node {
                        ASTNode::Expr{ override_kind: _, kind:_, expr: _, pos1: _, pos2: _ } => {
                            // Store it, and keep parsing!
                            last_node = node;
                            state = ParserState::RBracket_Expr;
                            continue;
                        }

                        // The rest is probably a malformed node
                        _ => {
                            // Simply ignore; any bracket errors are treated during the post-analysis
                            return String::new();
                        }
                    }
                }
            }

            ParserState::RBracket_Expr => {
                // Get the first symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Downcast
                    let token = s.as_any().downcast_ref::<Token>().unwrap();

                    // Switch on its kind
                    match token.kind {
                        TerminalKind::LBRACKET => {
                            // Simply join it in a smallfactor
                            let ns = Box::new(ASTNode::SmallFactor{
                                kind: ValueKind::Undefined,
                                expr: Box::new(last_node.clone()),
                                pos1: token.pos1,
                                pos2: last_token.pos2
                            });
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Done
                            return String::from("brackets");
                        }

                        // Simply ignore the rest; any bracket errors are treated during the post-analysis
                        _ => { return String::new(); }
                    }

                } else {
                    // Simply ignore the rest; any bracket errors are treated during the post-analysis
                    return String::new();
                }
            }
        }

        // Should never get here!
        
    }
}





/***** LIBRARY FUNCTIONS *****/
/// Parses the given string into an AST of the math it represents.
/// 
/// **Arguments**
///  * `input`: The string to parse.
/// 
/// **Returns**  
/// The first node in the AST, or None if an error occurred (which will already have been printed).
pub fn parse<'a>(input: &'a str) -> Option<ASTNode> {
    // Prepare the tokenizer to use for input
    let mut tokenizer = Tokenizer::new(input);
    let mut lookahead = tokenizer.get();

    // Prepare the stack
    let mut stack: Vec<Box<dyn Symbol>> = Vec::new();

    // Start parsing
    let mut errored = false;
    loop {
        // // Print the current stack
        // for i in 0..stack.len() {
        //     print!(" {:?}", stack[i]);
        // }
        // println!();

        // Try to reduce the current stack
        let rule = reduce(input, &mut stack, &lookahead);

        // If we did anything, retry; otherwise, reduce to get more
        if rule.len() > 0 {
            // If it's an error, mark it
            if rule.eq("error") { errored = true; }
            continue;
        } else {
            // Get the next token
            match lookahead.kind {
                TerminalKind::Eos => {
                    // No more tokens; we're done parsing
                    break;
                }
                TerminalKind::Undefined(ref err) => {
                    // Encountered an unknown token; try to get more
                    eprintln!("{}: Encountered unknown token '{}'.", lookahead.pos1, *err);
                    errored = true;
                }
                _ => {
                    // It's a legal token; push it to the stack
                    stack.push(Box::new(lookahead));
                }
            }

            // Update the lookahead
            lookahead = tokenizer.get();
        }
    }

    // Next, analyse the remaining stack
    let mut is_cmd = false;
    for i in 0..stack.len() {
        let s = &stack[i];

        // Switch on terminal VS nonterminal
        if s.is_terminal() {
            // Downcast
            let token = s.as_any().downcast_ref::<Token>().unwrap();

            // Switch on its kind
            match token.kind {
                TerminalKind::LBRACKET => {
                    eprintln!("{}: Unmatched left bracket.", token.pos1);
                    errored = true;
                    continue;
                }
                TerminalKind::RBRACKET => {
                    eprintln!("{}: Unmatched right bracket.", token.pos1);
                    errored = true;
                    continue;
                }

                _ => {
                    eprintln!("{}: Unexpected symbol '{}'.", token.pos1, &input[token.pos1 - 1..token.pos2]);
                    errored = true;
                    continue;
                }
            }

        } else {
            // Downcast
            let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

            // Switch on its kind
            match node {
                ASTNode::Expr{ override_kind: _, kind: _, expr: _, pos1: _, pos2: _ } => {
                    // Compain if in command mode
                    if is_cmd {
                        eprintln!("{}: Cannot give an expression ('{}') in between a command.", node.pos().0, &input[node.pos().0 - 1..node.pos().1]);
                        errored = true;
                        continue;
                    }
                }
                ASTNode::Cmd{ cmd: _, pos1: _, pos2:_  } => {
                    // Only change modes if it's the first
                    if i == 0 { is_cmd = true; }
                    else {
                        eprintln!("{}: Cannot give a command ('{}') in between an expression.", node.pos().0, &input[node.pos().0 - 1..node.pos().1]);
                        errored = true;
                        continue;
                    }
                }

                _ => {
                    eprintln!("{}: Unexpected symbol '{}'.", node.pos().0, &input[node.pos().0 - 1..node.pos().1]);
                    errored = true;
                    continue;
                }
            }
        }
    }
    for i in 1..stack.len() {
        eprintln!("{}: Unexpected symbol '{}'.", stack[i].pos().0, &input[stack[i].pos().0 - 1..stack[i].pos().1]);
        errored = true;
    }

    // If an error occurred, stop
    if errored { return None; }

    // Done; return the single node!
    return Some(stack[0].as_any().downcast_ref::<ASTNode>().unwrap().clone());
}



/// Given an already processed and parsed AST, tries to return the command it represents.
/// 
/// **Arguments**
///  * `ast`: The AST to examine.
/// 
/// **Returns**  
/// The command to represent. If there is none, returns None.
pub fn get_command(ast: &ASTNode) -> Option<&ASTNode> {
    // See if the topmost node is a Cmd
    if let ASTNode::Cmd{ cmd, pos1: _, pos2: _ } = ast {
        // Return the cmd
        return Some(cmd);
    }

    // Return None
    return None;
}





/// Given an already processed and parsed AST, returns the kind of the topmost expression.
/// 
/// **Arguments**
///  * `ast`: The AST to get the root node type from.
/// 
/// **Returns**  
/// The type of the root node as a ValueKind. While this can also be 'Undefined', this really shouldn't happen.
pub fn get_kind(ast: &ASTNode) -> ValueKind {
    // Check if the root node is an expression
    if let ASTNode::Expr{ override_kind: _, kind, expr: _, pos1: _, pos2: _ } = ast {
        return *kind;
    }

    // Otherwise, return blank
    return ValueKind::Undefined;
}
