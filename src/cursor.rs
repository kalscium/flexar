use std::{ops::Range, rc::Rc};

/// A full position of a string of characters in a file
pub type Position = (Cursor, Cursor);

/// Tracks the position of a character in a file
#[derive(Clone, Debug)]
pub struct Cursor {
    pub file_name: String,
    file_contents: Rc<FileContents>,
    pub abs_idx: u16,
    pub ln: u16,
    pub ln_idx: u16,
}

impl Cursor {
    pub fn new(file_name: String, contents: String) -> Self {
        Self {
            file_name,
            file_contents: Rc::new(FileContents::new(contents)),
            abs_idx: 0,
            ln: 1,
            ln_idx: 1,
        }
    }

    pub fn get_to<'a>(&'a self, other: &'a Self) -> Option<(&'a str, Range<usize>)> {
        // if reversed, reverse it again to not cause any errors
        if self.abs_idx > other.abs_idx {
            return other.get_to(self);
        }
        
        // This only works for one line, change it if needed
        self.get_ln().map(|x| (x, (self.ln_idx as usize -1)..(other.ln_idx as usize -1))) // `ln_idx` starts at idx 1 instead of zero
    }

    #[inline]
    pub fn get_ln(&self) -> Option<&'_ str> {
        self.file_contents.0
            .get(self.ln as usize -1) // line starts at one instead of zero
            .map(|x| x.as_ref())
    }
}

#[derive(Debug)]
pub struct FileContents(pub Box<[Box<str>]>);

impl FileContents {
    #[inline]
    pub fn new(contents: String) -> Self {
        Self(contents.split('\n').map(|x| x.into()).collect())
    }
}
