use std::collections::HashMap;

use datafusion::{arrow::array::RecordBatch, prelude::*};

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    ctx.register_csv("example", "example.csv", CsvReadOptions::new())
        .await?;

    // create a plan to run a SQL query
    let df = ctx
        .sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a order by a LIMIT 100")
        .await?;

    // execute and print results
    let rbs = df.collect().await.unwrap();

    println!("Got {} record batches", rbs.len());

    let schema = rbs[0].schema();
    let fields = schema.fields();
    let column_names: Vec<String> = fields
        .iter()
        .map(|field| field.name().to_string())
        .collect();
    println!("got column names {:?}", column_names);

    // Just print as strings
    for batch in &rbs {
        for row_index in 0..batch.num_rows() {
            let row: Vec<String> = (0..batch.num_columns())
                .map(|col_index| {
                    let column = batch.column(col_index);
                    datafusion::arrow::util::display::array_value_to_string(column, row_index)
                        .unwrap()
                })
                .collect();
            println!("{:?}", row);
        }
    }

    // Build a map
    let mut column_info: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for field in fields.iter() {
        let column_name = field.name().to_string();
        let column_type = field.data_type().to_string();

        let mut values: Vec<String> = Vec::new();

        for batch in &rbs {
            let column_index = schema.index_of(&column_name).unwrap();
            let column = batch.column(column_index);

            for row_index in 0..batch.num_rows() {
                let value = datafusion::arrow::util::display::array_value_to_string(column, row_index).unwrap();
                values.push(value);
            }
        }

        column_info.insert(column_name.clone(),
            HashMap::from([("type".to_string(), vec![column_type]), ("values".to_string(), values)])
        );
    }

    println!("Final map: {:?}", column_info);

    // Handle the values by type
    for (column_index, field) in fields.iter().enumerate() {
        let column_name = field.name().to_string();
        let column_type = field.data_type(); // could match on this type
        println!("Handling column {} ({})", column_name, column_type.to_string());

        for batch in &rbs {
            let column = batch.column(column_index);

            for row_index in 0..batch.num_rows() {
                let value = match column_type {
                    datafusion::arrow::datatypes::DataType::Int32 => {
                        let array = column.as_any().downcast_ref::<datafusion::arrow::array::Int32Array>().unwrap();
                        array.value(row_index).to_string()
                    },
                    datafusion::arrow::datatypes::DataType::Float64 => {
                        let array = column.as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap();
                        array.value(row_index).to_string()
                    },
                    datafusion::arrow::datatypes::DataType::Utf8 => {
                        let array = column.as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
                        array.value(row_index).to_string()
                    },
                    _ => {
                        datafusion::arrow::util::display::array_value_to_string(column, row_index).unwrap()
                    }
                };
                println!("Value: {}", value);
            }
        }
    }

    Ok(())
}
