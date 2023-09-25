use flexar::compiler_error;

compiler_error! {
    [[Define]]
    (E001) "invalid character": ((1) "`", "` is an invalid character");
    (E002) "string not closed": "expected `\"` to close string";
}

#[derive(Debug, PartialEq)]
pub enum Lexer {
    Slash,
    Plus,
    LParen,
    RParen,
    EE,
    EEE,
    EQ,
    Dot,
    Colon,
    Str(String),
    Int(u32),
    Float(f32),
}

flexar::lexer! {
    [[Lexer] flext: Flext, current, 'cycle]
    else compiler_error!((E001, flext.cursor.position()) current).throw();

    Slash: /;
    Plus: +;
    LParen: '(';
    RParen: ')';
    Dot: .;
    Colon: :;
    [" \n\t"] >> ({ flext.advance(); continue 'cycle; });

    // `=` stuff
    EEE: (= = =);
    EE: (= =);
    EQ: =;
    '"' child {
        { child.advance() };
        set string { String::new() };
        rsome current {
            ck (current, '"') {
                { child.advance() };
                done Str(string);
            };
            { string.push(current) };
        };
        throw E002(child.cursor.spawn().position());
    };
    ["0123456789"] child {
        set number { String::new() };
        set dot false;
        rsome (current, 'number) {
            set matched false;
            ck (current, ["0123456789"]) {
                mut matched true;
                { number.push(current) };
            };
            ck (current, '.') {
                if (dot) {
                    done Float(number.parse().unwrap());
                };
                mut matched true;
                mut dot true;
                { number.push(current) };
            };
            {if !matched {break 'number}};
        };
        { println!("{number}") };
        if (dot) { done Float(number.parse().unwrap()); };
        done Int(number.parse().unwrap());
    };
}

#[test]
fn test_single() {
    let contents = "+  /\n(  .:) /";
    let tokens = Lexer::tokenise(Flext::new(String::from("example"), contents));
    use Lexer as L;
    assert_eq!(*tokens, *Box::new([
        L::Plus,
        L::Slash,
        L::LParen,
        L::Dot,
        L::Colon,
        L::RParen,
        L::Slash,
    ]));
}

#[test]
fn test_multiple() {
    let contents = "=  ==\n=:  ====.==   =====";
    let tokens = Lexer::tokenise(Flext::new(String::from("example"), contents));
    use Lexer as L;
    assert_eq!(*tokens, *Box::new([
        L::EQ,
        L::EE,
        L::EQ,
        L::Colon,
        L::EEE,
        L::EQ,
        L::Dot,
        L::EE,
        L::EEE,
        L::EE,
    ]));
}

#[test]
fn test_string() {
    let contents = "+  /\n:( \"hello world?\"). /";
    let tokens = Lexer::tokenise(Flext::new(String::from("example"), contents));
    use Lexer as L;
    assert_eq!(*tokens, *Box::new([
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Str("hello world?".into()),
        L::RParen,
        L::Dot,
        L::Slash,
    ]));
}

#[test]
fn test_int() {
    let contents = "+  /\n:( 1234). /";
    let tokens = Lexer::tokenise(Flext::new(String::from("example"), contents));
    use Lexer as L;
    assert_eq!(*tokens, *Box::new([
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Int(1234),
        L::RParen,
        L::Dot,
        L::Slash,
    ]));
}

#[test]
fn test_float() {
    let contents = "+  /\n:( 12.34). /";
    let tokens = Lexer::tokenise(Flext::new(String::from("example"), contents));
    use Lexer as L;
    assert_eq!(*tokens, *Box::new([
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Float(12.34),
        L::RParen,
        L::Dot,
        L::Slash,
    ]));
}