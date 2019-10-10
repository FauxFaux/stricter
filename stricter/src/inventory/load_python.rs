use std::cell::RefCell;
use std::rc::Rc;

use failure::err_msg;
use failure::format_err;
use failure::Error;
use failure::ResultExt;
use num_traits::cast::ToPrimitive;
use rustpython_compiler::compile;
use rustpython_vm::obj::objdict::{PyDict, PyDictRef};
use rustpython_vm::obj::objfloat::PyFloat;
use rustpython_vm::obj::objint::PyInt;
use rustpython_vm::obj::objlist::PyList;
use rustpython_vm::obj::objlist::PyListRef;
use rustpython_vm::pyobject::ItemProtocol;
use rustpython_vm::pyobject::PyIterable;
use rustpython_vm::pyobject::PyObjectRef;
use rustpython_vm::pyobject::PyResult;
use rustpython_vm::pyobject::TryFromObject;
use rustpython_vm::pyobject::TypeProtocol;
use rustpython_vm::write_exception;
use rustpython_vm::VirtualMachine;
use serde_json::json;
use serde_json::Value;

pub fn run_rust_python(source: &str) -> Result<Value, Error> {
    let vm = rustpython_vm::VirtualMachine::new(Default::default());
    let scope = vm.new_scope_with_builtins();
    let docs: Rc<RefCell<Vec<Value>>> = Default::default();
    let our_docs = docs.clone();
    let callback =
        vm.ctx
            .new_rustfunc(move |items: PyObjectRef, vm: &VirtualMachine| -> PyResult {
                docs.borrow_mut().push(to_json(items, vm)?);
                Ok(vm.get_none())
            });
    scope
        .get_locals()
        .set_item("register", callback, &vm)
        .map_err(|e| format_err!("{:?}", e))?;

    let code_obj = vm
        .compile(source, compile::Mode::Exec, "<virtual>".to_string())
        .map_err(|err| vm.new_syntax_error(&err))
        .map_err(|err| exception_to_string(err, &vm))
        .with_context(|_| "compiling")?;

    vm.run_code_obj(code_obj, scope)
        .map_err(|err| exception_to_string(err, &vm))
        .with_context(|_| "executing")?;

    let docs = our_docs.borrow_mut();
    Ok(Value::Array(docs.to_vec()))
}

fn exception_to_string(err: PyObjectRef, vm: &VirtualMachine) -> Error {
    let mut out = Vec::with_capacity(1000);
    write_exception(&mut out, &vm, &err).expect("no io errors on Vec");
    err_msg(String::from_utf8_lossy(&out).to_string())
}

fn to_json(obj: PyObjectRef, vm: &VirtualMachine) -> Result<Value, PyObjectRef> {
    let obj = match obj.downcast::<PyInt>() {
        Ok(int) => return Ok(json!(int.as_bigint().to_i64().expect("TODO: bigint"))),
        Err(obj) => obj,
    };

    let obj = match obj.downcast::<PyFloat>() {
        Ok(float) => return Ok(json!(float.to_f64())),
        Err(obj) => obj,
    };

    let obj = match obj.downcast::<PyDict>() {
        Ok(dict) => return to_json_dict(dict, vm),
        Err(obj) => obj,
    };

    let obj = match obj.downcast::<PyList>() {
        Ok(list) => return to_json_array(list, vm),
        Err(obj) => obj,
    };

    Err(vm.new_exception(
        vm.ctx.exceptions.type_error.clone(),
        format!(
            "unexpected type {:?}: {:?}",
            obj.class().name,
            &vm.to_repr(&obj)?.as_str()
        ),
    ))
}

fn to_json_array(list: PyListRef, vm: &VirtualMachine) -> Result<Value, PyObjectRef> {
    let mut ret = Vec::new();
    for item in PyIterable::try_from_object(vm, list.into_object())?.iter(vm)? {
        let item = item?;
        ret.push(to_json(item, vm)?);
    }
    Ok(Value::Array(ret))
}

fn to_json_dict(dict: PyDictRef, vm: &VirtualMachine) -> Result<Value, PyObjectRef> {
    let mut ret = serde_json::Map::new();
    for (k, v) in dict.into_iter() {
        let k = vm.to_str(&k)?;
        let v = to_json(v, vm)?;
        ret.insert(k.to_string(), v);
    }

    Ok(Value::Object(ret))
}
