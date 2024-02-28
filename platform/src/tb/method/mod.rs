use std::collections::HashMap;

pub trait IRequest {
    fn get_method_name(&self) -> String;
    fn get_map_param(&self) -> HashMap<&str, String>;
}

pub mod ju_items_search;
pub mod tbk_activity_info_get;
pub mod tbk_coupon_get;
pub mod tbk_dg_material_optional;
pub mod tbk_dg_optimus_material;
pub mod tbk_dg_tpwd_report_get;
pub mod tbk_item_info_upgrade_get;
pub mod tbk_spread_get;
pub mod tbk_tpwd_create;
pub mod tbk_optimus_tou_material_ids_get;
