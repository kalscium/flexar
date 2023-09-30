#[macro_export]
macro_rules! parser {
    ([[$node:ty] $parxt:ident: $token:ty] $($func:ident {$($($pats:tt),* => $body:tt$end:tt)*} else $else:ident$else_body:tt;)*) => {
        impl $node {
            $(pub fn $func($parxt: &mut $crate::parxt::Parxt<'_, $token>) -> Result<$node, (u8, $crate::compile_error::CompileError)> {
                let mut last_error: Option<(u8, $crate::compile_error::CompileError)> = None;
                let mut child = $parxt.spawn();

                $($crate::parser!(@req $parxt child last_error $else$else_body 0, $($pats),* => $body$end);)*
                
                if let Some((i, x)) = last_error { if i > 0 { return Err((i, x)); } }
                Err($crate::parser!(@else $else$else_body 0))
            })*
        }
    };

    // Requirements
    (@req $parxt:ident $child:ident $last_error:ident $else:ident$else_body:tt $depth:expr, [$out:ident: $func:expr], $($tail:tt),* => $body:tt$end:tt) => {
        #[allow(unused_parens)]
        match $func(&mut $child) {
            Ok($out) => {
                $crate::parser!(@req $parxt $child $last_error $else$else_body $depth + 1, $($tail),* => $body$end);
            }
            Err((i, x)) => {
                let i = i + $depth;
                match $last_error {
                    None => $last_error = Some((i, x)),
                    Some((ii, _)) => if i > ii {
                        $last_error = Some((i, x));
                    },
                }
            },
        };
    };

    (@req $parxt:ident $child:ident $last_error:ident $else:ident$else_body:tt $depth:expr, [$out:ident: $func:expr] => $body:tt$end:tt) => {
        #[allow(unused_parens)]
        match $func(&mut $child) {
            Ok($out) => {
                $crate::parser!(@body $parxt $child $last_error $else$else_body $body$end | $depth + 1);
            }
            Err((i, x)) => {
                let i = i + $depth;
                match $last_error {
                    None => $last_error = Some((i, x)),
                    Some((ii, _)) => if i > ii {
                        $last_error = Some((i, x));
                    },
                }
            },
        };
    };

    (@req $parxt:ident $child:ident $last_error:ident $else:ident$else_body:tt $depth:expr, $head:pat, $($tail:tt),* => $body:tt$end:tt) => {
        #[allow(unused_parens)]
        if let $head = $child.current() {
            $crate::parser!(@advance $child $else$else_body $depth);
            $crate::parser!(@req $parxt $child $last_error $else$else_body $depth + 1, $($tail),* => $body$end);
        }
    };

    (@req $parxt:ident $child:ident $last_error:ident $else:ident$else_body:tt $depth:expr, $head:pat => $body:tt$end:tt) => {
        #[allow(unused_parens)]
        if let $head = $child.current() {
            $crate::parser!(@advance $child $else$else_body $depth);
            $crate::parser!(@body $parxt $child $last_error $else$else_body $body$end | $depth + 1);
        }
    };

    // Body
    (@body $parxt:ident $child:ident $last_error:ident $old_else:ident$old_else_body:tt {$($($pats:tt),* => $body:tt$end:tt)*} $((else $else:ident$else_body:tt))? $(;)? | $depth:expr) => {
        let mut last_error: Option<(u8, $crate::compile_error::CompileError)> = None;
        let mut child = $child.spawn();
        
        $($crate::parser!(@req $parxt child $last_error $old_else$old_else_body $depth, $($pats),* => $body$end);)* // not how I wanted it to work
        if let Some((i, x)) = last_error {
            $last_error = Some((i, x));
        } 
        $(#[allow(unreachable_code)]
        match $last_error {
            Some((i, _)) if i > $depth => *$parxt = $child, // if things break remove this,
            _ => {
                *$parxt = $child; // if things break remove this
                $last_error = Some($crate::parser!(@else $else$else_body $depth));
            },
        })?
    };

    (@body $parxt:ident $child:ident $last_error:ident $else:ident$else_body:tt ($node:expr); | $depth:expr) => {
        *$parxt = $child;
        return Ok($node);
    };

    // Else
    (@else Err$else:tt $depth:expr) => {
        ($depth, $crate::compiler_error!$else)
    };

    (@else Ok($else:expr) $depth:expr) => {
        return Ok($else);
    };

    (@else Other($variant:ident $else:expr) $depth:expr) => {
        match $else {
            Ok(x) => return Ok(Self::$variant(x)),
            Err((i, x)) => ((i + $depth, x)),
        }
    };

    (@advance $parxt:ident $else:tt$else_body:tt $depth:expr) => {
        if $parxt.done {
            return Err($crate::parser!(@else $else$else_body $depth));
        } $parxt.advance();
    };

    // Outputs
    (@output $variant:ident $rest:tt) => {
        Self::$variant$crate::parser!(@output $rest)
    };

    (@output $rest:tt) => {
        Self$rest
    };
}