use axum::{extract::Query, response::IntoResponse, Json};
use common::response::ResponseObject;
use platform::tb::{
    CreateTPwd, ItemDetail, MaterialIdGet, MaterialSearch, MaterialSelect, TaobaoRequest,
};
use std::collections::HashMap;

/**
 * 物料精选
 */
pub async fn query_material_select(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let material_id = params
        .get("material_id")
        .expect("material_id not found")
        .parse::<i32>();

    match material_id {
        Ok(material_id) => {
            if material_id > 0 {
                let page_no = utils::option_to_number(params.get("page_no"), 1);
                let page_size = utils::option_to_number(params.get("page_size"), 20);
                let favorites_id = utils::option_to_string(params.get("favorites_id"));
                let item_id = utils::option_to_string(params.get("item_id"));
                let app_key = dotenv::var("APP_KEY").unwrap();
                let app_secret = dotenv::var("APP_SECRET").unwrap();
                let adzone_id = dotenv::var("ADZONE_ID").unwrap();
                let arg = MaterialSelect::new(
                    adzone_id,
                    material_id,
                    page_no,
                    page_size,
                    item_id,
                    favorites_id,
                );
                let response = TaobaoRequest::new(app_key.as_str(), app_secret.as_str(), "")
                    .send_request(arg)
                    .await
                    .unwrap();
                Json(ResponseObject::from_result(&response))
            } else {
                Json(ResponseObject::from_error("material_id is null"))
            }
        }
        Err(err) => {
            println!("error: {}", err);
            Json(ResponseObject::from_error("material_id is null"))
        }
    }
}

/**
 * 物料搜索
 */
pub async fn query_material_search(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let key = params.get("key").expect("key not found").parse::<String>();
    match key {
        Ok(key) => {
            if key.len() > 0 {
                let page_no = utils::option_to_number(params.get("page_no"), 1);
                let page_size = utils::option_to_number(params.get("page_size"), 20);
                let material_id = utils::option_to_number(params.get("material_id"), 17004);
                let app_key = dotenv::var("APP_KEY").unwrap();
                let app_secret = dotenv::var("APP_SECRET").unwrap();
                let adzone_id = dotenv::var("ADZONE_ID").unwrap();
                let arg = MaterialSearch::new(adzone_id, key, page_no, page_size, material_id);
                let response = TaobaoRequest::new(app_key.as_str(), app_secret.as_str(), "")
                    .send_request(arg)
                    .await
                    .unwrap();
                Json(ResponseObject::from_result(&response))
            } else {
                Json(ResponseObject::from_error("key is null"))
            }
        }
        Err(err) => {
            println!("error: {}", err);
            Json(ResponseObject::from_error("key is null"))
        }
    }
}

/**
 * 物料id列表查询
 */
pub async fn query_material_id_get(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 物料主题类型, 1促销活动;2热门主题;3精选榜单;4行业频道等;5其他
    let subject = params
        .get("subject")
        .expect("subject not found")
        .parse::<i32>()
        .unwrap_or(0);
    // 物料类型，1: 商品；2:权益
    let material_type = params
        .get("material_type")
        .expect("material_type not found")
        .parse::<i32>()
        .unwrap_or(0);
    // 页码，默认1，取值范围1~100
    let page_no = utils::option_to_number(params.get("page_no"), 1);
    // 每页物料id数量，默认20，取值范围1~100
    let page_size = utils::option_to_number(params.get("page_size"), 20);
    if subject > 0 && material_type > 0 {
        let app_key = dotenv::var("APP_KEY").unwrap();
        let app_secret: String = dotenv::var("APP_SECRET").unwrap();
        let arg = MaterialIdGet::new(subject, material_type, page_no, page_size);
        let response = TaobaoRequest::new(app_key.as_str(), app_secret.as_str(), "")
            .send_request(arg)
            .await
            .unwrap();
        Json(ResponseObject::from_result(&response))
    } else {
        Json(ResponseObject::from_error(
            "subject or material_type is null",
        ))
    }
}

/**
 * 商品详情
 */
pub async fn query_item_detail(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let ids = params
        .get("ids")
        .expect("ids not found")
        .parse::<String>()
        .unwrap();
    if ids.len() > 0 {
        let app_key = dotenv::var("APP_KEY").unwrap();
        let app_secret = dotenv::var("APP_SECRET").unwrap();
        let arg = ItemDetail::new(ids);
        let response = TaobaoRequest::new(app_key.as_str(), app_secret.as_str(), "")
            .send_request(arg)
            .await
            .unwrap();
        Json(ResponseObject::from_result(&response))
    } else {
        Json(ResponseObject::from_error("ids is null"))
    }
}

/**
 * 创建淘口令
 */
pub async fn query_create_tpwd(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let url = params
        .get("url")
        .expect("url not found")
        .parse::<String>()
        .unwrap();
    if url.len() > 0 {
        let app_key = dotenv::var("APP_KEY").unwrap();
        let app_secret = dotenv::var("APP_SECRET").unwrap();
        let arg = CreateTPwd::new(&url);
        let response = TaobaoRequest::new(app_key.as_str(), app_secret.as_str(), "")
            .send_request(arg)
            .await
            .unwrap();
        Json(ResponseObject::from_result(&response))
    } else {
        Json(ResponseObject::from_error("url is null"))
    }
}
