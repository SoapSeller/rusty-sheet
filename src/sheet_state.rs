use std::num::NonZeroI128;

use pest::{self, Parser};

use rustpython::rustpython_vm as pyvm;

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


fn setup_py_module(vm: &pyvm::VirtualMachine) -> pyvm::PyObjectRef {
    let module = vm.new_module("mymodule", vm.ctx.new_dict(), None);

    vm.__module_set_attr(&module, "other_thing",
    vm.ctx.make_funcdef("other_thing",
    other_thing_fun).with_doc("other_thing($module, _)".to_owned(),
                                              &vm.ctx).into_function().with_module(vm.new_pyobj("custom_py".to_owned())).into_ref(&vm.ctx)).unwrap();

    module
}


fn other_thing_fun(_: rustpython_vm::builtins::PyStrRef) -> String { "what?".to_string() }

fn eval_python(_: &SheetState, text: &str) -> String
{
    let interp = pyvm::Interpreter::new_with_init(pyvm::PySettings::default(), |vm| {
        vm.add_native_module("custom_py".to_owned(), Box::new(setup_py_module));

        pyvm::InitParameter::External
    });
        
    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let mut source = "from custom_py import *\n".to_string();
        source.push_str(text);
        match vm.compile(source.as_str(), pyvm::compile::Mode::Single, "<embedded>".to_owned()) {
            Ok(bytecode) => {
                match vm.run_code_obj(bytecode, scope) {
                    Ok(output) => {
                        if vm.is_none(&output) {
                            "".to_string()
                        } else {
                            if output.payload_is::<pyvm::builtins::PyStr>() {
                                output.payload::<pyvm::builtins::PyStr>().unwrap().as_str().to_string()
                            } else if output.payload_is::<pyvm::builtins::PyFloat>() {
                                output.payload::<pyvm::builtins::PyFloat>().unwrap().to_f64().to_string()
                            } else if output.payload_is::<pyvm::builtins::PyInt>() {
                                format!("{}", output.payload::<pyvm::builtins::PyInt>().unwrap())
                            } else {
                                "Unkown Type".to_string()
                            }
                        }
                    },
                    Err(err) => {
                        vm.print_exception(err);
                        "Error".to_string()
                    }         
                }
            }
            Err(err) => {
                vm.print_exception(vm.new_syntax_error(&err)); 
                "Error".to_string()
            }
        }
        // match pyvm::eval::eval(vm, text, scope,&"<embedded>") {
        //     Ok(output) => {
        //         if vm.is_none(&output) {
        //             "".to_string()
        //         } else {
        //             if output.payload_is::<pyvm::builtins::PyStr>() {
        //                 output.payload::<pyvm::builtins::PyStr>().unwrap().as_str().to_string()
        //             } else if output.payload_is::<pyvm::builtins::PyFloat>() {
        //                 output.payload::<pyvm::builtins::PyFloat>().unwrap().to_f64().to_string()
        //             } else if output.payload_is::<pyvm::builtins::PyInt>() {
        //                 format!("{}", output.payload::<pyvm::builtins::PyInt>().unwrap())
        //             } else {
        //                 "Unkown Type".to_string()
        //             }
        //         }
        //     },
        //     Err(err) => {
        //         vm.print_exception(err);
        //         "Error".to_string()
        //     }
        // }
    })
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
        match self.sheet.get(idx) {
            Some(cell) => {
                let text = cell.value.trim();
                if text.is_empty() { return text.to_string(); }
        
                let semi_final = match cell.engine {
                    EngineType::Simple => { calc_value(self, text) },
                    EngineType::Python => { eval_python(self, text) }
                };


                let splt = semi_final.split('\r').collect::<Vec<&str>>();
        
                if splt.len() > 1 {
                    splt[0].to_string()
                } else {
                    semi_final
                }
            },
            None => { "".to_string() }
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

    #[test]
    fn plain_python() {
        let mut state = SheetState::new();

        assert_eq!(state.get_value(&state.selected), "".to_string());

        let cell = Cell{engine: EngineType::Python, value: "'test'".to_string()};

        state.sheet.insert(state.selected.clone(), cell);
        assert_eq!(state.get_value(&state.selected), "test".to_string());

        let cell = Cell{engine: EngineType::Python, value: "6".to_string()};

        state.sheet.insert(state.selected.clone(), cell);
        assert_eq!(state.get_value(&state.selected), "6".to_string());

        let cell = Cell{engine: EngineType::Python, value: "5.2".to_string()};

        state.sheet.insert(state.selected.clone(), cell);
        assert_eq!(state.get_value(&state.selected), "5.2".to_string());


        let cell = Cell{engine: EngineType::Python, value: "other_thing('x')".to_string()};

        state.sheet.insert(state.selected.clone(), cell);
        assert_eq!(state.get_value(&state.selected), "what?".to_string());
    }

}
