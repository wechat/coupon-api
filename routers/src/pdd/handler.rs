use axum::{extract::Query, response::IntoResponse, Json};
use std::collections::HashMap;
use common::response::ResponseObject;

pub async fn query_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Json(ResponseObject::from_result(&params))
}
