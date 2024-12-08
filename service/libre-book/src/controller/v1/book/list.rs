use crate::schema;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use libre_core::database::postgres::PostgresPool;

#[derive(Deserialize)]
pub struct BooksListQuery {
    pub limit: Option<u32>,
    pub by: Option<String>,
}

pub async fn list(
    db_pool: web::Data<PostgresPool>,
    query: web::Query<BooksListQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().await?;

    let books = match query.into_inner() {
        BooksListQuery { limit, by } => {
            let limit = limit.unwrap_or(10) as i64;
            let by = by.unwrap_or("id".to_string());
            match by.as_str() {
                "id" => {
                    schema::book::dsl::book
                        .select(schema::Book::as_select())
                        .order(schema::book::id.desc())
                        .limit(limit)
                        .load::<schema::Book>(&mut conn)
                        .await
                }
                "recent" => {
                    schema::book::dsl::book
                        .select(schema::Book::as_select())
                        .order(schema::book::added_date.desc())
                        .limit(limit)
                        .load::<schema::Book>(&mut conn)
                        .await
                }
                "top-rated" => {
                    schema::book::dsl::book
                        .select(schema::Book::as_select())
                        .order(schema::book::rating.desc())
                        .limit(limit)
                        .load::<schema::Book>(&mut conn)
                        .await
                }
                _ => return Err(actix_web::error::ErrorBadRequest("Invalid query parameter")),
            }
        }
    }.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(books))
}
