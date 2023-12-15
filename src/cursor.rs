use std::rc::Rc;

/// A mutable cursor for the lexer
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
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Position(pub Rc<Cursor>, pub Rc<Cursor>);

impl Position {
    /// Merges with another position to create a new position that contains both
    #[inline]
    pub fn combine(&self, other: &Position) -> Self {
        Self(self.0.clone(), other.1.clone())
    }

    #[inline]
    pub fn new_oneline(file_name: &str, line: &str, range: Option<(u16, u16)>) -> Self {
        let mut start = Cursor::new(file_name.to_string(), line);
        let mut end = start.clone();

        start.ln_idx = match range {
            Some(x) => x.0,
            None => 1,
        };
        end.ln_idx = match range {
            Some(x) => x.1,
            None => line.len() as u16,
        };

        Position(Rc::new(start), Rc::new(end))
    }
}

impl From<Cursor> for Position {
    fn from(cursor: Cursor) -> Self {
        let cursor = Rc::new(cursor);
        Self(cursor.clone(), cursor)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "<position: {}:{}-{}:{}-{}>",
            self.0.file_name,
            self.0.ln,
            self.1.ln,
            self.0.ln_idx,
            self.0.ln_idx
        )
    }
}

/// Tracks the position of a character in a file
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
            .collect::<Box<[char]>>()
            .get(self.ln_idx as usize -1)
            .copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let line_len = self.get_ln().unwrap().len();

        if line_len == 0 && self.ln_idx == 1 {
            self.ln_idx += 1; return Some('\n');
        }

        if self.ln_idx as usize == line_len { // still have to return end of line
            self.ln_idx += 1; return Some('\n');
        }

        if self.ln_idx as usize > line_len { // if reached end of line
            if self.ln as usize == self.file_contents.0.len() { return None; } // if reached last line
            self.ln += 1;
            self.ln_idx = 0;
            return self.advance();
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

/// Holds the contents of a file
#[derive(Hash, PartialEq, Eq)]
pub struct FileContents(pub Box<[Box<str>]>);

impl FileContents {
    #[inline]
    pub fn new(contents: &str) -> Self {
        Self(contents.split('\n').map(|x| x.into()).collect())
    }
}

impl std::fmt::Debug for FileContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<file contents>")
    }
}
