extern crate env_logger;
mod adaptor;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use log::{info, error};
use adaptor::read::{read_record, extract_table_name, is_read_operation};
use adaptor::write::{append_to_log, extract_data_from_query};

#[derive(Deserialize)]
struct SqlQuery {
    query: String,
}

#[derive(Serialize)]
struct QueryResult {
    result: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Server starting on http://127.0.0.1:8080");
    let result = HttpServer::new(|| {
        App::new()
            .service(web::resource("/query").route(web::post().to(execute_query)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    match result {
        Ok(_) => println!("Server stopped."),
        Err(e) => error!("Server error: {}", e),
    }

    Ok(())
}

async fn execute_query(query: web::Json<SqlQuery>) -> impl Responder {
    let dialect = GenericDialect {};
    let ast = match Parser::parse_sql(&dialect, &query.query) {
        Ok(ast) => ast,
        Err(e) => return HttpResponse::BadRequest().body(format!("SQL parsing error: {}", e)),
    };
    
    let is_read_query = is_read_operation(&ast);

    match is_read_query {
        true => {
            // Read lines from input file and write to http response
            let table = extract_table_name(&ast);
            println!("table: {}", table);   

            let mut response = HttpResponse::Ok();
            let body = read_record(&table);
            response.body(body)
        }
        false => {
            // Extract data from the SQL query
            let (table, data) = extract_data_from_query(&ast);
            println!("table: {}, data: {}", table, data);

            // Append the extracted data to the log file
            match append_to_log(&table, &data) {
                Ok(_) => HttpResponse::Ok().json(QueryResult { result: "Query executed successfully".to_string() }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to append to log: {}", e)),
            }
        }
    }
}
