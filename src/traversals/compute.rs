/* COMPUTE.rs
 *   by Lut99
 *
 * Created:
 *   06 Jan 2022, 12:59:34
 * Last edited:
 *   11 Jan 2022, 14:15:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Traverses the tree and performs the computations!
**/

use crate::ast::symbol_table::SymbolTable;
use crate::ast::parser::LowBinaryOperator;
use crate::ast::parser::HighBinaryOperator;
use crate::ast::parser::ASTNode;


/***** NODE FUNCTIONS *****/
/// Traverses the given node and annotates it with the constant type.
/// 
/// **Arguments**
///  * `node`: The node to traverse.
///  * `value`: The intermediate value on which we operate.
///  * `symtable`: The symbol table that we use to keep track of identifiers.
///  * `error`: Can be set to indicate an error has occurred.
/// 
/// **Returns**  
/// The given node, or else a replacement if deemed necessary.
fn traverse_node(mut node: ASTNode, value: &mut u64, symtable: &mut SymbolTable, error: &mut bool) -> ASTNode {
    // Switch on the node
    match node {
        ASTNode::Expr{ override_kind: _, kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve the child's value
            **expr = traverse_node(*expr.clone(), value, symtable, error);
        }

        ASTNode::Assign{ override_kind: _, kind: _, ref identifier, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve the child's value
            **expr = traverse_node(*expr.clone(), value, symtable, error);

            // Also push the update to the symbol table
            symtable.get_mut(identifier).unwrap().1 = *value;
        }
        ASTNode::BinOpLow{ override_kind: _, kind: _, operator, ref mut left, ref mut right, pos1, pos2: _ } => {
            // Traverse to resolve the children's value
            let mut left_val: u64 = 0; let mut right_val: u64 = 0;
            **left  = traverse_node(*left.clone(), &mut left_val, symtable, error);
            **right = traverse_node(*right.clone(), &mut right_val, symtable, error);

            // Switch on the operator type
            match operator {
                LowBinaryOperator::Plus => {
                    // Try to add the values
                    let result = left_val.checked_add(right_val);
                    match result {
                        Some(val) => {
                            *value = val;
                        }
                        None => {
                            // Overflow
                            eprintln!("   {}: Overflow occurred while performing {} + {}.", pos1, left_val, right_val);
                            *error = true;
                        }
                    }
                }
                LowBinaryOperator::Minus => {
                    // Try to subtract the values
                    let result = left_val.checked_sub(right_val);
                    match result {
                        Some(val) => {
                            *value = val;
                        }
                        None => {
                            // Overflow
                            eprintln!("   {}: Overflow occurred while performing {} - {}.", pos1, left_val, right_val);
                            *error = true;
                        }
                    }
                }
                LowBinaryOperator::Undefined => {
                    panic!("Encountered an undefined binoplow at pos {}: this should never happen!", pos1);
                }
            }
        }
        ASTNode::BinOpHigh{ override_kind: _, kind: _, operator, ref mut left, ref mut right, pos1, pos2: _ } => {
            // Traverse to resolve the children's value
            let mut left_val: u64 = 0; let mut right_val: u64 = 0;
            **left  = traverse_node(*left.clone(), &mut left_val, symtable, error);
            **right = traverse_node(*right.clone(), &mut right_val, symtable, error);

            // Switch on the operator type
            match operator {
                HighBinaryOperator::Multiply => {
                    // Try to add the values
                    let result = left_val.checked_mul(right_val);
                    match result {
                        Some(val) => {
                            *value = val;
                        }
                        None => {
                            // Overflow
                            eprintln!("   {}: Overflow occurred while performing {} * {}.", pos1, left_val, right_val);
                            *error = true;
                        }
                    }
                }
                HighBinaryOperator::Divide => {
                    // Try to subtract the values
                    let result = left_val.checked_div(right_val);
                    match result {
                        Some(val) => {
                            *value = val;
                        }
                        None => {
                            // Overflow
                            eprintln!("   {}: Overflow occurred while performing {} / {}.", pos1, left_val, right_val);
                            *error = true;
                        }
                    }
                }
                HighBinaryOperator::Undefined => {
                    panic!("Encountered an undefined binophigh at pos {}: this should never happen!", pos1);
                }
            }
        }
        ASTNode::MonOp{ kind: _, ref mut expr, pos1: _, pos2: _ } => {
            // Traverse to resolve the child's value
            **expr = traverse_node(*expr.clone(), value, symtable, error);
        }

        ASTNode::Id{ ref identifier, pos1: _, pos2: _ } => {
            // Fetch the value from the symbol table
            let (_, sym_value) = symtable.get(identifier).unwrap();
            *value = *sym_value;
        }
        ASTNode::Const{ kind: _, value: new_value, pos1: _, pos2: _ } => {
            // Simply pass its value
            *value = new_value;
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
///  * `value`: A reference to the value which will contain the result.
///  * `symbol_table`: The symbol table that we use to keep track of identifiers.
/// 
/// **Returns**  
/// The node to traverse, or else a replacement if the algorithm deems it necessary.
pub fn traverse(ast: ASTNode, value: &mut u64, symbol_table: &mut SymbolTable) -> Option<ASTNode> {
    // Simply return the traverse_node call
    let mut error: bool = false;
    let new_ast = traverse_node(ast, value, symbol_table, &mut error);
    if error { return None; }
    return Some(new_ast);
}
