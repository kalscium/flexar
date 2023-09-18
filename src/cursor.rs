use std::rc::Rc;

/// Tracks the position of a character in a file
#[derive(Clone)]
pub struct Position {
    pub file_name: String,
    pub file_contents: Rc<FileContents>,
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
}

pub struct FileContents(pub Box<[Box<[char]>]>);

impl FileContents {
    pub fn new(contents: String) -> Self {
        Self(contents.split('\n')
            .map(|x| x.chars().collect())
            .collect()
        )
    }

    pub fn get(&self, position: &Position) -> Option<char> {
        let line = match self.0.get(position.ln as usize -1) { // line starts at one instead of zero
            Some(x) => x,
            None => return None,
        };

        line.get(position.ln_idx as usize -1).copied() // line idx starts at one instead of zero
    }
}