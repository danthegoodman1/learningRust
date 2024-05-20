use std::convert::TryFrom;
use v8::ContextScope;
use v8::FunctionCallbackArguments;
use v8::FunctionTemplate;
use v8::HandleScope;
use v8::PromiseResolver;
use v8::ReturnValue;

fn rust_do_a_thing(value: i32) -> i32 {
    value * 2
}

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

        // Enter the context for compiling and running JavaScript.
        let scope = &mut ContextScope::new(handle_scope, context);

        // Create and compile the JavaScript source.
        let c_source = r#"
            async function do_a_thing(value) {
                return await rust_do_a_thing(value);
            }
        "#;

        let source = v8::String::new(scope, c_source).unwrap();
        let script = v8::Script::compile(scope, source, None).unwrap();
        script.run(scope).unwrap();

        // Get the global object.
        let global = context.global(scope);

        // Function binding for `rust_do_a_thing`.
        let rust_do_a_thing_fn = {
            let rust_do_a_thing =
                |scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue| {
                    let value = args.get(0).int32_value(scope).unwrap();
                    let result = rust_do_a_thing(value);
                    let promise = {
                        let resolver = PromiseResolver::new(scope).unwrap();
                        let val = v8::Integer::new(scope, result).into();
                        resolver.resolve(scope, val).unwrap();
                        resolver.get_promise(scope)
                    };
                    rv.set(promise.into());
                };
            FunctionTemplate::new(scope, rust_do_a_thing)
                .get_function(scope)
                .unwrap()
        };

        let rust_do_a_thing_key = v8::String::new(scope, "rust_do_a_thing").unwrap();
        global.set(scope, rust_do_a_thing_key.into(), rust_do_a_thing_fn.into());

        // Now, you can call `do_a_thing` from Rust.
        let key = v8::String::new(scope, "do_a_thing").unwrap();
        let do_a_thing_value = global.get(scope, key.into()).unwrap();

        // Ensure `do_a_thing` is a function.
        if !do_a_thing_value.is_function() {
            panic!("do_a_thing is not a function");
        }

        let do_a_thing_fn = v8::Local::<v8::Function>::try_from(do_a_thing_value).unwrap();

        // Call `do_a_thing` function from Rust with an example value.
        let arg = v8::Integer::new(scope, 5);
        let args = &[arg.into()];
        let result = do_a_thing_fn.call(scope, global.into(), args).unwrap();

        if result.is_promise() {
            let promise = v8::Local::<v8::Promise>::try_from(result).unwrap();

            let callback = |_scope: &mut v8::HandleScope,
                            args: v8::FunctionCallbackArguments,
                            mut rv: v8::ReturnValue| {
                rv.set(args.get(0));
            };

            let resolved_fn = v8::Function::new(scope, callback).unwrap();

            promise.then(scope, resolved_fn).unwrap();

            scope.perform_microtask_checkpoint();

            let resolved_value = promise.result(scope);
            let resolved_number = resolved_value.to_number(scope).unwrap();
            println!("do_a_thing result: {}", resolved_number.value());
        } else {
            println!("do_a_thing did not return a promise");
        }
    }
}
