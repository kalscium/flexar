use std::rc::Rc;

/// A mutable cursor for parsing / lexing with
#[derive(Debug, Clone)]
pub struct MutCursor {
    pub pos_start: Rc<Cursor>,
    pub pos_end: Rc<Cursor>,
    pub current_char: Option<char>,
}

impl MutCursor {
    /// Constructs a new mutable cursor
    pub fn new(pos_start: Cursor) -> Self {
        let pos_start = Rc::new(pos_start);
        Self {
            pos_start: pos_start.clone(),
            pos_end: pos_start,
            current_char: None,
        }
    }

    /// Constructs an immutable position from the ,`MutCursor`
    #[inline]
    pub fn position(&self) -> Position {
        Position(self.pos_start.clone(), self.pos_end.clone())
    }

    /// Spawns a child `MutCursor`
    pub fn spawn(&self) -> Self {
        Self {
            pos_start: self.pos_end.clone(),
            pos_end: self.pos_end.clone(),
            current_char: self.current_char,
        }
    }

    /// Updates the last position of the `MutCursor`
    #[inline]
    pub fn update(&mut self, cursor: Cursor) {
        self.pos_end = Rc::new(cursor);
    }

    /// Advances through the file
    pub fn advance(&mut self) {
        let mut cursor = self.pos_end.dupe();
        self.current_char = cursor.advance();
        self.update(cursor);
    }

    /// Un-advances through the file
    pub fn revance(&mut self) {
        let mut cursor = self.pos_end.dupe();
        self.current_char = cursor.revance();
        self.update(cursor);
    }
}

/// A full position of a string of characters in a file
#[derive(Debug, Clone)]
pub struct Position(pub Rc<Cursor>, pub Rc<Cursor>);

impl From<Cursor> for Position {
    fn from(cursor: Cursor) -> Self {
        let cursor = Rc::new(cursor);
        Self(cursor.clone(), cursor)
    }
}

/// Tracks the position of a character in a file
#[derive(Clone, Debug)]
pub struct Cursor {
    pub file_name: Rc<String>,
    file_contents: Rc<FileContents>,
    pub ln: u16,
    pub ln_idx: u16,
}

impl Cursor {
    pub fn new(file_name: String, contents: &str) -> Self {
        Self {
            file_name: Rc::new(file_name),
            file_contents: Rc::new(FileContents::new(contents)),
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

    /// Clones it through a Rc
    #[inline]
    pub fn dupe(&self) -> Self {
        self.clone()
    }

    #[inline]
    pub fn get_char(&self) -> Option<char> {
        self.get_ln().unwrap()
            .chars()
            .enumerate()
            .find(|(i, _)| *i == self.ln_idx as usize -1)
            .map(|(_, x)| x)
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.ln_idx as usize == self.get_ln().unwrap().len() { // still have to return end of line
            self.ln_idx += 1; return Some('\n');
        }

        if self.ln_idx as usize > self.get_ln().unwrap().len() { // if reached end of line
            if self.ln as usize == self.file_contents.0.len() { return None; } // if reached last line
            self.ln += 1;
            self.ln_idx = 1;
        } else {
            self.ln_idx += 1;
        }

        self.get_char()
    }

    pub fn revance(&mut self) -> Option<char> {
        if self.ln_idx == 1 { // if reached start of line
            if self.ln == 1 { return None; } // if reached first line
            self.ln -= 1;
            self.ln_idx = self.get_ln().unwrap().len() as u16;
        } else {
            self.ln_idx -= 1;
        }

        self.get_char()
    }
}

#[derive(Debug)]
pub struct FileContents(pub Box<[Box<str>]>);

impl FileContents {
    #[inline]
    pub fn new(contents: &str) -> Self {
        Self(contents.split('\n').map(|x| x.into()).collect())
    }
}
