/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   22 Dec 2021, 17:20:49
 * Last edited:
 *   22 Dec 2021, 17:45:07
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The OpString library provides the OpString (Operational String) class,
 *   which can be generated from a normal string and works solely one
 *   graphene units; basically as string as you'd expect.
**/

use std::ops;
use std::fmt;
use unicode_segmentation::{UnicodeSegmentation};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}





/***** OPSTRING CLASS *****/
/// The OpString class can 'wrap' around a normal string to work with graphene units instead of the normal, character method.
/// 
/// Basically tries to mimic a string the way you'd expect.
#[derive(Debug)]
struct OpString<'a> {
    /// Reference to the parent string
    parent : &'a str,
    /// The list of graphene items in this string
    chars  : Vec<&'a str>,
}

impl<'a> OpString<'a> {
    /// Constructor for the OpString class.
    /// 
    /// **Arguments**
    ///  * `s`: The normal string to wrap around. Note that only a reference is kept, so the lifetime of OpString is the same as the string.
    ///
    /// **Returns**
    /// A fully instantiated OpString instance.
    pub fn new(s: &'a str) -> OpString {
        OpString {
            parent : s,
            chars  : UnicodeSegmentation::graphemes(s, true).collect()
        }
    }
}

impl<'a> Default for OpString<'a> {
    /// Returns a default instance for the OpString class.
    fn default() -> Self {
        OpString::new("")
    }
}

impl<'a> fmt::Display for OpString<'a> {
    /// Formats the string nicely in a normal format operation.
    /// 
    /// **Arguments**
    ///  * `f`: The format configuration to use for writing.
    /// 
    /// **Returns**  
    /// Whether the writing was successful or not, as a fmt::Result.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parent)
    }
}

impl<'a> ops::Index<usize> for OpString<'a> {
    /// The output type of the immutable index operation.
    type Output = str;

    /// The immuteable index operation for he OpString class.
    /// 
    /// **Arguments**
    ///  * `index`: The index to return.
    /// 
    /// **Returns**
    /// The requested value. Will panic! if out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        // Make sure we're not out-of-bounds
        if index >= self.chars.len() {
            panic!("Index {} is out of bounds for OpString of size {}.", index, self.chars.len());
        }

        // Return the item
        return self.chars[index];
    }
}
