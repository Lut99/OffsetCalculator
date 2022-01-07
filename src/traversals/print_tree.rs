/* PRINT TREE.rs
 *   by Lut99
 *
 * Created:
 *   05 Jan 2022, 12:39:41
 * Last edited:
 *   07 Jan 2022, 12:16:55
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Traverses the AST to print is all out as neatly as possible.
**/

use crate::ast::symbol_table::SymbolTable;
use crate::ast::parser::ValueKind;
use crate::ast::parser::ASTNode;


/***** HELPER MACROS *****/
/// Generates a string of N spaces.
/// 
/// **Returns**  
/// A String of length N, each character a space.
macro_rules! n_spaces {
    ($n: expr) => {
        (std::iter::repeat(' ').take($n).collect::<String>())
    };
}





/***** NODE FUNCTIONS *****/
/// Traverses the given node to print it.
/// 
/// **Arguments**
///  * `node`: The node to traverse.
///  * `indent`: The number of spaces to print before each line.
///  * `symtable`: The symbol table to use for resolving identifier types.
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode, indent: usize, symtable: &SymbolTable) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Undefined => {
            println!("{}<UNDEFINED>", n_spaces!(indent));
        }

        ASTNode::Cmd{ ref mut cmd, pos1: _, pos2: _ } => {
            // Traverse its child to discover the contents
            println!("{}Cmd(", n_spaces!(indent));
            **cmd = traverse_node(*cmd.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::Del{ ref identifier, pos1: _, pos2: _ } => {
            println!("{}Del({})", n_spaces!(indent), identifier);
        }
        ASTNode::DelAll{ pos1: _, pos2: _ } => {
            println!("{}DelAll", n_spaces!(indent));
        }
        ASTNode::ShowVars{ pos1: _, pos2: _ } => {
            println!("{}ShowVars", n_spaces!(indent));
        }
        ASTNode::ClearHist{ pos1: _, pos2: _ } => {
            println!("{}ClearHist", n_spaces!(indent));
        }
        ASTNode::Help{ pos1: _, pos2: _ } => {
            println!("{}Help", n_spaces!(indent));
        }
        ASTNode::Exit{ pos1: _, pos2: _ } => {
            println!("{}Exit", n_spaces!(indent));
        }

        ASTNode::Expr{ override_kind, kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the expression recursively
            println!("{}Expr<{} {:?}>(", n_spaces!(indent), override_kind, kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::StrongExpr{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the term recursively
            println!("{}StrongExpr<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::Term{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the term recursively
            println!("{}Term<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::Factor{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the term recursively
            println!("{}Factor<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::SmallFactor{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the term recursively
            println!("{}SmallFactor<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }

        ASTNode::Assign{ override_kind, kind, ref identifier, ref mut expr, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}Assign<{} {:?}>(", n_spaces!(indent), override_kind, kind);
            println!("{}{} =", n_spaces!(indent + 3), identifier);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::BinOpLow{ override_kind, kind, operator, ref mut left, ref mut right, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}BinOpL<{} {:?}>(", n_spaces!(indent), override_kind, kind);
            **left = traverse_node(*left.clone(), indent + 3, symtable);
            println!("{}{:?}", n_spaces!(indent + 3), operator);
            **right = traverse_node(*right.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::BinOpHigh{ override_kind, kind, operator, ref mut left, ref mut right, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}BinOpH<{} {:?}>(", n_spaces!(indent), override_kind, kind);
            **left = traverse_node(*left.clone(), indent + 3, symtable);
            println!("{}{:?}", n_spaces!(indent + 3), operator);
            **right = traverse_node(*right.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::MonOp{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}MonOp<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3, symtable);
            println!("{})", n_spaces!(indent));
        }

        ASTNode::Id{ ref identifier, pos1: _, pos2: _ } => {
            // Try to get the kind and the value
            let mut kind      = ValueKind::Undefined;
            let mut value:u64 = 0;
            if symtable.contains_key(identifier) {
                kind  = symtable.get(identifier).unwrap().0;
                value = symtable.get(identifier).unwrap().1;
            }

            // Print it
            println!("{}Id<{:?}>({}{})", n_spaces!(indent), kind, if symtable.contains_key(identifier) { identifier } else { "undeclared" }, if kind != ValueKind::Undefined { format!(" {}", value) } else { String::new() });
        }
        ASTNode::Const{ kind, value, pos1: _, pos2: _ } => {
            println!("{}{}<{:?}>", n_spaces!(indent), value, kind);
        }

    }

    // Done, return it
    return node;
}





/***** LIBRARY FUNCTIONS *****/
/// Traverses the given AST to print it all out.
/// 
/// **Arguments**
///  * `ast`: The AST to traverse.
///  * `symbol_table`: The symbol table to use for resolving identifier types.
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode, symbol_table: &SymbolTable) -> ASTNode {
    // Simply return the traverse_node call
    return traverse_node(ast, 0, symbol_table);
}
