use std::time::SystemTime;

fn main() {
    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let start = SystemTime::now();
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);

        // Enter the context for compiling and running the hello world script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Create a string containing the JavaScript source code for the 'multiply' function.
        let c_source = r#"
            function multiply(a, b) {
                return a * b;
            }
        "#;

        let source = v8::String::new(scope, c_source).unwrap();

        // Compile the source code.
        let script = v8::Script::compile(scope, source, None).unwrap();

        // Run the script to define the function.
        script.run(scope).unwrap();

        // Get the multiply function from the global object.
        let global = context.global(scope);
        let key = v8::String::new(scope, "multiply").unwrap();
        let multiply_value = global.get(scope, key.into()).unwrap();

        // Ensure it's a function.
        if !multiply_value.is_function() {
            panic!("multiply is not a function");
        }

        let multiply_fn = v8::Local::<v8::Function>::try_from(multiply_value).unwrap();

        // Now we can call the `multiply` function from Rust.
        let arg1 = v8::Number::new(scope, 3.0);
        let arg2 = v8::Number::new(scope, 4.0);
        let args = &[arg1.into(), arg2.into()];
        let result = multiply_fn.call(scope, global.into(), args).unwrap();

        // Convert the result to a number.
        let result = result.to_number(scope).unwrap();
        println!("3 * 4 = {} in {}us", result.value(), start.elapsed().unwrap().as_micros());
    }
}
