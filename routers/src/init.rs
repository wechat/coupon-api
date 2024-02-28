use super::{jd, pdd, tb};
use axum::{routing::MethodRouter, Router};
use dotenv::var;
use http::HeaderValue;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

// 构建路由公共方法
pub fn handle_router(path: &str, method_router: MethodRouter) -> Router {
    // 设置跨域
    let domain = var("DOMAIN").expect("DOMAIN must be set");
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([domain.parse::<HeaderValue>().unwrap()]))
        .allow_methods(Any)
        .allow_headers(Any);
    let _path = format!("/api{}", path); // 统一api 路径
    Router::new().route(&_path, method_router).layer(cors)
}

// api 路由入口
pub fn router() -> Router {
    auth_init_router().merge(init_router())
}

// 需要权限认证的路由
fn auth_init_router() -> Router {
    let app = Router::new();
    return app;
}

// 不需要权限认证的路由
fn init_router() -> Router {
    let app = Router::new()
        .merge(tb::material_select()) // 物料精选
        .merge(tb::material_search()) // 物料搜索
        .merge(tb::material_id_get()) // 物料搜索
        .merge(tb::item_detail()) // 商品详情
        .merge(tb::create_tpwd()) // 创建淘口令
        .merge(jd::list()) // 商品列表
        .merge(pdd::list()); // 商品列表
    return app;
}
