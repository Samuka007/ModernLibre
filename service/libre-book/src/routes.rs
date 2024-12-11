// use super::casdoor::validator;
use actix_web::web;

use crate::controller::v1;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    v1::book::service_config(cfg);
}
