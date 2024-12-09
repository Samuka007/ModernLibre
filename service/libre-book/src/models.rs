use chrono::NaiveDate;
use derive_builder::Builder;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub cover_url: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub rating: Option<f64>,
    pub description: Option<String>,
    pub added_date: NaiveDate,
}

#[derive(Insertable, Builder)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub added_date: NaiveDate,
}

impl NewBookBuilder {
    pub fn with_defaults() -> Self {
        let mut new_book = NewBookBuilder::default();
        new_book.added_date(chrono::Local::now().naive_local().date());
        new_book
    }
}
