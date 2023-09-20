#[macro_export]
macro_rules! compile_error {
    (($id:ident, $pos:expr) $($arg:expr),*) => {{
        $crate::compile_error::CompileError::new(stringify!($id), $crate::compile_error::CompileError::$id.error_type, $crate::compile_error::CompileError::$id.fmt.format(&[$($arg.to_string()),*]), $pos)
    }};

    ($(
        $([$separator:ident])?
        $(#[$about:meta])* ($id:ident) $error_type:literal : $msg:tt
    );* $(;)?) => {
        pub trait CompileErrors;
        impl CompileErrors for $crate::compile_error::CompileError {
            $(
                compile_error!(@inner $(#[$about])* ($id) $error_type : $msg);
            )*
        }
    };

    (@inner $(#[$about:meta])* ($id:ident) $error_type:literal : (($len:literal) $($str:literal),*)) => {
        $(#[$about])*
        pub const $id: $crate::compile_error::CompileErrorTemplate<{$len+1}> = $crate::compile_error::CompileErrorTemplate::new($error_type, $crate::compilerr_fmt!(($len) $($str),*));
    };

    (@inner $(#[$about:meta])* ($id:ident) $error_type:literal : $str:literal) => {
        $(#[$about])*
        pub const $id: $crate::compile_error::CompileErrorTemplate<1> = $crate::compile_error::CompileErrorTemplate::new($error_type, $crate::compilerr_fmt!($str));
    };
}