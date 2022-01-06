/* TRIM.rs
 *   by Lut99
 *
 * Created:
 *   06 Jan 2022, 12:04:53
 * Last edited:
 *   06 Jan 2022, 15:08:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines a traversal that tries to trim the tree by removing all the
 *   precedence nonterminals, and resulting in only expressions.
**/

use crate::ast::parser::ASTNode;


/***** NODE FUNCTIONS *****/
/// Traverses the given node and removes it if necessary.
/// 
/// **Arguments**
///  * `node`: The node to traverse.
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Expr{ override_kind: _, kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve it
            **expr = traverse_node(*expr.clone());

            // If that turns out to be an expression as well, return our child instead of that
            match **expr {
                ASTNode::Expr{ override_kind: _, kind: _, expr: _, pos1: _, pos2: _ } => {
                    // Return the child instead
                    return *expr.clone();
                }

                _ => {
                    // Return us
                    return node;
                }
            }
        }

        ASTNode::StrongExpr{ kind, ref mut expr, pos1, pos2 } |
        ASTNode::Term{ kind, ref mut expr, pos1, pos2 } |
        ASTNode::Factor{ kind, ref mut expr, pos1, pos2 } |
        ASTNode::SmallFactor{ kind, ref mut expr, pos1, pos2 } => {
            // Traverse into the child
            **expr = traverse_node(*expr.clone());

            // If the child is an expression, remove this; otherwise, replace with an expression ourselves
            match **expr {
                ASTNode::Expr{ override_kind: _, kind: _, expr: _, pos1: _, pos2: _ } => {
                    // Return the child instead
                    return *expr.clone();
                }

                _ => {
                    // Replace ourselves with an expression
                    return ASTNode::Expr{
                        override_kind: false,
                        kind: kind,
                        expr: expr.clone(),
                        pos1: pos1, pos2: pos2
                    };
                }
            }
        }

        ASTNode::Assign{ override_kind: _, kind: _, identifier: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse its child return
            **expr = traverse_node(*expr.clone());
            return node;
        }
        ASTNode::BinOpLow{ override_kind: _, kind: _, operator: _, ref mut left, ref mut right, pos1: _, pos2: _ } |
        ASTNode::BinOpHigh{ override_kind: _, kind: _, operator: _, ref mut left, ref mut right, pos1: _, pos2: _ } => {
            // Traverse both children return
            **left = traverse_node(*left.clone());
            **right = traverse_node(*right.clone());
            return node;
        }

        ASTNode::MonOp{ kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse its child return
            **expr = traverse_node(*expr.clone());
            return node;
        }

        _ => {
            // Just return the node itself
            return node;
        }
    }
}





/***** LIBRARY FUNCTIONS *****/
/// Traverses the given AST to try and trim the now obsolete smallfactors, factors and terms.
/// 
/// **Arguments**
///  * `ast`: The AST to traverse.
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode) -> ASTNode {
    // Simply return the traverse_node call
    return traverse_node(ast);
}
