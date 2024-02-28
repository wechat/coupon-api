use super::handler::query_list;
use crate::init::handle_router;
use axum::{routing::get, Router};

const ROUTER_SEARCH: &str = "/pdd";

// TODO
pub fn list() -> Router {
    let path = format!("{}/list", ROUTER_SEARCH);
    handle_router(&path, get(query_list))
}
