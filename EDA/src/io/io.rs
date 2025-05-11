use polars::prelude::*;
use std::path::Path;
use std::fs;


pub fn read_csv(path: &str) -> PolarsResult<DataFrame> {
    let df = CsvReader::from_path(path)?.finish()?;
    println!("{:?}", df);
    return Ok(df);
}

pub fn write_parquet(df: &mut DataFrame, path: &str) -> PolarsResult<()> {
    if Path::new(path).exists() {
        println!("parquet file {} already exists!", path);
        return Ok(());
    }
    let file = std::fs::File::create(path).map_err(PolarsError::Io)?;
    ParquetWriter::new(file).finish(df)?;
    println!("DataFrame written to '{}'.", path);
    Ok(())
}


