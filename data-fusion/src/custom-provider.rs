use std::{any::Any, collections::HashMap, sync::Arc};

use async_trait::async_trait;
use dashmap::DashMap;
use datafusion::{
    arrow::array::RecordBatch,
    catalog::{schema::SchemaProvider, CatalogProvider},
    datasource::TableProvider,
    error::DataFusionError,
    prelude::*,
};

struct MemorySchemaProvider {
    tables: DashMap<String, Arc<dyn TableProvider>>,
}

impl MemorySchemaProvider {
    fn new() -> Self {
        MemorySchemaProvider {
            tables: DashMap::new(),
        }
    }
}

#[async_trait]
impl SchemaProvider for MemorySchemaProvider {
    #[doc = " Returns this `SchemaProvider` as [`Any`] so that it can be downcast to a"]
    #[doc = " specific implementation."]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[doc = " Retrieves the list of available table names in this schema."]
    fn table_names(&self) -> Vec<String> {
        println!("getting table names");
        self.tables.iter().map(|f| f.key().clone()).collect()
    }

    async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError> {
        println!("getting table: {}", name);
        let table = self.tables.get(name);
        if let Some(table) = table {
            return Ok(Some(table.value().clone()));
        } else {
            return Ok(None);
        }
    }

    // This is not required
    fn register_table(
        &self,
        name: String,
        table: Arc<dyn TableProvider>,
    ) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError> {
        println!("setting table {}", name);
        self.tables.insert(name, table.clone());
        Ok(Some(table))
    }

    #[doc = " Returns true if table exist in the schema provider, false otherwise."]
    fn table_exist(&self, name: &str) -> bool {
        println!("table exists: {}", name);
        self.tables.contains_key(name)
    }
}

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    let schema_provider = Arc::new(MemorySchemaProvider::new());

    let catalog_provider = datafusion::catalog::MemoryCatalogProvider::new();
    catalog_provider
        .register_schema("public", schema_provider)
        .unwrap();

    // Putting the csv table in our custom schema!
    ctx.register_catalog("datafusion", Arc::new(catalog_provider));
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

    // catalog.schema.table
    let df = ctx.sql("select * from blah").await.unwrap_err();
    match df {
        DataFusionError::Plan(plan) => {
            println!("Got plan err: {}", plan)
        },
        _ => panic!("{df}"),
    }

    Ok(())
}
