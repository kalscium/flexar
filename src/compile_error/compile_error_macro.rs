#[macro_export]
macro_rules! compile_error {
    ($(
        $([$separator:ident])?
        $(#[$about:meta])* ($id:ident) $origin:literal : $msg:tt
    );* $(;)?) => {
        impl $crate::compile_error::CompileError {
            $(
                compile_error!(@inner $(#[$about])* ($id) $origin : $msg);
            )*
        }
    };

    (@inner $(#[$about:meta])* ($id:ident) $origin:literal : (($len:literal) $($str:literal),*)) => {
        $(#[$about])*
        pub const $id: $crate::compile_error::CompileErrorTemplate<{$len+1}> = $crate::compile_error::CompileErrorTemplate::new($origin, $crate::compilerr_fmt!(($len) $($str),*));
    };

    (@inner $(#[$about:meta])* ($id:ident) $origin:literal : $str:literal) => {
        $(#[$about])*
        pub const $id: $crate::compile_error::CompileErrorTemplate<1> = $crate::compile_error::CompileErrorTemplate::new($origin, $crate::compilerr_fmt!($str));
    };

    (($id:ident, $pos:expr) $($arg:expr),*) => {{
        $crate::compile_error::CompileError::new(CompileError::$id.origin, $crate::compile_error::CompileError::$id.fmt.format(&[$($arg.to_string()),*]), $pos)
    }}
}