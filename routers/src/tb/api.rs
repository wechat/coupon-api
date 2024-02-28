use super::handler::{
    query_create_tpwd, query_item_detail, query_material_id_get, query_material_search,
    query_material_select,
};
use crate::init::handle_router;
use axum::{routing::get, Router};

const ROUTER_SEARCH: &str = "/tb";

/**
 * 物料精选
 */
pub fn material_select() -> Router {
    let path = format!("{}/materialselect", ROUTER_SEARCH);
    handle_router(&path, get(query_material_select))
}

/**
 * 物料搜索
 */
pub fn material_search() -> Router {
    let path = format!("{}/materialsearch", ROUTER_SEARCH);
    handle_router(&path, get(query_material_search))
}

/**
 * 物料id列表查询
 */
pub fn material_id_get() -> Router {
    let path = format!("{}/materialid", ROUTER_SEARCH);
    handle_router(&path, get(query_material_id_get))
}

/**
 * 创建淘口令
 */
pub fn create_tpwd() -> Router {
    let path = format!("{}/createtpwd", ROUTER_SEARCH);
    handle_router(&path, get(query_create_tpwd))
}

/**
 * 商品详情
 */
pub fn item_detail() -> Router {
    let path = format!("{}/item", ROUTER_SEARCH);
    handle_router(&path, get(query_item_detail))
}
