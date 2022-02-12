

use pest::{self, Parser};

use crate::{sheet_state::SheetState, sheet::CellIdx};


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

pub fn calc(sheet_state: &mut SheetState, text: &str) -> String
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

