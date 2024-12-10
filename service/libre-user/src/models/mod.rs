use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::users::dsl::*;

#[derive(Default, Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub uid: uuid::Uuid,
    pub login: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub admin: bool,
    pub github_id: Option<i64>,
    pub casdoor_id: Option<String>,
}

impl User {
    pub async fn create(
        &self,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        // TODO: should use `let now = select(diesel::dsl::now).get_result::<SystemTime>(conn)?;`
        diesel::insert_into(users)
            .values(self)
            .get_result(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn find_by_login(
        input_login: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        users.filter(login.eq(input_login))
            .select(User::as_select())
            .first(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn find_by_github_id(
        input_github_id: u64,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        users.filter(github_id.eq(input_github_id as i64))
            .select(User::as_select())
            .first(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn find_by_casdoor_id(
        input_casdoor_id: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        users.filter(casdoor_id.eq(input_casdoor_id))
            .select(User::as_select())
            .first(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_login(
        input_login: &str,
        new_login: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        let existing_user: Result<User, diesel::result::Error> = users
            .filter(login.eq(input_login))
            .select(User::as_select())
            .first(conn)
            .await;

        if existing_user.is_ok() {
            return Err(Error::Conflict);
        }

        diesel::update(users.filter(login.eq(input_login)))
            .set(login.eq(new_login))
            .execute(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_username(
        input_login: &str,
        new_username: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        diesel::update(users.filter(login.eq(input_login)))
            .set(name.eq(new_username))
            .execute(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_email(
        input_login: &str,
        new_email: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        diesel::update(users.filter(login.eq(input_login)))
            .set(email.eq(new_email))
            .execute(conn)
            .await
            .map_err(Error::from)
    }
}

impl From<&User> for libre_core::jsonwebtoken::Claims {
    fn from(input_user: &User) -> Self {
        let iat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self {
            iss: "libre".to_string(),
            exp: iat + 3600,
            sub: input_user.uid.to_string(),
            aud: "libre".to_string(),
            iat,
            jti: input_user.uid.to_string(),
            login: input_user.login.clone(),
            name: input_user.name.clone().unwrap_or_default(),
            admin: input_user.admin,
        }
    }
}
pub enum Error {
    NotFound,
    InternalServerError,
    Conflict,
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => {
                log::warn!("Database error: {:?}", err);
                Error::InternalServerError
            },
        }
    }
}

impl From<Error> for actix_web::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
            Error::InternalServerError => {
                actix_web::error::ErrorInternalServerError("Internal server error")
            }
            Error::Conflict => actix_web::error::ErrorConflict("Conflict"),
        }
    }
}
