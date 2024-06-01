use datafusion::{arrow::array::RecordBatch, prelude::*};

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    ctx.register_csv("example", "example.csv", CsvReadOptions::new())
        .await?;

    // create a plan to run a SQL query
    let df = ctx
        .sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100")
        .await?;

    // execute and print results
    let rbs = df.collect().await.unwrap();
    // println!("{:?}", a);
    let mut buf = Vec::new();
    datafusion::arrow::json::writer::LineDelimitedWriter::new(&mut buf)
        .write_batches(&rbs.iter().collect::<Vec<&RecordBatch>>())
        .unwrap();
    println!("JSON: {:?}", String::from_utf8(buf).unwrap());
    // df.show().await?;
    Ok(())
}
