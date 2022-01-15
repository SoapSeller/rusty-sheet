use pest::{self, Parser};

use crate::sheet::*;

pub struct SheetState {
    pub selected: CellIdx,
    pub view_offset: CellIdx,
    pub text: String,
    pub sheet: Sheet,
}

#[derive(pest_derive::Parser)]
#[grammar = "simple.pest"] // relative to src
struct SimpleParser;

fn str_to_col(s: &str) -> u32 {
    let mut col: u32 = 0;

    let mut mult = s.len() as u32 - 1;
    for c in s.chars() {
        let val = c as u32 - 'A' as u32;
        if mult == 0 {
            col += val;
        } else {
            col += (val+1) * mult * 26;
            mult -= 1;
        }
    }
    col
}

fn calc_value(sheet_state: &SheetState, text: &str) -> String
{
    let parsed = SimpleParser::parse(Rule::Expr, text);
    match parsed {
        Ok(pairs) => {
            for pair in pairs {
                if let Rule::Expr = pair.as_rule() {
                    let pair = pair.into_inner().next().unwrap();
                    let mut pair = pair.into_inner();
                    let alphas = pair.next().unwrap();
                    let digits = pair.next().unwrap();

                    let col = str_to_col(alphas.as_str());
                    let row = digits.as_str().parse::<u32>().unwrap();
                    if row != 0 {
                        let crow = row - 1;
                        return sheet_state.get_value(&CellIdx{row: crow, col});
                    }
                    //calc_value(sheet, sheet.get_text(CellIdx{row: }))
                }
            }
            "Error".to_string()
         },
        _  => { text.to_string() }
    }
}

impl SheetState {
    pub fn new() -> Self {
        SheetState{selected: CellIdx{col: 0, row: 0}, view_offset: CellIdx{col: 0, row: 0}, text: "".to_string(), sheet: Sheet::new()}
    }

    pub fn get_value(&self, idx: &CellIdx) -> String
    {
        let text = self.sheet.get_text(idx);
        let text = text.trim();
        if text.is_empty() { return text.to_string(); }

        let semi_final = calc_value(self, text);
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

        assert_eq!(state.get_value(&state.selected), "".to_string());

        state.sheet.set_text(state.selected.clone(), "test".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());
    }

    #[test]
    fn simple_engine_single_reference() {
        let mut state = SheetState::new();

        assert_eq!(state.get_value(&state.selected), "".to_string());

        state.sheet.set_text(state.selected.clone(), "test".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());

        state.selected.col = 1;
        state.sheet.set_text(state.selected.clone(), "=A1".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());


        let very_large_idx = CellIdx{col: 53, row: 999};
        state.sheet.set_text(very_large_idx, "another test".to_string());
        state.sheet.set_text(state.selected.clone(), "=BB1000".to_string());
        assert_eq!(state.get_value(&state.selected), "another test".to_string());
    }

    #[test]
    fn simple_engine_double_reference() {
        let mut state = SheetState::new();

        assert_eq!(state.get_value(&state.selected), "".to_string());

        state.sheet.set_text(state.selected.clone(), "test".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());

        state.selected.col = 1;
        state.sheet.set_text(state.selected.clone(), "=A1".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());


        state.selected.col = 2;
        state.sheet.set_text(state.selected.clone(), "=B1".to_string());
        assert_eq!(state.get_value(&state.selected), "test".to_string());
    }
}
