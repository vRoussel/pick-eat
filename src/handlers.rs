pub mod recipes;
pub mod ingredients;
pub mod tags;
pub mod categories;
pub mod units;

pub enum RangeError {
    OutOfBounds,
    TooWide,
    Invalid
}

pub fn check_range(range: (i64,i64), max_range_size: i64, total_count: i64) -> Result<(),RangeError> {
    let (range_first, range_last) = range;

    //TODO < 0 not working
    if range_first < 0 || range_last < 0
       || range_last < range_first {
        return Err(RangeError::Invalid);
    }

    let range_size = range_last - range_first + 1;
    if range_size > max_range_size {
        return Err(RangeError::TooWide);
    }

    if range_first > total_count {
        return Err(RangeError::OutOfBounds);
    }

    Ok(())
}

pub async fn get_total_count(db_conn: &tokio_postgres::Client, table_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let query = format!("SELECT count(*) FROM {}", table_name);
    db_conn.query(query.as_str(), &[])
        .await
        .map(|rows| rows[0].get(0))
        .map_err(|e|e.into())
}
