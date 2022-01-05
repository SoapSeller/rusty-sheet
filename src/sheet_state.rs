use crate::sheet::*;

pub struct SheetState {
    pub selected: CellIdx,
    pub view_offset: CellIdx,
    pub text: String,
    pub sheet: Sheet,
}

fn calc_value(_sheet: &Sheet, text: String) -> String
{
    text
    // if text.len() < 2 || !text.starts_with('=') { return text; }

    // let mut value = String::new();

    // let slice = &text[1..];

    // value = calc_value(sheet, sheet.get_text(idx)

    // value
}

impl SheetState {
    pub fn new() -> Self {
        SheetState{selected: CellIdx{col: 0, row: 0}, view_offset: CellIdx{col: 0, row: 0}, text: "".to_string(), sheet: Sheet::new()}
    }

    pub fn get_value(&self, idx: &CellIdx) -> String
    {
        let text = self.sheet.get_text(idx);
        if text.is_empty() { return text; }

        calc_value(&self.sheet, text)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_values() {
        let mut state = SheetState::new();

        assert_eq!(state.get_value(&state.selected), "".to_string());

        state.sheet.set_text(state.selected.clone(), "test".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());
    }

    // #[test]
    // fn simple_reference() {
    //     let mut state = SheetState::new();

    //     assert_eq!(state.get_value(&state.selected), "".to_string());

    //     state.sheet.set_text(state.selected.clone(), "test".to_string());
    //     assert_eq!(state.get_value(&state.selected), "test".to_string());

    //     state.selected.col = 1;
    //     state.sheet.set_text(state.selected.clone(), "=A1".to_string());
    //     assert_eq!(state.get_value(&state.selected), "test".to_string());
    // }
}