

#[macro_export]
macro_rules! old_flexar {
    ([[$struct:ident] $flext:ident: $flext_type:ty] $(
        fn $func:ident $body:tt
    )*) => {
        impl $struct {
            $(pub fn $func($flext: &mut $flext_type) -> Result<Self, $crate::compile_error::CompileError> {
                $flext.advance();
                $crate::old_flexar!(@body $body);
            })*
        }
    };

    (@body {$($type:ident $action:tt;)*}) => {
        $($crate::old_flexar!(@action $type $action);)*
    };

    (@action ok ($ok:expr)) => {
        return Ok($ok);
    };

    (@action err ($err:expr)) => {
        return Err($err);
    };

    (@action switch {
        $($name:ident: $req:expr => $res:expr,)*
        _ => $err:expr,
    }) => {
        return Ok(if false { panic!("not possible") }
        $(else if let Ok($name) = $req { $res; })*
        else { return Err($err) });
    };

    (@action set ($name:ident = $value:expr)) => {
        let mut $name = $value;
    };

    (@action if (($condition:expr) $body:block else $else:expr)) => {
        if $condition {
            return Ok($body);
        } else {
            return Err($else);
        }
    };

    (@action while (($condition:expr) $body:block else ($else:expr) $error:expr)) => {
        while $condition $body
        if !$else { return Err($error) }
    };

    (@action $invalid:ident $whatever:tt) => {
        compile_error!(concat!("Invalid action `", stringify!($invalid), "`!"));
    }
}