/* PRINT.rs
 *   by Lut99
 *
 * Created:
 *   05 Jan 2022, 12:39:41
 * Last edited:
 *   05 Jan 2022, 16:45:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Traverses the AST to print is all out as neatly as possible.
**/

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
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode, indent: usize) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Undefined => {
            println!("{}<Undefined>", n_spaces!(indent));
        }

        ASTNode::Cmd{ ref mut cmd, pos1: _, pos2: _ } => {
            // Traverse its child to discover the contents
            println!("{}Cmd(", n_spaces!(indent));
            **cmd = traverse_node(*cmd.clone(), indent + 3);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::Exit{ pos1: _, pos2: _ } => {
            println!("{}Exit", n_spaces!(indent));
        }

        ASTNode::Expr{ kind, ref mut expr, pos1: _, pos2: _ } => {
            // Print the child of the expression first
            println!("{}Expr<{:?}>(", n_spaces!(indent), kind);
            **expr = traverse_node(*expr.clone(), indent + 3);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::BinOp{ kind, operator, ref mut left, ref mut right, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}BinOp<{:?}>(", n_spaces!(indent), kind);
            **left = traverse_node(*left.clone(), indent + 3);
            println!("{}{:?}", n_spaces!(indent + 3), operator);
            **right = traverse_node(*right.clone(), indent + 3);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::MonOp{ kind, operator, ref mut expr, pos1: _, pos2: _ } => {
            // Print the binop with its expressions - but now we use indent
            println!("{}MonOp<{:?}>(", n_spaces!(indent), kind);
            println!("{}{:?}", n_spaces!(indent + 3), operator);
            **expr = traverse_node(*expr.clone(), indent + 3);
            println!("{})", n_spaces!(indent));
        }
        ASTNode::Const{ kind, value, pos1: _, pos2: _ } => {
            // Print the constant as a constant
            println!("{}Const<{:?}>({})", n_spaces!(indent), kind, value);
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
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode) -> ASTNode {
    // Simply return the traverse_node call
    return traverse_node(ast, 0);
}
