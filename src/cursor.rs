use std::rc::Rc;

/// A full position of a string of characters in a file
#[derive(Debug, Clone)]
pub enum Position {
    Single(Rc<Cursor>),
    Mul(Rc<Cursor>, Rc<Cursor>),
}

impl From<Cursor> for Position {
    #[inline]
    fn from(cursor: Cursor) -> Self {
        Self::Single(Rc::new(cursor))
    }
}

impl Position {
    /// Creates a new position from two cursors, if you want a single one then use `cursor.into()`
    #[inline]
    pub fn new(start: Cursor, end: Cursor) -> Self {
        Self::Mul(Rc::new(start), Rc::new(end))
    }

    /// Grabs the starting position
    #[inline]
    pub fn start(&self) -> &Cursor {
        match self {
            Self::Single(x) => x,
            Self::Mul(x, _) => x,
        }
    }

    /// Grabs the end position
    #[inline]
    pub fn end(&self) -> &Cursor {
        match self {
            Self::Single(x) => x,
            Self::Mul(_, x) => x,
        }
    }
}

/// Tracks the position of a character in a file
#[derive(Clone, Debug)]
pub struct Cursor {
    pub file_name: Rc<String>,
    file_contents: Rc<FileContents>,
    pub abs_idx: u16,
    pub ln: u16,
    pub ln_idx: u16,
}

impl Cursor {
    pub fn new(file_name: String, contents: String) -> Self {
        Self {
            file_name: Rc::new(file_name),
            file_contents: Rc::new(FileContents::new(contents)),
            abs_idx: 0,
            ln: 1,
            ln_idx: 1,
        }
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
