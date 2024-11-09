use anyhow::Result;
use component::WasmStr;
use wasmtime::*;

fn main() -> Result<()> {
    let start_time = std::time::Instant::now();

    // Load the Wasm module binary
    let wasm_bytes = std::fs::read("/Users/dangoodman/code/learningRust/wasm/code/pkg/code_bg.wasm")?;
    let load_duration = start_time.elapsed();

    // Create an Engine and Store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let store_duration = start_time.elapsed() - load_duration;

    // Compile the module
    let module = Module::new(&engine, &wasm_bytes)?;
    let compile_duration = start_time.elapsed() - store_duration - load_duration;

    // Create instance and execute function
    let instance = Instance::new(&mut store, &module, &[])?;
    let multiply = instance.get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;
    let result = multiply.call(&mut store, (3, 4))?;
    let execution_duration = start_time.elapsed() - compile_duration - store_duration - load_duration;

    // Serialize the module
    let serialized = module.serialize()?;
    let serialize_duration = start_time.elapsed() - execution_duration - compile_duration - store_duration - load_duration;

    // Deserialize and create new instance
    let deserialized_module = unsafe { Module::deserialize(&engine, &serialized) }?;
    let deserialize_duration = start_time.elapsed() - serialize_duration - execution_duration - compile_duration - store_duration - load_duration;

    // Create second instance and execute
    let instance2 = Instance::new(&mut store, &deserialized_module, &[])?;
    let multiply2 = instance2.get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;
    let result2 = multiply2.call(&mut store, (5, 6))?;
    let second_execution_duration = start_time.elapsed() - deserialize_duration - serialize_duration - execution_duration - compile_duration - store_duration - load_duration;

    // Print all timing results at the end
    println!("Results:");
    println!("  Load time: {:?}", load_duration);
    println!("  Store creation: {:?}", store_duration);
    println!("  Module compilation: {:?}", compile_duration);
    println!("  First instance creation and multiply execution: {:?}", execution_duration);
    println!("  Module serialization: {:?}", serialize_duration);
    println!("  Module deserialization: {:?}", deserialize_duration);
    println!("  Second instance creation and execution: {:?}", second_execution_duration);
    println!("  Total time: {:?}", start_time.elapsed());
    println!("\nCalculation results:");
    println!("  First run:  3 * 4 = {}", result);
    println!("  Second run: 5 * 6 = {}", result2);

    Ok(())
}
