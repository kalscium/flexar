/// Creates a lexer
#[macro_export]
macro_rules! lexer {
    ([[$token_type:ident] $lext:ident, $current:ident $(, $label:tt)?] else $no_match:expr; token_types {$($variant:ident$(($varin_name:ident: $varin_type:ty))? => $fmt:expr;)*} $($first:tt$sep:tt$second:tt;)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $token_type {
            $($variant$(($varin_type))?),*
        }

        impl std::fmt::Display for $token_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant$(($varin_name))? => write!(f, "{}", $fmt)),*
                }
            }
        }

        impl $token_type {
            pub fn tokenize(mut $lext: $crate::lext::Lext) -> Box<[$crate::token_node::Token<Self>]> {
                let mut tokens = Vec::<$crate::token_node::Token<Self>>::new();
                $($label:)? while let Some($current) = $lext.current {
                    tokens.push('code: {
                        $($crate::lexer!(@sect $lext 'code $current $first$sep$second);)*
                        $no_match
                    });
                    $lext.cursor.pos_start = $lext.cursor.pos_end.clone(); // cause different tokens with different start pos
                } tokens.into_boxed_slice()
            }
        }
    };
    
    // Sections
    
    (@sect $lext:ident $label:tt $current:ident $out:ident: ($($tail:tt)*)) => {{ // Change to something more efficient if too slow
        let mut child = $lext.spawn();
        $crate::lexer!(@recur-sect1 $label $out $lext child $($tail)*);
    }};

    (@sect $lext:ident $label:tt $current:ident $name:ident: $char:tt) => {
        if $crate::lexer!(@value $current $char) {
            use $crate::flext::Flext;
            $lext.advance();
            break $label $crate::token_node::Token {
                position: $lext.rposition(),
                token_type: Self::$name,
            };
        }
    };

    (@sect $lext:ident $label:tt $current:ident $start:tt $child:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $crate::lexer!(@value $current $start) {
            use $crate::flext::Flext;
            let mut $child = $lext.spawn();
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@sect $lext:ident $label:tt $current:ident $char:tt >> ($action:expr)) => {
        if $crate::lexer!(@value $current $char) {
            $action;
        }
    };

    // Recur

        // For section 1
        (@recur-sect1 $label:tt $out:ident $lext:ident $child:ident $char:tt) => {
            if let Some(current) = $child.current {
                if $crate::lexer!(@value current $char) {
                    $lext = $child.clone();
                    $lext.advance();
                    break $label $crate::token_node::Token {
                        position: $lext.rposition(),
                        token_type: Self::$out,
                    };
                }
            }
        };

        (@recur-sect1 $label:tt $out:ident $lext:ident $child:ident $char:tt $($tail:tt)*) => {
            if let Some(current) = $child.current {
                if $crate::lexer!(@value current $char) {
                    $child.advance();
                    $crate::lexer!(@recur-sect1 $label $out $lext $child $($tail)*);
                }
            }
        };

    // Detailed

    (@det $child:ident $lext:ident $label:tt ck ($current:ident, $val:tt) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $crate::lexer!(@value $current $val) {
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $child:ident $lext:ident $label:tt if ($condition:expr) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $condition {
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $child:ident $lext:ident $label:tt scope $name:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        {
            let mut $name = $child.spawn();
            $name.advance();
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $child:ident $lext:ident $label:tt done $var:ident ($($spec:expr)?)) => {
        $lext = $child.clone();
        break $label $crate::token_node::Token {
            position: $lext.rposition(),
            token_type: Self::$var$(($spec))?,
        };
    };

    (@det $child:ident $lext:ident $label:tt update: ()) => {
        $lext = $child.clone();
    };

    (@det $child:ident $lext:ident $label:tt advance: $current:ident) => {
        $child.advance();
        let $current = $child.current.unwrap_or(' ');
    };

    (@det $child:ident $lext:ident $label:tt advance: ()) => {
        $child.advance();
    };

    (@det $child:ident $lext:ident $label:tt set $var:ident $val:expr) => {
        let mut $var = $val;
    };

    (@det $child:ident $lext:ident $label:tt mut $var:ident $val:expr) => {
        $var = $val;
    };

    (@det $child:ident $lext:ident $label:tt throw $err:ident ($position:expr $(,$spec:tt)?)) => {
        let _: () = $crate::compiler_error!(($err, $position) $($spec)?).throw();
    };

    (@det $child:ident $lext:ident $label:tt rsome $current:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        while let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $child:ident $lext:ident $label:tt rsome ($current:ident, $while_label:tt) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        $while_label: while let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $child:ident $lext:ident $label:tt some $current:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $child:ident $lext:ident $label:tt $invalid:ident $val:tt $var:tt) => {
        compile_error!(concat!("[lexer] invalid detailed instruction `", stringify!($invalid), "`"))
    };

    // Values

    (@value $current:ident [$val:literal]) => {
        $val.contains($current)
    };

    (@value $current:ident $val:literal) => {
        $current == $val
    };

    (@value $current:ident $val:tt) => {
        stringify!($val).contains($current)
    };
}