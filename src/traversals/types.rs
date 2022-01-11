/* TYPES.rs
 *   by Lut99
 *
 * Created:
 *   06 Jan 2022, 09:17:21
 * Last edited:
 *   11 Jan 2022, 14:15:01
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Traverses the AST to properly annotate everything of the ConstKind.
**/

use crate::ast::symbol_table::SymbolTable;
use crate::ast::parser::ValueKind;
use crate::ast::parser::ASTNode;


/***** HELPER MACROS *****/
/// Returns the child's override and kind, based on its type.
/// 
/// **Arguments**
///  * `child`: The node to extract the type data from.
/// 
/// **Returns**  
/// The override_kind and kind of that child as a tuple (in that order). If something's wrong and the type doesn't have a type, panics.
macro_rules! get_child_kind {
    ($child: expr, $symtable: expr, $error: expr) => {
        match $child {
            ASTNode::Expr{ override_kind: child_override_kind, kind: child_kind, expr: _, pos1: _, pos2: _ } |
            ASTNode::Assign{ override_kind: child_override_kind, kind: child_kind, identifier: _, expr: _, pos1: _, pos2: _ } |
            ASTNode::BinOpLow{ override_kind: child_override_kind, kind: child_kind, operator: _, left: _, right: _, pos1: _, pos2: _ } |
            ASTNode::BinOpHigh{ override_kind: child_override_kind, kind: child_kind, operator: _, left: _, right: _, pos1: _, pos2: _ } => {
                // Take on the override kind and the kind from this child
                (child_override_kind, child_kind)
            }
            ASTNode::MonOp{ kind: child_kind, expr: _, pos1: _, pos2: _ } => {
                // Set the child's type to ours, with an always-override type
                (true, child_kind)
            }

            ASTNode::Id{ ref identifier, pos1, pos2: _ } => {
                // Get the data
                let (kind, _) = $symtable.get(identifier).unwrap();

                // If the type is undefined, it's never been initialized (ans)
                if *kind == ValueKind::Undefined {
                    eprintln!("   {}: Identifier '{}' is defined, but not initialized yet.", pos1, identifier);
                    *$error = true;
                }

                // Return the valuekind in its stead
                (false, *kind)
            }
            ASTNode::Const{ kind: child_kind, value: _, pos1: _, pos2: _ } => {
                // Set the child's type to ours, with a never overriding type
                (false, child_kind)
            }

            _ => {
                // Panic
                panic!("Encountered non-typed {:?} node nested in the expression tree; this shouldn't happen!", $child);
            }
        }
    }
}





/***** NODE FUNCTIONS *****/
/// Traverses the given node and annotates it with the constant type.
/// 
/// **Arguments**
///  * `node`: The node to traverse.
///  * `symtable`: The symbol table that we use to keep track of identifiers.
///  * `error`: Can be set to indicate an error has occurred.
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode, symtable: &mut SymbolTable, error: &mut bool) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Expr{ ref mut override_kind, ref mut kind, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve the child's type
            **expr = traverse_node(*expr.clone(), symtable, error);

            // Based on the child, take what we need to properly propogate the type
            let (child_override_kind, child_kind) = get_child_kind!(**expr, symtable, error);
            *override_kind = child_override_kind;
            *kind = child_kind;
        }

        ASTNode::Assign{ ref mut override_kind, ref mut kind, ref identifier, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse through the child to resolve
            **expr  = traverse_node(*expr.clone(), symtable, error);

            // Based on the child, take what we need to properly propogate the type
            let (child_override_kind, child_kind) = get_child_kind!(**expr, symtable, error);
            *override_kind = child_override_kind;
            *kind = child_kind;

            // With this info, update the entry for this type
            symtable.get_mut(identifier).unwrap().0 = *kind;
        }
        ASTNode::BinOpLow{ ref mut override_kind, ref mut kind, operator: _, ref mut left, ref mut right, pos1, pos2: _ } |
        ASTNode::BinOpHigh{ ref mut override_kind, ref mut kind, operator: _, ref mut left, ref mut right, pos1, pos2: _ } => {
            // Traverse to resolve the children's type
            **left  = traverse_node(*left.clone(), symtable, error);
            **right = traverse_node(*right.clone(), symtable, error);

            // Read the properties from the children
            let (left_override_kind, left_kind)   = get_child_kind!(**left, symtable, error);
            let (right_override_kind, right_kind) = get_child_kind!(**right, symtable, error);

            // Now decide what to do
            if left_override_kind && right_override_kind && left_kind != right_kind {
                // Show error message, but take the left
                eprintln!("   {}: Ambigious typing: casted to both {:?} (LHS) and {:?} (RHS); choosing left.", pos1, left_kind, right_kind);
                *override_kind = true;
                *kind = left_kind;
            } else if !left_override_kind && right_override_kind {
                // Take the right one
                *override_kind = true;
                *kind = right_kind;
            } else if left_override_kind && !right_override_kind {
                // Take the left one
                *override_kind = true;
                *kind = left_kind;
            } else {
                // Neither; still ambigious, but deal with it silently this time.
                *override_kind = false;
                *kind = left_kind;
            }
        }

        // Ignore the rest
        _ => {}
    }

    // Done
    return node;
}





/***** LIBRARY FUNCTIONS *****/
/// Traverses the given AST to annotate all nodes with their types.
/// 
/// **Arguments**
///  * `ast`: The AST to traverse.
///  * `symbol_table`: The symbol table that we use to keep track of identifiers.
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode, symbol_table: &mut SymbolTable) -> Option<ASTNode> {
    // Simply return the traverse_node call
    let mut error = false;
    let new_ast = traverse_node(ast, symbol_table, &mut error);
    if error { return None };
    return Some(new_ast);
}
