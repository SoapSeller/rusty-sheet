use crate::sheet::*;

pub struct SheetState {
    pub selected: CellIdx,
    pub view_offset: CellIdx,
    pub text: String,
    pub sheet: Sheet,
}

impl SheetState {
    pub fn new() -> Self {
        SheetState{selected: CellIdx{col: 0, row: 0}, view_offset: CellIdx{col: 0, row: 0}, text: "".to_string(), sheet: Sheet::new()}
    }
}
