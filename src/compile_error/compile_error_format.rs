pub struct CompileErrFormatter<const N: usize>(pub &'static str, pub [&'static str; N]);

impl<const N: usize> CompileErrFormatter<N> {
    pub fn format(&self, inputs: &[String]) -> String {
        let mut string = String::new();

        // Check if the args are correct (replace with compile-time check later)
        if N-1 != inputs.len() {
            panic!("FlexarCompilerErrorFormatter: compiler error `{}` expected `{}` arg(s) but received `{}`", self.0, N-1, inputs.len());
        }

        let mut first = true;
        for (i, x) in self.1.iter().enumerate() {
            if !first { string.push_str(&inputs[i-1]) }
            else { first = false }
            string.push_str(x);
        } string
    }
}

#[macro_export]
macro_rules! compilerr_fmt {
    (($len:expr) $id:ident $($str:literal),*) => {
        $crate::compile_error::compile_error_format::CompileErrFormatter::<{$len}>(stringify!($id), [$($str),*])
    };
}