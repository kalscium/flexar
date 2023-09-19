use crate::compile_error;

compile_error! {

[Lexer]
    /// Occurs whenever there is an unexpected character (unknown symbol) while tokenizing a file
    /// ## Example
    /// ```example
    /// error[LX001]: invalid character
    ///  --> example:1:1
    /// 1 | $
    ///   | ^ not a valid character or symbol
    ///  <--
    (LX001) "invalid character": "not a valid character or symbol";

}