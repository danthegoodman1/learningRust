use datafusion::{
    arrow::{array::RecordBatch, ipc::Bool},
    logical_expr::LogicalPlan,
    prelude::*,
};

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    ctx.register_csv("example", "example.csv", CsvReadOptions::new())
        .await?;

    let df = ctx
        .sql("with z as (select * from example) SELECT a, MIN(b) FROM z WHERE a <= b GROUP BY a LIMIT 100")
        .await?;
    let plan = df.logical_plan();
    println!("Plan: {:?}", plan);
    let has_modify = check_for_write(plan);
    println!("has modify: {:?}", has_modify);

    let df = ctx.sql("COPY example TO 'file_name.json'").await?;
    let plan = df.logical_plan();
    let has_modify = check_for_write(plan);
    println!("has modify: {:?}", has_modify);

    let df = ctx.sql("CREATE EXTERNAL TABLE test STORED AS CSV LOCATION 'example.csv'").await?;
    let plan = df.logical_plan();
    let has_modify = check_for_write(plan);
    println!("has modify: {:?}", has_modify);

    Ok(())
}

fn check_for_write(plan: &LogicalPlan) -> bool {
    match plan {
        LogicalPlan::Copy(_) | LogicalPlan::Ddl(_) | LogicalPlan::Dml(_) | LogicalPlan::EmptyRelation(_) => true,
        p => {
            let inputs = p.inputs().len();
            if inputs > 0 {
                return check_for_write(plan.inputs()[0]);
            }
            false
        }
    }
}
