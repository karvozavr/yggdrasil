#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn print_hello_world(_py: Python, val: &str) -> PyResult<u64> {
    println!("Hello, Rust! {}", val);

    Ok(42)
}

// add bindings to the generated python module
// N.B: name: "yggdrasilcore" must be the name of the `.so` file
py_module_initializer!(yggdrasilcore, inityggdrasilcore, PyInit_yggdrasilcore, |py, m | {
    m.add(py, "__doc__", "Python bindings for yggdrasil-core Rust library")?;
    m.add(py, "print_hello_world", py_fn!(py, print_hello_world(val: &str)))?;
    Ok(())
});