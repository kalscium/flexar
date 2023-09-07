pub struct CompileError;

impl CompileError {
    pub fn throw(msg: &str) {
        panic!("{msg}");
    }
}

macro_rules! compile_error {
    (($id:ident) $msg:literal) => {{
        impl CompileError {
            pub const $id: &'static str = $msg;
        }

        compile_error!($id)
    }};

    ($id:ident) => {
        CompileError::throw(CompileError::$id)
    }
}