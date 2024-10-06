use std::fs::OpenOptions;
use std::io::{Write, Result};
use sqlparser::ast::Expr;
use sqlparser::ast::SetExpr;
use sqlparser::ast::Statement;

// Helper function to extract data from the SQL AST
pub(crate) fn extract_data_from_query(ast: &[Statement]) -> (String, String) {
    match &ast[0] {
        Statement::Insert(insert) => {
            let table = insert.table_name.to_string();
            let source = insert.source.as_ref().unwrap();
            // For INSERT statements, extract the values
            if let SetExpr::Values(values) = &*source.body {
                if let Some(row) = values.rows.get(0) {
                    return (table, row.iter()
                        .map(|v| match v {
                            Expr::Value(val) => val.to_string(),
                            _ => "NULL".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(","));
                }
            }
        }
        Statement::Update { table, assignments, .. } => {
            // For UPDATE statements, extract the new values
            return (table.to_string(), assignments.iter()
                .map(|a| format!("{}={}", a.target, a.value))
                .collect::<Vec<String>>()
                .join(","));
        }
        // Add more cases for other types of statements as needed
        _ => {}
    }
    ("".to_string(), "".to_string()) // Return empty string if we couldn't extract data
}

pub fn append_to_log(table_name: &str, data: &str) -> Result<()> {
    let input_file = format!("data/{}.txt", table_name);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(input_file)?;
    writeln!(file, "{}", data)
}