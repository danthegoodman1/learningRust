use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // Load the Wasm module binary
    let wasm_bytes = std::fs::read("/Users/dangoodman/code/learningRust/wasm/code/pkg/code_bg.wasm")?;

    // Create an Engine and Store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // Compile the module
    let module = Module::new(&engine, &wasm_bytes)?;

    // Create an import object
    let instance = Instance::new(&mut store, &module, &[])?;

    // Retrieve the `multiply` function from the instance
    let multiply = instance.get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;

    // Execute the function
    let result = multiply.call(&mut store, (3, 4))?;
    println!("3 * 4 = {}", result);

    Ok(())
}
