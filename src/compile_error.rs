#[derive(Debug)]
pub struct CompileError {
    pub origin: &'static str,
    pub msg: &'static str,
}

impl CompileError {
    pub const fn new(origin: &'static str, msg: &'static str) -> Self {
        Self {
            origin,
            msg,
        }
    }

    pub fn throw(error: &CompileError) {
        panic!("{error:#?}");
    }
}

macro_rules! compile_error {
    (($id:ident) $origin:literal: $msg:literal) => {{
        impl CompileError {
            pub const $id: CompileError = CompileError::new($origin, $msg);
        }

        compile_error!($id)
    }};

    ($id:ident) => {
        CompileError::throw(CompileError::$id)
    }
}