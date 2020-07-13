use serde::{Deserialize, Serialize};

/// Helper function used for queries with 'batch insert'
/// gen_sql_query_params_from(2,3,1) will return: "($1,$2,$3),($4,$5,$6)"
/// gen_sql_query_params_from(2,3,2) will return: "($2,$3,$4),($5,$6,$7)"
pub fn gen_sql_query_params_from(n_rows: usize, n_params_per_row: usize, start_from: usize) -> String {
    let mut rows = Vec::with_capacity(n_rows);
    let mut n = start_from;
    for _ in 0..n_rows {
        let mut row = Vec::with_capacity(n_params_per_row);
        for _ in 0..n_params_per_row {
            row.push(format!("${}", n));
            n += 1;
        }
        rows.push(format!("({})", row.join(",")));
    }
    rows.join(",")
}

pub fn gen_sql_query_params(n_rows: usize, n_params_per_row: usize) -> String {
    gen_sql_query_params_from(n_rows, n_params_per_row, 1)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpParams {
    range: String
}

impl HttpParams {
    pub fn range(&self) -> (i64,i64) {
        let pair = self.range.split('-')
            .map(|s| s.parse::<i64>().expect("Bad range"))
            .collect::<Vec<_>>();
        (pair[0], pair[1])
    }
}

