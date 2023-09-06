use std::fmt::Debug;

pub trait Token: Debug {
    fn value(&self) -> &str;
}