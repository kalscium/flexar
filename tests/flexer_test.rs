use matcha::flexer::{Flext, token::Token};

#[test]
pub fn flext_multipush() {
    #[derive(Clone, Debug)]
    struct MyToken(String);
    impl Token for MyToken {
        fn value(&self) -> &str {
            &self.0
        }
    }

    let ref1: Flext<MyToken> = Flext::new();
    let ref2 = ref1.clone();

    ref1.parse(MyToken(String::from("Value 1")));
    ref2.parse(MyToken(String::from("Value 2")));
}