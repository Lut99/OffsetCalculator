/* SYMBOL TABLE.rs
 *   by Lut99
 *
 * Created:
 *   06 Jan 2022, 14:07:35
 * Last edited:
 *   11 Jan 2022, 14:14:54
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Traverses the tree to resolve identifiers in the active symbol table.
**/

use crate::ast::symbol_table::SymbolTable;
use crate::ast::parser::ValueKind;
use crate::ast::parser::ASTNode;


/***** NODE FUNCTIONS *****/
/// Traverses the given node and tries to resolve identifiers with the given symbol table.
/// 
/// **Arguments**
///  * `node`: The node to traverse.
///  * `symbol_table`: The SymbolTable with declared identifiers.
///  * `error`: Can be set to indicate an error has occurred.
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode, symbol_table: &mut SymbolTable, error: &mut bool) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Expr{ override_kind: _, kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve it
            **expr = traverse_node(*expr.clone(), symbol_table, error);
        }

        ASTNode::Assign{ override_kind: _, kind: _, ref identifier, ref mut expr, pos1: _, pos2: _ } => {
            // Try to make a note in the table
            if !symbol_table.contains_key(identifier) {
                symbol_table.insert(identifier.clone(), (ValueKind::Undefined, 0));
            }

            // Now traverse into its child
            **expr = traverse_node(*expr.clone(), symbol_table, error);
        }
        ASTNode::BinOpLow{ override_kind: _, kind: _, operator: _, ref mut left, ref mut right, pos1: _, pos2: _ } |
        ASTNode::BinOpHigh{ override_kind: _, kind: _, operator: _, ref mut left, ref mut right, pos1: _, pos2: _ } => {
            // Traverse both children return
            **left = traverse_node(*left.clone(), symbol_table, error);
            **right = traverse_node(*right.clone(), symbol_table, error);
        }

        ASTNode::MonOp{ kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse its child return
            **expr = traverse_node(*expr.clone(), symbol_table, error);
        }

        ASTNode::Id { ref identifier, pos1, pos2: _ } => {
            // See if we have seen it
            if !symbol_table.contains_key(identifier) {
                eprintln!("   {}: Unknown identifier '{}'.", pos1, identifier);
                *error = true;
            }
        }

        _ => {
            // Just return the node itself
        }
    }

    // Done!
    return node;
}





/***** LIBRARY FUNCTIONS *****/
/// Traverses the given AST to try and trim the now obsolete smallfactors, factors and terms.
/// 
/// **Arguments**
///  * `ast`: The AST to traverse.
///  * `symbol_table`: The SymbolTable with declared identifiers.
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode, symbol_table: &mut SymbolTable) -> Option<ASTNode> {
    // Simply return the traverse_node call
    let mut error: bool = false;
    let new_ast = traverse_node(ast, symbol_table, &mut error);
    if error { return None; }
    return Some(new_ast);
}

