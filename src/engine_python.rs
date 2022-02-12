
use crate::engine_simple;
use crate::sheet_state::SheetState;

use pyo3::prelude::*;
//use pyo3::types::IntoPyDict;
use pyo3::types::PyDict;
use pyo3::types::PyString;

#[pyfunction]
fn cell(py: Python<'_>, sheet: SheetWrapper, input: String) -> PyResult<&PyString> {
    // // We release the GIL here so any other Python threads get a chance to run.
    // py.allow_threads(move || {
    //     // An example of an "expensive" Rust calculation
    //     let sum = numbers.iter().sum();

    //     Ok(sum)
    // })

    //py.

    let text;
    unsafe {
        let ref_sheet: &mut SheetState = &mut *sheet.state_ptr;
        let str = "=".to_string() + &input;
        text = engine_simple::calc(ref_sheet, str.as_str())
    }

    let str = PyString::new(py, text.as_str());


    Ok(str)
}

#[pyclass(unsendable)]
#[derive(Clone)]
struct SheetWrapper {
     state_ptr: *mut SheetState
}


pub fn calc(sheet_state: &mut SheetState, text: &str) -> String {
    let res: PyResult<String> = Python::with_gil(|py| {
        // let sys = py.import("sys")?;
        // let version: String = sys.getattr("version")?.extract()?;

        // let locals = [("os", py.import("os")?)].into_py_dict(py);
        // let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        // let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        // println!("Hello {}, I'm Python {}", user, version);
        // Ok(user)


        let locals = PyDict::new(py);

        let fun = pyo3::wrap_pyfunction!(cell, py)?;
        locals.set_item("cell", fun)?;

        //let obj: &PyAny = Py::new(py, SheetWrapper { &state })?.into_ref(py);
        let obj = SheetWrapper{state_ptr: sheet_state as *mut SheetState};
        locals.set_item("sheet", obj.into_py(py))?;
        //let res = fun.call1((vec![1_u32, 2, 3],))?;
        println!("Evaluating \"{:}\"", text);
        let res: String = py.eval(text, None, Some(locals))?.str()?.to_string();

        Ok(res)
    });

    match res {
        Ok(str) => str,
        _ => "Error".to_string()
    }

}
