use std::cell::RefCell;
use std::rc::Rc;

use failure::format_err;
use failure::Error;
use failure::ResultExt;
use rustpython_compiler::compile;
use rustpython_vm::obj::objdict::PyDict;
use rustpython_vm::print_exception;
use rustpython_vm::pyobject::ItemProtocol;
use rustpython_vm::pyobject::PyIterable;
use rustpython_vm::pyobject::PyResult;
use rustpython_vm::VirtualMachine;

pub fn load() -> Result<(), Error> {
    run_rust_python()
}

fn run_rust_python() -> Result<(), Error> {
    let vm = rustpython_vm::VirtualMachine::new(Default::default());
    let scope = vm.new_scope_with_builtins();
    let keys: Rc<RefCell<Vec<String>>> = Default::default();
    let our_keys = keys.clone();
    let callback = vm
        .ctx
        .new_rustfunc(move |items: PyIterable, vm: &VirtualMachine| -> PyResult {
            for item in items.iter(vm)? {
                let item = item?;
                for (k, _v) in item.downcast::<PyDict>()?.into_iter() {
                    keys.borrow_mut().push(vm.to_str(&k)?.as_str().to_string());
                }
            }
            Ok(vm.get_none())
        });
    scope
        .get_locals()
        .set_item("register", callback, &vm)
        .map_err(|e| format_err!("{:?}", e))?;
    let code_obj = vm
        .compile(
            "for i in range(5):\n  register([{'hello': 5, 'world': 'quux'}])",
            compile::Mode::Exec,
            "<virtual>".to_string(),
        )
        //.map_err(|err| vm.new_syntax_error(&err))?;
        .with_context(|_| "compiling")?;

    if let Err(err) = vm.run_code_obj(code_obj, scope) {
        print_exception(&vm, &err);
    }

    println!("{:?}", our_keys);

    Ok(())
}
