use std::{ffi::c_void, ptr, time::Instant};

use rusqlite::{params_from_iter, types::ValueRef, Connection, ToSql};
use serde_json::{json, Map, Value};
use v8::{HeapStatistics, PropertyFilter};

fn main() {
    let total_start = Instant::now();
    
    // Initialize V8.
    let v8_init_start = Instant::now();
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    println!("V8 initialization took: {:?}", v8_init_start.elapsed());

    {
        // Create isolate
        let isolate_start = Instant::now();
        let mb = 1 << 20;
        let mut isolate = &mut v8::Isolate::new(v8::CreateParams::default().heap_limits(mb, 10 * mb));
        println!("Isolate creation took: {:?}", isolate_start.elapsed());

        extern "C" fn oom_handler(_: *const std::os::raw::c_char, _: &v8::OomDetails) {
            panic!("OOM!")
        }
        isolate.set_oom_error_handler(oom_handler);

        extern "C" fn heap_limit_callback(
            data: *mut c_void,
            current_heap_limit: usize,
            _initial_heap_limit: usize,
        ) -> usize {
            // let state = unsafe { &mut *(data as *mut TestHeapLimitState) };
            // state.near_heap_limit_callback_calls += 1;
            let isolate = unsafe { &mut *(data as *mut v8::Isolate) };
            let mut stats = <HeapStatistics as std::default::Default>::default();
            isolate.get_heap_statistics(&mut stats);
            println!(
                "used heap size: {}, total heap size: {}",
                stats.used_heap_size(),
                stats.total_heap_size()
            );
            let terminated = isolate.terminate_execution();
            println!("near limit! {:?}", terminated);
            // murder the isolate
            current_heap_limit * 2 // give us some space to kill it
        }
        let isolate_ptr: &mut v8::Isolate = &mut isolate;

        // Cast the isolate pointer to *mut c_void
        let data: *mut c_void = isolate_ptr as *mut v8::Isolate as *mut c_void;
        isolate.add_near_heap_limit_callback(heap_limit_callback, data);

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context_start = Instant::now();
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);
        let mut scope = v8::TryCatch::new(scope);
        println!("Context creation took: {:?}", context_start.elapsed());

        // Define the `query` function in Rust.
        fn query(
            scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            mut rv: v8::ReturnValue,
        ) {
            // Get the first argument as a string
            let query_str = args.get(0);
            let query_str = query_str
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope);

            // Get the second argument as an array
            let query_params = args.get(1);
            let query_params = v8::Local::<v8::Array>::try_from(query_params).unwrap();
            // let mut params = vec![];

            for i in 0..query_params.length() {
                let elem = query_params.get_index(scope, i).unwrap();
                // params.push(elem);

                // Determine and print the type of each element
                if elem.is_string() {
                    let str_val = elem.to_string(scope).unwrap().to_rust_string_lossy(scope);
                    println!("Param {}: String - `{}`", i, str_val);
                } else if elem.is_int32() {
                    let num_val = elem.to_int32(scope).unwrap().value();
                    println!("Param {}: Int - `{}`", i, num_val);
                } else if elem.is_number() {
                    let num_val = elem.to_number(scope).unwrap().value();
                    println!("Param {}: Float - `{}`", i, num_val);
                } else if elem.is_boolean() {
                    let bool_val = elem.to_boolean(scope).is_true();
                    println!("Param {}: Boolean - `{}`", i, bool_val);
                } else if elem.is_array() {
                    println!("Param {}: Array", i);
                } else if elem.is_object() {
                    println!("Param {}: Object", i);
                } else if elem.is_null_or_undefined() {
                    println!("Param {}: Null or Undefined", i);
                } else {
                    println!("Param {}: Unknown type", i);
                }
            }

            let mut params: Vec<rusqlite::types::Value> = vec![];

            for i in 0..query_params.length() {
                let elem = query_params.get_index(scope, i).unwrap();

                // Convert V8 values to rust-sqlite compatible types
                if elem.is_string() {
                    let str_val = elem.to_string(scope).unwrap().to_rust_string_lossy(scope);
                    params.push(rusqlite::types::Value::Text(str_val));
                } else if elem.is_int32() {
                    let num_val = elem.to_int32(scope).unwrap().value();
                    params.push(rusqlite::types::Value::Integer(num_val.into()));
                } else if elem.is_number() {
                    let num_val = elem.to_number(scope).unwrap().value();
                    params.push(rusqlite::types::Value::Real(num_val));
                } else if elem.is_boolean() {
                    let bool_val = elem.to_boolean(scope).is_true();
                    params.push(rusqlite::types::Value::Integer(bool_val.into()));
                } else if elem.is_null_or_undefined() {
                    params.push(rusqlite::types::Value::Null);
                } else {
                    println!("Param {}: Unsupported type", i);
                }
            }

            // Here you can perform whatever action you need with the query and params
            println!("Query: {}", query_str);
            for param in &params {
                println!("\tParam: {:?}", param.to_sql().unwrap());
            }

            let conn = Connection::open_in_memory().unwrap();
            let mut stmt = conn
                .prepare("with blah as (values (?, ?, ?, ?)) SELECT * FROM blah")
                .unwrap();
            let cols: Vec<String> = stmt.column_names().iter().map(|&s| s.to_string()).collect();
            println!("cols: {:?}", cols);
            let mut rows = stmt
                .query(params_from_iter(
                    params
                        .iter()
                        .map(|f| f.clone())
                        .collect::<Vec<rusqlite::types::Value>>(),
                ))
                .unwrap(); // cloning just so I can reuse the value on the response of the function call below

            // Collect JSON objects into an array
            let mut json_array: Vec<Map<String, Value>> = Vec::new();

            while let Some(row) = rows.next().unwrap() {
                let mut json_object = serde_json::Map::new();

                let cols = row.as_ref().column_names();

                for (idx, col) in cols.iter().enumerate() {
                    let value = match row.get_ref(idx).unwrap() {
                        ValueRef::Null => Value::Null,
                        ValueRef::Integer(i) => Value::Number(serde_json::Number::from(i)),
                        ValueRef::Real(r) => {
                            Value::Number(serde_json::Number::from_f64(r).unwrap())
                        }
                        ValueRef::Text(t) => Value::String(String::from_utf8(t.to_vec()).unwrap()),
                        // ValueRef::Blob(b) => Value::String(String::from_utf8(b.to_vec()).unwrap()), // would b64 encode
                        ValueRef::Blob(b) => json!(b.to_vec()),
                        // ValueRef::Blob(b) => {
                        //     Value::Array(b.iter().map(|&f| Value::Number(f.into())).collect())
                        // }
                    };
                    json_object.insert(col.to_string(), value);
                }

                json_array.push(json_object)
            }

            // Print the JSON array
            println!("{}", serde_json::to_string_pretty(&json_array).unwrap());

            // Return a result back to JavaScript (for example, the length of params)
            // let result = v8::Number::new(scope, params.len() as f64);
            // rv.set(result.into());
            // rv.set(v8::Number::new(scope, 3 as f64).into());

            let fake_rows = vec![1, 2, 3];
            let arr = v8::Array::new(scope, fake_rows.len() as i32);
            for (index, &v) in fake_rows.iter().enumerate() {
                let obj = v8::Object::new(scope);
                let key = v8::String::new(scope, "val").unwrap();
                let val = v8::Number::new(scope, v as f64);
                let index = v8::Number::new(scope, index as f64);
                obj.set(scope, key.into(), val.into());
                arr.set(scope, index.into(), obj.into());
            }

            rv.set(arr.into())
        }

        // Create a function template
        let query_tmpl = v8::FunctionTemplate::new(&mut scope, query);

        // Convert the function template to a function
        let query_fn = query_tmpl.get_function(&mut scope).unwrap();

        // Add the function to the global object
        let global = context.global(&mut scope);
        let query_key = v8::String::new(&mut scope, "query").unwrap();
        global.set(&mut scope, query_key.into(), query_fn.into());

        fn console_log(
            scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            mut _rv: v8::ReturnValue,
        ) {
            let mut line: Vec<String> = Vec::with_capacity(args.length() as usize);
            for i in 0..args.length() {
                let s = args.get(i).to_rust_string_lossy(scope);
                line.push(s);
            }
            println!("LOG: {}", line.join(" "));
        }
        fn console_error(
            scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            mut _rv: v8::ReturnValue,
        ) {
            let mut line: Vec<String> = Vec::with_capacity(args.length() as usize);
            for i in 0..args.length() {
                let s = args.get(i).to_rust_string_lossy(scope);
                line.push(s);
            }
            println!("ERROR: {}", line.join(" "));
        }

        // Create the `console` object
        let console = v8::Object::new(&mut scope);

        // Create a function template
        let console_log_tmpl = v8::FunctionTemplate::new(&mut scope, console_log);

        // Convert the function template to a function
        let console_log_fn = console_log_tmpl.get_function(&mut scope).unwrap();

        // Attach the `log` function to the `console` object
        let log_key = v8::String::new(&mut scope, "log").unwrap();
        console.set(&mut scope, log_key.into(), console_log_fn.into());

        // Create a function template
        let console_error_tmpl = v8::FunctionTemplate::new(&mut scope, console_error);

        // Convert the function template to a function
        let console_error_fn = console_error_tmpl.get_function(&mut scope).unwrap();

        // Attach the `log` function to the `console` object
        let log_key = v8::String::new(&mut scope, "error").unwrap();
        console.set(&mut scope, log_key.into(), console_error_fn.into());

        // Add the `console` object to the global object
        let console_key = v8::String::new(&mut scope, "console").unwrap();
        global.set(&mut scope, console_key.into(), console.into());

        // Create a string containing the JavaScript source code for MyClass.
        let c_source = r#"
            class MyClass {
                multiply(a, b) {
                    let z = []
                    // Comment this in to OOM
                    // while (true) {
                    //     z.push("THIS IS A VERY LONG STRING")
                    // }
                    return a * b;
                }

                testQuery() {
                    let a = query("SELECT * FROM data", [1, 2.1, 'test', true]);
                    console.log("hey", a)
                    console.error("hey", JSON.stringify(a), JSON.parse(JSON.stringify(a)))
                    return a
                }
            }
            this.MyClass = MyClass;"#;

        // Compile and run MyClass
        let compile_start = Instant::now();
        let source = v8::String::new(&mut scope, c_source).unwrap();
        let script = v8::Script::compile(&mut scope, source, None).unwrap();
        script.run(&mut scope).unwrap();
        println!("Compiling and running MyClass took: {:?}", compile_start.elapsed());

        // Create instance and run multiply
        let multiply_start = Instant::now();
        let global = context.global(&mut scope);
        let key = v8::String::new(&mut scope, "MyClass").unwrap();
        let class_value = global.get(&mut scope, key.into()).unwrap();

        // Ensure it's a function (constructor).
        if !class_value.is_function() {
            panic!("MyClass is not a function");
        }

        let class_constructor = v8::Local::<v8::Function>::try_from(class_value).unwrap();

        // Get the prototype of the class
        let proto_key = v8::String::new(&mut scope, "prototype").unwrap().into();
        let prototype = class_constructor
            .get(&mut scope, proto_key)
            .unwrap();
        let prototype_object = v8::Local::<v8::Object>::try_from(prototype).unwrap();

        // Get the property names of the prototype object
        let property_names = prototype_object.get_own_property_names(&mut scope, v8::GetPropertyNamesArgs { mode: v8::KeyCollectionMode::IncludePrototypes, property_filter: PropertyFilter::ALL_PROPERTIES, index_filter: v8::IndexFilter::IncludeIndices, key_conversion: v8::KeyConversionMode::ConvertToString }).unwrap();

        println!("Instance methods:");
        for i in 0..property_names.length() {
            let key = property_names.get_index(&mut scope, i).unwrap();
            let key_str = key.to_string(&mut scope).unwrap();
            println!(" - {}", key_str.to_rust_string_lossy(&mut scope));
        }

        // Create an instance of MyClass.
        let instance = class_constructor.new_instance(&mut scope, &[]).unwrap();

        // Get the multiply method from the instance.
        let multiply_key = v8::String::new(&mut scope, "multiply").unwrap();
        let multiply_value = instance.get(&mut scope, multiply_key.into()).unwrap();

        // Ensure it's a function.
        if !multiply_value.is_function() {
            panic!("multiply is not a function");
        }

        let multiply_fn = v8::Local::<v8::Function>::try_from(multiply_value).unwrap();

        // Now we can call the `multiply` method on the instance from Rust.
        let arg1 = v8::Number::new(&mut scope, 3.0);
        let arg2 = v8::Number::new(&mut scope, 4.0);
        let args = &[arg1.into(), arg2.into()];
        let result = match multiply_fn.call(&mut scope, instance.into(), args) {
            Some(result) => {
                println!("result");
                result
            }
            None => {
                println!(
                    "Has caught: {}, can continue: {}",
                    scope.has_caught(),
                    scope.can_continue()
                );
                panic!("exiting now")
            }
        };

        // Convert the result to a number.
        let result = result.to_number(&mut scope).unwrap();
        println!("3 * 4 = {}", result.value());
        println!("Multiply execution took: {:?}", multiply_start.elapsed());

        // Test calling the query function from JavaScript.
        let query_start = Instant::now();
        let test_query_key = v8::String::new(&mut scope, "testQuery").unwrap();
        let test_query_value = instance.get(&mut scope, test_query_key.into()).unwrap();
        let test_query_fn = v8::Local::<v8::Function>::try_from(test_query_value).unwrap();
        let result = test_query_fn
            .call(&mut scope, instance.into(), &[])
            .unwrap();
        println!("Returned array: {:?}", result.is_array());

        if result.is_array() {
            let arr = v8::Local::<v8::Array>::try_from(result).unwrap();
            for i in 0..arr.length() {
                let key = v8::Number::new(&mut scope, i as f64);
                let elem = arr.get(&mut scope, key.into()).unwrap();

                if elem.is_string() {
                    let str_val = elem
                        .to_string(&mut scope)
                        .unwrap()
                        .to_rust_string_lossy(&mut scope);
                    println!("Array item {}: String - `{}`", i, str_val);
                } else if elem.is_int32() {
                    let num_val = elem.to_int32(&mut scope).unwrap().value();
                    println!("Array item {}: Int - `{}`", i, num_val);
                } else if elem.is_number() {
                    let num_val = elem.to_number(&mut scope).unwrap().value();
                    println!("Array item {}: Float - `{}`", i, num_val);
                } else if elem.is_boolean() {
                    let bool_val = elem.to_boolean(&mut scope).is_true();
                    println!("Array item {}: Boolean - `{}`", i, bool_val);
                } else if elem.is_array() {
                    println!("Array item {}: Array", i);
                } else if elem.is_object() {
                    println!(
                        "Array item {}: Object - {}",
                        i,
                        v8::json::stringify(&mut scope, elem)
                            .unwrap()
                            .to_rust_string_lossy(&mut scope)
                    );
                } else if elem.is_null_or_undefined() {
                    println!("Array item {}: Null or Undefined", i);
                } else {
                    println!("Array item {}: Unknown type", i);
                }
            }
        }

        println!("Query execution took: {:?}", query_start.elapsed());
    }

    println!("Total execution time: {:?}", total_start.elapsed());

    // Explicit disposal of V8 platform is not necessary.
}
