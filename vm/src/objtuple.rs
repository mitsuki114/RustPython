use super::pyobject::{PyObject, PyObjectKind, PyObjectRef, PyResult};
use super::vm::VirtualMachine;

fn get_pos(l: &Vec<PyObjectRef>, p: i32) -> usize {
    if p < 0 {
        l.len() - ((-p) as usize)
    } else {
        p as usize
    }
}

pub fn get_item(rt: &mut VirtualMachine, l: &Vec<PyObjectRef>, b: PyObjectRef) -> PyResult {
    match &(b.borrow()).kind {
        PyObjectKind::Integer { value } => {
            let pos_index = get_pos(l, *value);
            if pos_index < l.len() {
                let obj = l[pos_index].clone();
                Ok(obj)
            } else {
                Err(rt.new_exception("Index out of bounds!".to_string()))
            }
        }
        PyObjectKind::Slice { start, stop, step } => {
            let start = match start {
                &Some(start) => get_pos(l, start),
                &None => 0,
            };
            let stop = match stop {
                &Some(stop) => get_pos(l, stop),
                &None => l.len() as usize,
            };
            let step = match step {
                //Some(step) => step as usize,
                &None => 1 as usize,
                _ => unimplemented!(),
            };
            // TODO: we could potentially avoid this copy and use slice
            let obj = PyObject::new(
                PyObjectKind::Tuple {
                    elements: l[start..stop].to_vec(),
                },
                rt.get_type(),
            );
            Ok(obj)
        }
        _ => Err(rt.new_exception(format!(
            "TypeError: indexing type {:?} with index {:?} is not supported (yet?)",
            l, b
        ))),
    }
}
