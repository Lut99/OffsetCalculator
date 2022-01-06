/* SYMBOL TABLE.rs
 *   by Lut99
 *
 * Created:
 *   06 Jan 2022, 14:08:50
 * Last edited:
 *   06 Jan 2022, 14:13:09
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Provides code for the symbol table.
**/

pub use crate::ast::parser::ValueKind;

/// Type shortcut for the SymbolTable.
pub type SymbolTable = std::collections::HashMap<String, (ValueKind, u64)>;
