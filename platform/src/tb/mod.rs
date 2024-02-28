mod request;
pub use self::request::TaobaoRequest;

mod method;
pub use self::method::IRequest;

// 物料精选
pub use self::method::tbk_dg_optimus_material::MaterialSelect;

// 物料搜索
pub use self::method::tbk_dg_material_optional::MaterialSearch;

// 聚划算商品搜索
pub use self::method::ju_items_search::JhsSearch;

// 淘宝客商品详情查询升级版(简版)
pub use self::method::tbk_item_info_upgrade_get::ItemDetail;

// 阿里妈妈推广券详情查询
pub use self::method::tbk_coupon_get::CouponDetail;

// 淘宝客-公用-长链转短链
pub use self::method::tbk_spread_get::SpreadGet;

// 淘宝客-推广者-淘口令回流数据查询
pub use self::method::tbk_dg_tpwd_report_get::GetTPwdReport;

// 创建淘口令
pub use self::method::tbk_tpwd_create::CreateTPwd;

// 淘宝客-推广者-官方活动转链
pub use self::method::tbk_activity_info_get::GetActivityInfo;

// 淘宝客-推广者-物料id列表查询
pub use self::method::tbk_optimus_tou_material_ids_get::MaterialIdGet;
