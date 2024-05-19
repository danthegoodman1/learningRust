use std::any::Any;

use rusqlite::{params_from_iter, types::ValueRef, Connection, ToSql};
use serde_json::{json, Map, Value};

fn main() {
    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);

        // Enter the context for compiling and running scripts.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

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

            let fake_rows = vec![1,2,3];
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
        let query_tmpl = v8::FunctionTemplate::new(scope, query);

        // Convert the function template to a function
        let query_fn = query_tmpl.get_function(scope).unwrap();

        // Add the function to the global object
        let global = context.global(scope);
        let query_key = v8::String::new(scope, "query").unwrap();
        global.set(scope, query_key.into(), query_fn.into());

        // Create a string containing the JavaScript source code for MyClass.
        let c_source = r#"
            class MyClass {
                multiply(a, b) {
                    return a * b;
                }

                testQuery() {
                    return query("SELECT * FROM data", [1, 2.1, 'test', true]);
                }
            }
            this.MyClass = MyClass;"#;

        let source = v8::String::new(scope, c_source).unwrap();

        // Compile the source code.
        let script = v8::Script::compile(scope, source, None).unwrap();

        // Run the script to define the class.
        script.run(scope).unwrap();

        // Get the MyClass constructor from the global object.
        let global = context.global(scope);
        let key = v8::String::new(scope, "MyClass").unwrap();
        let class_value = global.get(scope, key.into()).unwrap();

        // Ensure it's a function (constructor).
        if !class_value.is_function() {
            panic!("MyClass is not a function");
        }

        let class_constructor = v8::Local::<v8::Function>::try_from(class_value).unwrap();

        // Create an instance of MyClass.
        let instance = class_constructor.new_instance(scope, &[]).unwrap();

        // Get the multiply method from the instance.
        let multiply_key = v8::String::new(scope, "multiply").unwrap();
        let multiply_value = instance.get(scope, multiply_key.into()).unwrap();

        // Ensure it's a function.
        if !multiply_value.is_function() {
            panic!("multiply is not a function");
        }

        let multiply_fn = v8::Local::<v8::Function>::try_from(multiply_value).unwrap();

        // Now we can call the `multiply` method on the instance from Rust.
        let arg1 = v8::Number::new(scope, 3.0);
        let arg2 = v8::Number::new(scope, 4.0);
        let args = &[arg1.into(), arg2.into()];
        let result = multiply_fn.call(scope, instance.into(), args).unwrap();

        // Convert the result to a number.
        let result = result.to_number(scope).unwrap();
        println!("3 * 4 = {}", result.value());

        // Test calling the query function from JavaScript.
        let test_query_key = v8::String::new(scope, "testQuery").unwrap();
        let test_query_value = instance.get(scope, test_query_key.into()).unwrap();
        let test_query_fn = v8::Local::<v8::Function>::try_from(test_query_value).unwrap();
        let result = test_query_fn.call(scope, instance.into(), &[]).unwrap();
        println!("Returned array: {:?}", result.is_array());

        if result.is_array() {
            let arr = v8::Local::<v8::Array>::try_from(result).unwrap();
            for i in 0..arr.length() {
                let key = v8::Number::new(scope, i as f64);
                let elem = arr.get(scope, key.into()).unwrap();

                if elem.is_string() {
                    let str_val = elem.to_string(scope).unwrap().to_rust_string_lossy(scope);
                    println!("Array item {}: String - `{}`", i, str_val);
                } else if elem.is_int32() {
                    let num_val = elem.to_int32(scope).unwrap().value();
                    println!("Array item {}: Int - `{}`", i, num_val);
                } else if elem.is_number() {
                    let num_val = elem.to_number(scope).unwrap().value();
                    println!("Array item {}: Float - `{}`", i, num_val);
                } else if elem.is_boolean() {
                    let bool_val = elem.to_boolean(scope).is_true();
                    println!("Array item {}: Boolean - `{}`", i, bool_val);
                } else if elem.is_array() {
                    println!("Array item {}: Array", i);
                } else if elem.is_object() {
                    println!("Array item {}: Object - {}", i, v8::json::stringify(scope, elem).unwrap().to_rust_string_lossy(scope));
                } else if elem.is_null_or_undefined() {
                    println!("Array item {}: Null or Undefined", i);
                } else {
                    println!("Array item {}: Unknown type", i);
                }
            }
        }

        // let result = result.to_number(scope).unwrap();
        // println!("Query result length = {}", result.value());
    }

    // Explicit disposal of V8 platform is not necessary.
}
