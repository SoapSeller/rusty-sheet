use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellIdx {
    pub col: u32,
    pub row: u32,
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    pub value: String,
}
    
pub struct Sheet {
    cells: HashMap<CellIdx, Cell>,
}

impl Sheet {
    pub fn new() -> Sheet {
        Sheet{cells: HashMap::new()}
    }

    pub fn insert(&mut self, idx: CellIdx, value: Cell) {
        self.cells.insert(idx, value);
    }

    pub fn get(&self, idx: &CellIdx) -> Option<&Cell> {
        self.cells.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn smoke() {
        let mut sheet = Sheet::new();
        let idx = CellIdx{col: 5, row: 3};
        assert_eq!(sheet.get(&idx), None);

        let cell = Cell{value: "test".to_string()};
        sheet.insert(idx.clone(), cell.clone());
        assert_eq!(sheet.get(&idx), Some(&cell));
    }
}
