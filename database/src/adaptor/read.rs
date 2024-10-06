use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use sqlparser::ast::{Query, TableFactor};
use sqlparser::ast::SetExpr;
use sqlparser::ast::Statement;

pub fn extract_table_name(ast: &[Statement]) -> String {
    match &ast[0] {
        Statement::Query(query) => {
            // For SELECT statements, extract the table name
            let table = match &*query.body {
                SetExpr::Select(select) => select.from.first()
                    .and_then(|table_with_join| match &table_with_join.relation {
                        TableFactor::Table { name, .. } => Some(name.to_string()),
                        _ => None,
                    })
                    .unwrap_or_default(),
                _ => String::new(),
            };
        
            table
        }
        _ => String::new(),
    }
}

pub fn read_record(table_name: &str) -> String {
    let input_file = format!("data/{}.txt", table_name);
    println!("Reading from {}", input_file);
    let mut body = String::new();
            
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                body.push_str(&ip);
                body.push('\n');
            }
        }
    }

    body
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn is_read_operation(ast: &[Statement]) -> bool {
    if ast.len() != 1 {
        return false; // For simplicity, we'll assume multiple statements are not read-only
    }

    match &ast[0] {
        Statement::Query(query) => is_read_query(query),
        Statement::Explain { .. } => true, // EXPLAIN is considered read-only
        Statement::ShowVariable { .. } => true, // SHOW statements are read-only
        Statement::ShowColumns { .. } => true,
        Statement::ShowTables { .. } => true,
        _ => false, // All other statement types are considered write operations
    }
}

fn is_read_query(query: &Query) -> bool {
    // Check if the query is a SELECT statement without any write operations
    match &*query.body {
        sqlparser::ast::SetExpr::Select(_) => true,
        _ => false,
    }
}