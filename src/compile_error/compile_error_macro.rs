/// Defines or throws a compile error
/// 
/// ## Error Example
/// ```rust
/// flexar::compiler_error! {
///     [[Define] CompilerErrors]
/// 
///     [Errors]
///     /// An example error
///     (E001) "example error type": "examle error msg";
///     /// Invalid character error
///     (E002) "invalid character": "character `", "` is invalid";
/// }
/// ```
/// ## Throwing Example
/// `flexar::compile_error!((E002, position), '$')`
#[macro_export]
macro_rules! compiler_error {
    (($id:ident, $pos:expr) $($arg:expr),*) => {{
        $crate::compile_error::CompileError::new(stringify!($id), $crate::compile_error::CompileError::$id.error_type, $crate::compile_error::CompileError::$id.fmt.format(&[$($arg.to_string()),*]), $pos)
    }};

    ([[Define] $name:ident]$(
        $([$($separator:tt)*])?
        $(#[$about:meta])* ($id:ident) $error_type:literal : $($msg:literal),+
    );* $(;)?) => {
        pub trait $name {
            $(
                $crate::compiler_error!(@trait $(#[$about])* $id $crate::compiler_error!(@count 1, $($msg)+));
            )*
        }

        impl $name for $crate::compile_error::CompileError {
            $(
                $crate::compiler_error!(@impl $(#[$about])* $id $error_type $crate::compiler_error!(@count 1, $($msg)+), $($msg)+);
            )*
        }
    };

    (@count $count:expr, $head:literal) => {
        $count
    };

    (@count $count:expr, $head:literal $($tail:literal)+) => {
        $crate::compiler_error!(@count $count+1, $($tail)+)
    };

    (@impl $(#[$about:meta])* $id:ident $error_type:literal $len:expr, $($str:literal)+) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<{$len}> = $crate::compile_error::CompileErrorTemplate::new($error_type, $crate::compilerr_fmt!(($len) $($str),+));
    };

    (@trait $(#[$about:meta])* $id:ident $len:expr) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<{$len}>;
    };
}