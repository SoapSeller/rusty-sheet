use std::{collections::HashMap, ops::Add};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellIdx {
    pub col: u32,
    pub row: u32,
}

impl Add for CellIdx {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// pub struct Cell {
//     pub value: String,
// }

pub struct Sheet {
    cells: HashMap<CellIdx, String>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet{cells: HashMap::new()}
    }

    // pub fn insert(&mut self, idx: CellIdx, value: Cell) {
    //     self.cells.insert(idx, value);
    // }

    // pub fn get(&self, idx: &CellIdx) -> Option<&Cell> {
    //     self.cells.get(idx)
    // }

    pub fn set_text(&mut self, idx: CellIdx, text: String) {
        self.cells.insert(idx, text);
    }

    pub fn get_text(&self, idx: &CellIdx) -> String {
        match self.cells.get(idx) {
            Some(text) => text.clone(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut sheet = Sheet::new();
        let idx = CellIdx{col: 5, row: 3};
        //assert_eq!(sheet.get(&idx), None);
        assert_eq!(sheet.get_text(&idx), "".to_string());

        // let cell = Cell{value: "test".to_string()};
        // sheet.insert(idx.clone(), cell.clone());
        // assert_eq!(sheet.get(&idx), Some(&cell));

        sheet.set_text(idx.clone(), "test".to_string());
        assert_eq!(sheet.get_text(&idx), "test".to_string());
    }
}
