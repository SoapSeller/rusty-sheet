
use crate::{sheet::*, engine_simple, engine_python};

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

    pub fn get_value(&mut self, idx: &CellIdx) -> String
    {
        let (text, engine) = match self.sheet.get(idx) {
                Some(cell) => {
                    let text = cell.value.trim();
                    if text.is_empty() { return "".to_string(); }

                    ( text.to_string(), cell.engine )

                },
                None => { return "".to_string(); }
        };

        let semi_final = match engine {
            EngineType::Simple => { engine_simple::calc(self, text.as_str()) },
            EngineType::Python => { engine_python::calc(self, text.as_str()) }
        };

        let splt = semi_final.split('\r').collect::<Vec<&str>>();
        if splt.len() > 1 {
            splt[0].to_string()
        } else {
            semi_final
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_values() {
        let mut state = SheetState::new();

        let idx = state.selected.clone();

        assert_eq!(state.get_value(&idx), "".to_string());

        state.sheet.set_text(idx.clone(), "test".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());
    }

    // TODO: Fix this test...
    // #[test]
    // fn simple_engine_self_reference() {
    //     let mut state = SheetState::new();
    //     let mut idx = state.selected.clone();

    //     assert_eq!(state.get_value(&idx), "".to_string());

    //     state.sheet.set_text(idx.clone(), "=A1".to_string());
    //     assert_eq!(state.get_value(&idx), "test".to_string());
    // }

    #[test]
    fn simple_engine_single_reference() {
        let mut state = SheetState::new();
        let mut idx = state.selected.clone();

        assert_eq!(state.get_value(&idx), "".to_string());

        state.sheet.set_text(state.selected.clone(), "test".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());

        idx.col = 1;
        state.sheet.set_text(idx.clone(), "=A1".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());


        let very_large_idx = CellIdx{col: 53, row: 999};
        state.sheet.set_text(very_large_idx, "another test".to_string());
        state.sheet.set_text(idx.clone(), "=BB1000".to_string());
        assert_eq!(state.get_value(&idx), "another test".to_string());
    }

    #[test]
    fn simple_engine_double_reference() {
        let mut state = SheetState::new();
        let mut idx = state.selected.clone();


        assert_eq!(state.get_value(&idx), "".to_string());

        state.sheet.set_text(idx.clone(), "test".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());

        idx.col = 1;
        state.sheet.set_text(idx.clone(), "=A1".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());


        idx.col = 2;
        state.sheet.set_text(idx.clone(), "=B1".to_string());
        assert_eq!(state.get_value(&idx), "test".to_string());
    }

    #[test]
    fn plain_python() {
        let mut state = SheetState::new();
        let mut idx = state.selected.clone();


        assert_eq!(state.get_value(&idx), "".to_string());

        let cell = Cell{engine: EngineType::Python, value: "'test'".to_string()};

        state.sheet.insert(idx.clone(), cell);
        assert_eq!(state.get_value(&idx), "test".to_string());

        let cell = Cell{engine: EngineType::Python, value: "6".to_string()};

        state.sheet.insert(idx.clone(), cell);
        assert_eq!(state.get_value(&idx), "6".to_string());

        let cell = Cell{engine: EngineType::Python, value: "5.2".to_string()};

        state.sheet.insert(idx.clone(), cell);
        assert_eq!(state.get_value(&idx), "5.2".to_string());


        let cell = Cell{engine: EngineType::Python, value: "cell(sheet, 'A1')".to_string()};

        idx.col = 1;
        state.sheet.insert(idx.clone(), cell);
        assert_eq!(state.get_value(&idx), "5.2".to_string());
    }


}
