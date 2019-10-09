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

fn builtin_callback(items: PyIterable, vm: &VirtualMachine) -> PyResult {
    for item in items.iter(vm)? {
        let item = item?;
        for (k, v) in item.downcast::<PyDict>()?.into_iter() {
            println!("{:?} -> {:?}", k, v);
        }
        println!("--");
    }
    Ok(vm.get_none())
}

fn run_rust_python() -> Result<(), Error> {
    let vm = rustpython_vm::VirtualMachine::new(Default::default());
    let scope = vm.new_scope_with_builtins();
    let callback = vm.ctx.new_rustfunc(builtin_callback);
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

    Ok(())
}
