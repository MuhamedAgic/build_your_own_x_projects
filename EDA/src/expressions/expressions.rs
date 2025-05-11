use std::net::Shutdown;
use polars::prelude::*;


pub fn select_all_except(df: &DataFrame, exclude_columns: &[&str]) -> PolarsResult<DataFrame> {
    let output = df.clone()
        .lazy()
        .select([col("*").exclude(exclude_columns)])
        .collect()?;
    println!("{:?}", output);
    Ok(output)
}

pub fn count_unique(df: &DataFrame, column: &str) -> PolarsResult<usize> {
    Ok(df.column(column)?.n_unique()?) // counts amount of unique values within a column
}

// from assignment
pub fn downcast_floats(df: &mut DataFrame) -> PolarsResult<DataFrame> {
    // floats 64 naar floats 32 casten
    let downcasted_float_cols = df
        .get_columns()
        .iter()
        .filter(|series| series.dtype() == &DataType::Float64)
        .map(|series| col(series.name()).cast(DataType::Float32))
        .collect::<Vec<_>>();

    df.clone()
        .lazy()
        .with_columns(downcasted_float_cols)
        .collect()
}

// my version
pub fn cast_columns(df: &mut DataFrame,
                    columns: &[&str],
                    from_dtype: DataType,
                    to_dtype: DataType) -> PolarsResult<DataFrame> {
    let casted_cols: Vec<Expr> = columns
        .iter()
        .filter_map(|&col_name| {
            match df.column(col_name) {
                Ok(series) if series.dtype() == &from_dtype => {
                    Some(col(series.name()).cast(to_dtype.clone()))
                }
                _ => None
            }
        })
        .collect();

    df.clone()
        .lazy()
        .with_columns(casted_cols)
        .collect()
}

pub fn filter(df: &DataFrame, column: &str, value: &str) -> PolarsResult<DataFrame> {
    let series = df.column(column)?;
    let filtered_rows = series.utf8()?.equal(value);
    df.filter(&filtered_rows)
}

pub fn explore(df: &DataFrame, column: &str) -> PolarsResult<DataFrame> {
    let explore_df = df
        .clone()
        .lazy()
        .select([col(column)])
        .collect()?;
    println!("{:?}", explore_df);
    Ok(explore_df)
}


pub fn aggregate(df: &mut DataFrame, column: &str) -> PolarsResult<DataFrame> {
    // implement functionality to count the total records from a certain city or country

}