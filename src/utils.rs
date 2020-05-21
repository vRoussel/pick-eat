/// Helper function used for queries with 'batch insert'
/// gen_query_param(2,3) will return: "($1,$2,$3),($4,$5,$6)"
pub fn gen_query_params(n_rows: usize, n_params_per_row: usize) -> String {
    let mut rows = Vec::with_capacity(n_rows);
    let mut n = 1;
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

