use std::env;
use std::fs::*;
use polars::prelude::{self, DataType, PolarsResult};
use crate::expressions::explore;

mod io;
mod concurrency;
mod transformations;
mod expressions;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut df = io::read_csv("datasets/GlobalTemperatures.csv")?;
    io::write_parquet(&mut df, "datasets/my.parquet")?;

    let selected_columns = expressions::select_all_except(&df, &["LandAverageTemperature"]);
    println!("selected columns: {:?}", selected_columns);

    // let downcasted_df = expressions::downcast_floats(&mut df);
    // println!("downcasted df: {:?}", downcasted_df);

    let downcasted_floats_df = expressions::cast_columns(&mut df,
                                                         &["LandAverageTemperature"],
                                                         DataType::Float64,
                                                         DataType::Float32);

    println!("downcasted_floats_df: {:?}", downcasted_floats_df);

    let _ = expressions::explore(&df, "LandAverageTemperature");





    return Ok(());
}



