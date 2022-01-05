/* PARSER.rs
 *   by Lut99
 *
 * Created:
 *   04 Jan 2022, 12:00:03
 * Last edited:
 *   05 Jan 2022, 12:39:06
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
enum ParserState {
    /// The start state
    Start,

    /// We've seen one Expr
    Expr,
    /// We've seen one Expr and an operator
    #[allow(non_camel_case_types)]
    Expr_Op,

    /// We've seen the right bracket
    RBracket,
    /// We've seen the right bracket followed by an expression
    #[allow(non_camel_case_types)]
    RBracket_Expr,
}

/// Defines all constant types in the AST.
#[derive(Debug, Clone)]
pub enum ConstantKind {
    /// Meta value for when no kind is defined yet
    Undefined,

    /// The decimal constant type
    Decimal,
    /// The hexadecimal constant type
    Hexadecimal,
    /// The binary constant type
    Binary,
}

/// Defines all mono operators in the AST.
#[derive(Debug, Clone)]
pub enum MonaryOperator {
    /// Meta value for when no operator is defined
    Undefined,

    /// The to-dec operator
    ToDec,
    /// The to-hex operator
    ToHex,
    /// The to-bin operator
    ToBin,
}

impl From<TerminalKind> for MonaryOperator {
    fn from(val: TerminalKind) -> Self {
        match val {
            TerminalKind::ToDecimal => { MonaryOperator::ToDec }
            TerminalKind::ToHex     => { MonaryOperator::ToHex }
            TerminalKind::ToBin     => { MonaryOperator::ToBin }
            _                       => { MonaryOperator::Undefined }
        }
    }
}

/// Defines all binary operators in the AST.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// Meta value for when no operator is defined
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

impl From<TerminalKind> for BinaryOperator {
    fn from(val: TerminalKind) -> Self {
        match val {
            TerminalKind::Plus     => { BinaryOperator::Plus }
            TerminalKind::Minus    => { BinaryOperator::Minus }
            TerminalKind::Multiply => { BinaryOperator::Multiply }
            TerminalKind::Divide   => { BinaryOperator::Divide }
            _                      => { BinaryOperator::Undefined }
        }
    }
}





/***** AST NODES *****/
/// Enum that defines the AST nodes
#[derive(Clone)]
pub enum ASTNode {
    /// A simple node that can be used as a placeholder
    Undefined,
    /// Defines the 'exit' node.
    Exit { pos1: usize, pos2: usize },

    /// Defines a runnable command that is NOT an expression.
    Cmd { cmd: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines an expression in the AST, which could be either a Const, BinOp or MonOp
    Expr { kind: ConstantKind, expr: Box<ASTNode>, pos1: usize, pos2: usize },

    /// Defines a binary operator in the AST
    BinOp { kind: ConstantKind, operator: BinaryOperator, left: Box<ASTNode>, right: Box<ASTNode>, pos1: usize, pos2: usize },
    /// Defines a monary operator in the AST
    MonOp { kind: ConstantKind, operator: MonaryOperator, expr: Box<ASTNode>, pos1: usize, pos2: usize },
    
    /// Defines a constant in the AST
    Const { kind: ConstantKind, value: u64, pos1: usize, pos2: usize },
}

impl std::fmt::Debug for ASTNode {
    /// Write a debug counterpart for this Token
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Undefined                => { write!(f, "Undefined") }
            ASTNode::Exit{ pos1: _, pos2: _ } => { write!(f, "Exit") }

            ASTNode::Cmd{ cmd: _, pos1: _, pos2: _ }         => { write!(f, "Cmd(...)") }
            ASTNode::Expr{ kind, expr: _, pos1: _, pos2: _ } => { write!(f, "Expr<{:?}>(...)", kind) }

            ASTNode::BinOp{ kind, operator, left: _, right: _, pos1: _, pos2: _ } => { write!(f, "BinOp<{:?}>(... {:?} ...)", kind, operator) }
            ASTNode::MonOp{ kind, operator, expr: _, pos1: _, pos2: _ }           => { write!(f, "MonOp<{:?}>({:?} ...)", kind, operator) }

            ASTNode::Const{ kind, value: _, pos1: _, pos2: _ } => { write!(f, "Const<{:?}>", kind) }
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
            ASTNode::Undefined          => { (usize::MAX, usize::MAX) }
            ASTNode::Exit{ pos1, pos2 } => { (*pos1, *pos2) }

