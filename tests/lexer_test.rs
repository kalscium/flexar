use flexar::{lext::Lext, flext::Flext, token::Token};

flexar::compiler_error! {
    [[Define]]
    (E001) "invalid character": ((1) "`", "` is an invalid character");
    (E002) "string not closed": "expected `\"` to close string";
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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
    Undefined,
}

flexar::lexer! {
    [[TokenType] lext, current, 'cycle]
    else flexar::compiler_error!((E001, lext.position()) current).throw();

    Slash: /;
    Plus: +;
    LParen: '(';
    RParen: ')';
    Dot: .;
    Colon: :;
    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });

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
        throw E002(child.spawn().position());
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
    let tokens = TokenType::tokenize(Lext::new(String::from("example"), contents));
    use TokenType as L;
    assert_tokens(&tokens, &[
        L::Plus,
        L::Slash,
        L::LParen,
        L::Dot,
        L::Colon,
        L::RParen,
        L::Slash,
    ]);
}

#[test]
fn test_multiple() {
    let contents = "=  ==\n=:  ====.==   =====";
    let tokens = TokenType::tokenize(Lext::new(String::from("example"), contents));
    use TokenType as L;
    assert_tokens(&tokens, &[
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
    ]);
}

#[test]
fn test_string() {
    let contents = "+  /\n:( \"hello world?\"). /";
    let tokens = TokenType::tokenize(Lext::new(String::from("example"), contents));
    use TokenType as L;
    assert_tokens(&tokens, &[
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Str("hello world?".into()),
        L::RParen,
        L::Dot,
        L::Slash,
    ]);
}

#[test]
fn test_int() {
    let contents = "+  /\n:( 1234). /";
    let tokens = TokenType::tokenize(Lext::new(String::from("example"), contents));
    use TokenType as L;
    assert_tokens(&tokens, &[
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Int(1234),
        L::RParen,
        L::Dot,
        L::Slash,
    ]);
}

#[test]
fn test_float() {
    let contents = "+  /\n:( 12.34). /";
    let tokens = TokenType::tokenize(Lext::new(String::from("example"), contents));
    use TokenType as L;
    assert_tokens(&tokens, &[
        L::Plus,
        L::Slash,
        L::Colon,
        L::LParen,
        L::Float(12.34),
        L::RParen,
        L::Dot,
        L::Slash,
    ]);
}

fn assert_tokens(tokens: &[Token<TokenType>], expected: &[TokenType]) {
    tokens.iter()
        .enumerate()
        .for_each(|(i, x)| if x.token_type != expected[i] {
            panic!("Expected: {expected:?}\nGot: {:?}", tokens.iter().map(|x| &x.token_type).collect::<Box<[&TokenType]>>())
        });
}