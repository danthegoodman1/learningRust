use std::cell::RefCell;
use std::{ffi::c_void, rc::Rc};
use v8::{
    self, External, FunctionCallbackArguments, FunctionCallbackInfo, FunctionTemplate, HandleScope,
    Local, Name, ObjectTemplate, PropertyCallbackArguments, PropertyCallbackInfo, ReturnValue,
};

#[derive(Debug)]
struct Query {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
}

impl Query {
    fn new(table: &str) -> Self {
        Query {
            table: table.to_string(),
            columns: vec![],
            conditions: vec![],
        }
    }

    fn select(&mut self, columns: &str) {
        self.columns = columns.split(',').map(|s| s.trim().to_string()).collect();
    }

    fn r#where(&mut self, condition: &str) {
        self.conditions.push(condition.to_string());
    }
}

fn query_constructor(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let table = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let query = Box::new(Query::new(&table));
    println!("Query instance created with table: {}", table);
    let query_ptr = Box::into_raw(query);

    // Create a template for the Query object.
    let query_template = v8::ObjectTemplate::new(scope);
    let set = query_template.set_internal_field_count(1);
    println!("set internal field count: {}", set);

    query_template.set_accessor_with_setter(
        v8::String::new(scope, "table").unwrap().into(),
        get_table,
        set_table,
    );

    query_template.set(
        v8::String::new(scope, "select").unwrap().into(),
        v8::FunctionTemplate::new(scope, query_select).into(),
    );

    query_template.set(
        v8::String::new(scope, "where").unwrap().into(),
        v8::FunctionTemplate::new(scope, query_where).into(),
    );

    let obj = query_template.new_instance(scope).unwrap();
    obj.set_internal_field(0, v8::External::new(scope, query_ptr as *mut c_void).into());

    // Return the newly created object
    rv.set(obj.into());
}

fn get_query<'a>(scope: &mut HandleScope<'a>, obj: Local<v8::Object>) -> Option<&'a mut Query> {
    let internal_field = obj.get_internal_field(scope, 0)?;
    let external = v8::Local::<v8::External>::try_from(internal_field).unwrap();
    let query_ptr = external.value() as *mut Query;
    Some(unsafe { &mut *query_ptr })
}

fn get_table(
    scope: &mut HandleScope,
    name: Local<Name>,
    args: PropertyCallbackArguments,
    rt: ReturnValue,
) {
    println!("get table")
}

fn set_table(
    scope: &mut HandleScope,
    name: Local<Name>,
    value: Local<v8::Value>,
    args: PropertyCallbackArguments,
    rt: ReturnValue,
) {
    println!("set table");
    if let Some(query) = get_query(scope, args.this()) {
        // let columns = args
        //     .get(0)
        //     .to_string(scope)
        //     .unwrap()
        //     .to_rust_string_lossy(scope);
        // query.select(&columns);
        // println!("Query select called with columns: {}", columns);
        // rv.set(v8::undefined(scope).into());
        println!("got it!")
    } else {
        eprintln!("Failed to get Query instance in query_select");
    }
}

fn query_select(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    println!("query seelct");
    if let Some(query) = get_query(scope, args.this()) {
        println!("got it");
        let condition = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        query.r#where(&condition);
        println!("Query where called with condition: {}", condition);
        rv.set(v8::undefined(scope).into());
    } else {
        eprintln!("Failed to get Query instance in query_where");
    }
}

fn query_where(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    if let Some(query) = get_query(scope, args.this()) {
        let condition = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        query.r#where(&condition);
        println!("Query where called with condition: {}", condition);
        rv.set(v8::undefined(scope).into());
    } else {
        eprintln!("Failed to get Query instance in query_where");
    }
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

        // Enter the context for compiling and running the script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Create a Query constructor function.
        let query_constructor_template = v8::FunctionTemplate::new(scope, query_constructor);
        query_constructor_template.set_class_name(v8::String::new(scope, "Query").unwrap());

        // Register the constructor in the global object.
        let global = context.global(scope);
        {
            let key = v8::String::new(scope, "Query").unwrap().into();
            let val = query_constructor_template
                .get_function(scope)
                .unwrap()
                .into();
            global.set(scope, key, val);
        }

        // JavaScript code to create and manipulate a Query object.
        let code = r#"
            let q = new Query("users");
            q.select("name, age");
            // q.where("age > 21");
            q
        "#;
        let source = v8::String::new(scope, code).unwrap();
        let script = v8::Script::compile(scope, source, None).unwrap();

        // Run the script.
        let result = script.run(scope).unwrap();

        // Extract the result (the Query object) from JavaScript.
        let query_obj = result.to_object(scope).unwrap();
        if let Some(query) = get_query(scope, query_obj) {
            // Print the resulting Query object.
            println!("Rust Query object: {:?}", query);
        } else {
            eprintln!("Failed to get Query instance in main");
        }
    }
}
