use std::{ops::Range, rc::Rc};

/// Tracks the position of a character in a file
#[derive(Clone)]
pub struct Position {
    pub file_name: String,
    file_contents: Rc<FileContents>,
    pub abs_idx: u16,
    pub ln: u16,
    pub ln_idx: u16,
}

impl Position {
    pub fn new(file_name: String, contents: String) -> Self {
        Self {
            file_name,
            file_contents: Rc::new(FileContents::new(contents)),
            abs_idx: 0,
            ln: 1,
            ln_idx: 1,
        }
    }

    // /// For the flexer
    // pub fn advance(&mut self) -> Option<char> {
    //     match self.file_contents.get(self) {
    //         Some(x) => {
    //             self.ln_idx += 1;
    //             self.abs_idx += 1;
    //             return Some(x);
    //         },
    //         None => (),
    //     };

    //     // Try to increase line number to see if that helps
    //     let mut position = self.clone();
    //     position.ln += 1;
    //     position.ln_idx = 1;
    // }

    pub fn get_to<'a>(&'a self, other: &'a Self) -> Option<(&'a str, Range<usize>)> {
        // if reversed, reverse it again to not cause any errors
        if self.abs_idx > other.abs_idx {
            return other.get_to(self);
        }
        
        // This only works for one line, change it if needed
        self.get_ln().map(|x| (x, (self.ln_idx as usize -1)..(other.ln_idx as usize -1))) // `ln_idx` starts at idx 1 instead of zero
    }

    #[inline]
    pub fn get_ln<'a>(&'a self) -> Option<&'a str> {
        self.file_contents.0
            .get(self.ln as usize -1) // line starts at one instead of zero
            .map(|x| x.as_ref())
    }
}

pub struct FileContents(pub Box<[Box<str>]>);

impl FileContents {
    #[inline]
    pub fn new(contents: String) -> Self {
        Self(contents.split('\n').map(|x| x.into()).collect())
    }
}
