pub struct CompileErrFormatter<const N: usize>(pub [&'static str; N]);

impl<const N: usize> CompileErrFormatter<N> {
    pub fn format(&self, inputs: &[String]) -> String {
        let mut string = String::new();

        let mut first = true;
        for (i, x) in self.0.iter().enumerate() {
            if !first { string.push_str(&inputs[i]) }
            else { first = false }
            string.push_str(x);
        } string
    }
}

#[macro_export]
macro_rules! compilerr_fmt {
    ($str:literal) => {
        $crate::compile_error::compile_error_format::CompileErrFormatter::<1>([$str])
    };

    (($len:literal) $($str:literal),*) => {
        $crate::compile_error::compile_error_format::CompileErrFormatter::<$len>([$($str),*])
    };
}