            ASTNode::Cmd{ cmd: _, pos1, pos2 }            => { (*pos1, *pos2) }
            ASTNode::Expr{ kind: _, expr: _, pos1, pos2 } => { (*pos1, *pos2) }

            ASTNode::BinOp{ kind: _, operator: _, left: _, right: _, pos1, pos2 } => { (*pos1, *pos2) }
            ASTNode::MonOp{ kind: _, operator: _, expr: _, pos1, pos2 }           => { (*pos1, *pos2) }

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
            ASTNode::Undefined                          => {}
            ASTNode::Exit{ ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::Cmd{ cmd: _, ref mut pos1, ref mut pos2 }            => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::Expr{ kind: _, expr: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::BinOp{ kind: _, operator: _, left: _, right: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
            ASTNode::MonOp{ kind: _, operator: _, expr: _, ref mut pos1, ref mut pos2 }           => { *pos1 = new_pos1; *pos2 = new_pos2; }

            ASTNode::Const{ kind: _, value: _, ref mut pos1, ref mut pos2 } => { *pos1 = new_pos1; *pos2 = new_pos2; }
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
fn reduce(input: &str, stack: &mut Vec<Box<dyn Symbol>>) -> String {
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
                    match token.kind {
                        TerminalKind::Exit => {
                            // Replace with the nonterminal version
                            stack[i] = Box::new(ASTNode::Exit {
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("exit");
                        }

                        TerminalKind::Decimal(val) => {
                            // Replace on the stack with a constant
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ConstantKind::Decimal,
                                value: val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("constant_dec");
                        }
                        TerminalKind::Hex(val) => {
                            // Replace on the stack with a constant
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ConstantKind::Hexadecimal,
                                value: val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("constant_hex");
                        }
                        TerminalKind::Bin(val) => {
                            // Replace on the stack with a constant
                            stack[i] = Box::new(ASTNode::Const{
                                kind: ConstantKind::Binary,
                                value: val,
                                pos1: token.pos1, pos2: token.pos2
                            });
                            return String::from("constant_bin");
                        }

                        TerminalKind::RBracket => {
                            // Start consuming the next expression
                            last_token = token;
                            state = ParserState::RBracket;
                            continue;
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on its kind
                    match node {
                        ASTNode::Expr{ kind:_, expr: _, pos1: _, pos2: _ } => {
                            // Start traversing deeping in the thingy
                            last_node = node;
                            state = ParserState::Expr;
                            continue;
                        }

                        ASTNode::Exit{ pos1, pos2 } => {
                            // Cast to a command
                            stack[i] = Box::new(ASTNode::Cmd{
                                cmd: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("cmd");
                        }
                        ASTNode::MonOp{ kind: _, operator: _, expr: _, pos1, pos2 } | ASTNode::BinOp{ kind: _, operator: _, left: _, right: _, pos1, pos2 } | ASTNode::Const{ kind: _, value: _, pos1, pos2 } => {
                            // Cast to an expression
                            stack[i] = Box::new(ASTNode::Expr{
                                kind: ConstantKind::Undefined,
                                expr: Box::new(node.clone()),
                                pos1: *pos1, pos2: *pos2
                            });
                            return String::from("expr");
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }
                }
            }



            ParserState::Expr => {
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
                        // Could be a monary op
                        TerminalKind::ToDecimal | TerminalKind::ToHex | TerminalKind::ToBin => {
                            // It is; generate the new symbol
                            let ns = Box::new(ASTNode::MonOp{
                                kind: ConstantKind::Undefined,
                                operator: MonaryOperator::from(token.kind.clone()),
                                expr: Box::new(last_node.clone()),
                                pos1: token.pos1, pos2: last_node.pos().1 }
                            );
                            
                            // Insert it in the stack instead of the top two ones
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Done
                            return String::from("binop");
                        }

                        // Could be a binary op
                        TerminalKind::Plus | TerminalKind::Minus | TerminalKind::Multiply | TerminalKind::Divide => {
                            // Store this operator too, then continue
                            last_token = &token;
                            state = ParserState::Expr_Op;
                            continue;
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }

                } else {
                    // Whatever the case, this seems funky
                    eprintln!("{}: Missing operator before new value.", s.pos().1 + 1);
                    // Pop the most recent one from the stack
                    stack.remove(stack.len() - 1);
                    // Return
                    return String::from("error");
                }
            }

            ParserState::Expr_Op => {
                // Get the first symbol
                if i == 0 { return String::new(); }
                i -= 1;
                let s = &stack[i];

                // Switch on terminal VS nonterminal
                if s.is_terminal() {
                    // Whatever the case, this seems funky
                    eprintln!("{}: Stray operator.", last_token.pos1);
                    // Pop the most recent two from the stack
                    stack.remove(stack.len() - 1);
                    stack.remove(stack.len() - 1);
                    // Return
                    return String::from("error");

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on its kind
                    match node {
                        ASTNode::Expr{ kind:_, expr: _, pos1: _, pos2: _ } => {
                            // We have a binary operator! Construct it first
                            let ns = Box::new(ASTNode::BinOp{
                                kind: ConstantKind::Undefined,
                                operator: BinaryOperator::from(last_token.kind.clone()),
                                left: Box::new(node.clone()),
                                right: Box::new(last_node.clone()),
                                pos1: s.pos().0, pos2: last_node.pos().1
                            });
                            
                            // Replace it on the stack
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Return
                            return String::from("binop");
                        }

                        // The rest is probably a malformed node
                        _ => {
                            eprintln!("{}: Encountered unexpected operator.", last_token.pos1);
                            // Pop the most recent two from the stack
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            // Return
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
                    // Whatever the case, this seems funky
                    eprintln!("{}: Unexpected right bracket.", last_token.pos1);
                    // Pop the most recent from the stack
                    stack.remove(stack.len() - 1);
                    // Return
                    return String::from("error");

                } else {
                    // Downcast
                    let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

                    // Switch on its kind
                    match node {
                        ASTNode::Expr{ kind:_, expr: _, pos1: _, pos2: _ } => {
                            // Store it, and keep parsing!
                            last_node = node;
                            state = ParserState::RBracket_Expr;
                            continue;
                        }

                        // The rest is probably a malformed node
                        _ => {
                            eprintln!("{}: Encountered right bracket.", last_token.pos1);
                            // Pop the most recent two from the stack
                            stack.remove(stack.len() - 1);
                            // Return
                            return String::from("error");
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
                        TerminalKind::LBracket => {
                            // Simply join it in an expression
                            let mut ns = Box::new(last_node.clone());
                            ns.set_pos(token.pos1, last_token.pos2);
                            stack.remove(stack.len() - 1);
                            stack.remove(stack.len() - 1);
                            stack[i] = ns;

                            // Done
                            return String::from("brackets");
                        }

                        // Could be a binary op
                        TerminalKind::Plus | TerminalKind::Minus | TerminalKind::Multiply | TerminalKind::Divide => {
                            // Store this operator too, then continue
                            last_token = &token;
                            state = ParserState::Expr_Op;
                            continue;
                        }

                        // Ignore the rest
                        _ => { return String::new(); }
                    }

                } else {
                    // Whatever the case, this seems funky
                    eprintln!("{}: Encountered unexpected symbol '{}'.", last_node.pos().0, &input[last_node.pos().0 - 1..last_node.pos().1]);
                    // Pop the most recent two from the stack
                    stack.remove(stack.len() - 1);
                    stack.remove(stack.len() - 1);
                    // Return
                    return String::from("error");
                }
            }
        }
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
        let rule = reduce(input, &mut stack);

        // If we did anything, retry; otherwise, reduce to get more
        if rule.len() > 0 {
            // If it's an error, mark it
            if rule.eq("error") { errored = true; }
            continue;
        } else {
            // Get the next token
            let token = tokenizer.get();
            match token.kind {
                TerminalKind::Eos => {
                    // No more tokens; we're done parsing
                    break;
                }
                TerminalKind::Undefined(err) => {
                    // Encountered an unknown token; try to get more
                    eprintln!("{}: Encountered unknown token '{}'.", token.pos1, err);
                    continue;
                }
                _ => {
                    // It's a legal token; push it to the stack
                    stack.push(Box::new(token));
                }
            }
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
                _ => {}
            }

        } else {
            // Downcast
            let node = s.as_any().downcast_ref::<ASTNode>().unwrap();

            // Switch on its kind
            match node {
                ASTNode::Expr{ kind: _, expr: _, pos1: _, pos2: _ } => {
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
    if stack.len() != 1 {
        eprintln!("{}: Unexpected symbol '{}'.", stack[0].pos().0, &input[stack[0].pos().0 - 1..stack[0].pos().1]);
        errored = true;
    }

    // If an error occurred, stop
    if errored { return None; }

    // Done; return the single node!
    return Some(stack[0].as_any().downcast_ref::<ASTNode>().unwrap().clone());
}
