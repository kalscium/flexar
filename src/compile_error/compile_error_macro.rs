/// Defines or throws a compile error
/// 
/// ## Error Example
/// ```rust
/// flexar::compile_error! {
///     [[Define]]
/// 
///     [Errors]
///     /// An example error
///     (E001) "example error type": "examle error msg";
///     /// Invalid character error
///     (E002) "invalid character": ((1) "character `", "` is invalid");
/// }
/// ```
/// ## Throwing Example
/// `flexar::compile_error!((E002, position), '$')`
#[macro_export]
macro_rules! compile_error {
    (($id:ident, $pos:expr) $($arg:expr),*) => {{
        $crate::compile_error::CompileError::new(stringify!($id), $crate::compile_error::CompileError::$id.error_type, $crate::compile_error::CompileError::$id.fmt.format(&[$($arg.to_string()),*]), $pos)
    }};

    ([[Define]]$(
        $([$separator:ident])?
        $(#[$about:meta])* ($id:ident) $error_type:literal : $msg:tt
    );* $(;)?) => {
        pub trait CompileErrors {
            $(
                $crate::compile_error!(@trait $(#[$about])* $id $msg);
            )*
        }

        impl CompileErrors for $crate::compile_error::CompileError {
            $(
                $crate::compile_error!(@impl $(#[$about])* $id $error_type $msg);
            )*
        }
    };

    (@impl $(#[$about:meta])* $id:ident $error_type:literal (($len:literal) $($str:literal),*)) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<{$len+1}> = $crate::compile_error::CompileErrorTemplate::new($error_type, $crate::compilerr_fmt!(($len) $($str),*));
    };

    (@impl $(#[$about:meta])* $id:ident $error_type:literal $str:literal) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<1> = $crate::compile_error::CompileErrorTemplate::new($error_type, $crate::compilerr_fmt!($str));
    };

    (@trait $(#[$about:meta])* $id:ident (($len:literal) $($str:literal),*)) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<{$len+1}>;
    };

    (@trait $(#[$about:meta])* $id:ident $str:literal) => {
        $(#[$about])*
        const $id: $crate::compile_error::CompileErrorTemplate<1>;
    };
}