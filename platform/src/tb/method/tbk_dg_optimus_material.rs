use super::IRequest;
use std::collections::HashMap;

// taobao.tbk.dg.optimus.material(淘宝客-推广者-物料精选)
// https://open.taobao.com/api.htm?docId=33947&docType=2&scopeId=16518
// taobao.tbk.dg.material.recommend(淘宝客-推广者-物料精选升级版)
// https://open.taobao.com/api.htm?docId=62201&docType=2&scopeId=27939

pub struct MaterialSelect {
    pub adzone_id: String,    // 推广位id
    pub material_id: i32,     // 物料id
    pub page_size: i32,       // 每页记录数，默认20，最大100
    pub page_no: i32,         // 页码，默认1
    pub favorites_id: String, // 收藏夹ID
    pub item_id: String,      // 商品ID，用于相似商品推荐
}
impl MaterialSelect {
    pub fn new(
        adzone_id: String,
        material_id: i32,
        page_no: i32,
        page_size: i32,
        item_id: String,
        favorites_id: String,
    ) -> Self {
        MaterialSelect {
            adzone_id,
            material_id,
            page_no,
            page_size,
            item_id,
            favorites_id,
        }
    }
}
impl IRequest for MaterialSelect {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.dg.material.recommend".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("adzone_id", self.adzone_id.to_string());
        map.insert("material_id", self.material_id.to_string());

        if self.page_size > 0 {
            map.insert("page_size", self.page_size.to_string());
        }
        if self.page_no > 0 {
            map.insert("page_no", self.page_no.to_string());
        }
        if self.favorites_id.len() > 0 {
            map.insert("favorites_id", self.favorites_id.clone());
        }
        if self.item_id.len() > 0 {
            map.insert("item_id", self.item_id.clone());
        }
        map
    }
}
