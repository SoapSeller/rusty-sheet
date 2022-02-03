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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EngineType {
    Simple,
    Python
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    pub engine: EngineType,
    pub value: String,
}

pub struct Sheet {
    cells: HashMap<CellIdx, Cell>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet{cells: HashMap::new()}
    }

    pub fn insert(&mut self, idx: CellIdx, value: Cell) {
        self.cells.insert(idx, value);
    }

    pub fn get(&self, idx: &CellIdx) -> Option<&Cell> {
        self.cells.get(idx)
    }

    pub fn set_text(&mut self, idx: CellIdx, value: String) {
        let engine = if let Some(current) = self.cells.get(&idx) {
            current.engine.clone()
        } else {
            EngineType::Simple
        };
        self.cells.insert(idx, Cell{engine, value});
    }

    pub fn get_text(&self, idx: &CellIdx) -> String {
        match self.cells.get(idx) {
            Some(text) => text.value.clone(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_text() {
        let mut sheet = Sheet::new();
        let idx = CellIdx{col: 5, row: 3};
        assert_eq!(sheet.get_text(&idx), "".to_string());

        sheet.set_text(idx.clone(), "test".to_string());
        assert_eq!(sheet.get_text(&idx), "test".to_string());
    }

    #[test]
    fn engine() {
        let mut sheet = Sheet::new();
        let idx = CellIdx{col: 5, row: 3};
        assert_eq!(sheet.get(&idx), None);

        let cell = Cell{engine: EngineType::Python, value: "test".to_string()};
        sheet.insert(idx.clone(), cell.clone());
        assert_eq!(sheet.get(&idx), Some(&cell));

    }
}
