/// Creates a lexer for tokenising a file
/// # Example
/// ```rust
/// use flexar::*;
/// 
/// compiler_error! {
///    [[Define]]
///    (E001) "invalid character": ((1) "`", "` is an invalid character");
///    (E002) "string not closed": "expected `\"` to close string";
/// }
///
/// #[derive(Debug, Clone, PartialEq)]
/// pub enum TokenType {
///    Slash,
///    Plus,
///    LParen,
///    RParen,
///    EE,
///    EEE,
///    EQ,
///    Dot,
///    Colon,
///    Str(String),
///    Int(u32),
///    Float(f32),
/// }
///
/// lexer! {
///    [[Token, TokenType] lext, current, 'cycle]
///    else compiler_error!((E001, lext.position()) current).throw();
///
///    Slash: /;
///    Plus: +;
///    LParen: '(';
///    RParen: ')';
///    Dot: .;
///    Colon: :;
///    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });
///
///    // `=` stuff
///    EEE: (= = =);
///    EE: (= =);
///    EQ: =;
///    '"' child {
///        { child.advance() };
///        set string { String::new() };
///        rsome current {
///            ck (current, '"') {
///               { child.advance() };
///               done Str(string);
///           };
///           { string.push(current) };
///        };
///        { child.advance() };
///        throw E002(child.position());
///    };
///    ["0123456789"] child {
///        set number { String::new() };
///        set dot false;
///        rsome (current, 'number) {
///            set matched false;
///             ck (current, ["0123456789"]) {
///                 mut matched true;
///                 { number.push(current) };
///             };
///             ck (current, '.') {
///                 if (dot) {
///                     done Float(number.parse().unwrap());
///                 };
///                 mut matched true;
///                 mut dot true;
///                 { number.push(current) };
///             };
///             {if !matched {break 'number}};
///         };
///         if (dot) { done Float(number.parse().unwrap()); };
///         done Int(number.parse().unwrap());
///     };
/// }
#[macro_export]
macro_rules! lexer {
    ([[$token:ident, $token_type:ty] $lext:ident, $current:ident $(, $label:tt)?] else $no_match:expr; $($first:tt$sep:tt$second:tt;)*) => {
        #[derive(Debug, Clone)]
        pub struct $token(
            $crate::cursor::Position,
            $token_type,
        );

        impl $token {
            #[inline]
            pub fn tokenize(mut $lext: $crate::lext::Lext) -> Box<[$token]> {
                <$token_type>::tokenize($lext)
            }
        }

        impl $token_type {
            pub fn tokenize(mut $lext: $crate::lext::Lext) -> Box<[$token]> {
                let mut tokens = Vec::<$token>::new();
                $($label:)? while let Some($current) = $lext.current {
                    tokens.push('code: {
                        $($crate::lexer!(@sect $token $lext 'code $current $first$sep$second);)*
                        $no_match
                    });
                    $lext.cursor.pos_start = $lext.cursor.pos_end.clone(); // cause different tokens with different start pos
                } tokens.into_boxed_slice()
            }
        }
    };
    
    // Sections
    
    (@sect $token:ident $lext:ident $label:tt $current:ident $out:ident: ($char:tt $($tail:tt)*)) => { // Change to something more efficient if too slow
        if $crate::lexer!(@value $current $char) {
            use $crate::flext::Flext;
            let mut child = $lext.spawn();
            child.advance();
            $crate::lexer!(@recur-sect1 $token $label $out $lext child $($tail)*);
        }
    };

    (@sect $token:ident $lext:ident $label:tt $current:ident $name:ident: $char:tt) => {
        if $crate::lexer!(@value $current $char) {
            use $crate::flext::Flext;
            $lext.advance();
            break $label $token($lext.rposition(), Self::$name);
        }
    };

    (@sect $token:ident $lext:ident $label:tt $current:ident $start:tt $child:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $crate::lexer!(@value $current $start) {
            use $crate::flext::Flext;
            let mut $child = $lext.spawn();
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@sect $token:ident $lext:ident $label:tt $current:ident $char:tt >> ($action:expr)) => {
        if $crate::lexer!(@value $current $char) {
            $action;
        }
    };

    // Recur

        // For section 1
        (@recur-sect1 $token:ident $label:tt $out:ident $lext:ident $child:ident $char:tt) => {
            if let Some(current) = $child.current {
                if $crate::lexer!(@value current $char) {
                    $lext = $child;
                    $lext.advance();
                    break $label $token($lext.rposition(), Self::$out);
                }
            }
        };

        (@recur-sect1 $token:ident $label:tt $out:ident $lext:ident $child:ident $char:tt $($tail:tt)*) => {
            if let Some(current) = $child.current {
                if $crate::lexer!(@value current $char) {
                    let mut child = $child.spawn();
                    child.advance();
                    $crate::lexer!(@recur-sect1 $token $label $out $lext child $($tail)*);
                }
            }
        };

    // Detailed

    (@det $token:ident $child:ident $lext:ident $label:tt ck ($current:ident, $val:tt) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $crate::lexer!(@value $current $val) {
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt if ($condition:expr) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if $condition {
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt scope $name:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        {
            let mut $name = $child.spawn();
            $name.advance();
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt done $var:ident ($($spec:expr)?)) => {
        $lext = $child;
        break $label $token($lext.rposition(), Self::$var$(($spec))?);
    };

    (@det $token:ident $child:ident $lext:ident $label:tt set $var:ident $val:expr) => {
        let mut $var = $val;
    };

    (@det $token:ident $child:ident $lext:ident $label:tt mut $var:ident $val:expr) => {
        $var = $val;
    };

    (@det $token:ident $child:ident $lext:ident $label:tt throw $err:ident ($position:expr $(,$spec:tt)?)) => {
        let _: () = $crate::compiler_error!(($err, $position) $($spec)?).throw();
    };

    (@det $token:ident $child:ident $lext:ident $label:tt rsome $current:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        while let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt rsome ($current:ident, $while_label:tt) {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        $while_label: while let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt some $current:ident {$($($code:block)? $($key:ident $param:tt $body:tt)?;)*}) => {
        if let Some($current) = $child.current {
            $(
                $($crate::lexer!(@det $token $child $lext $label $key $param $body);)?
                $($code;)?
            )*
            $child.advance();
        }
    };

    (@det $token:ident $child:ident $lext:ident $label:tt $invalid:ident $val:tt $var:tt) => {
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