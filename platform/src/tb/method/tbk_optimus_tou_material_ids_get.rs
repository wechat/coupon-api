use super::IRequest;
use serde_json::json;
use std::collections::HashMap;

// taobao.tbk.optimus.tou.material.ids.get(淘宝客-推广者-物料id列表查询)
// https://open.taobao.com/api.htm?docId=64333&docType=2&scopeId=27939

fn hashmap_to_string(map: HashMap<&str, String>) -> String {
    let json = json!(map);
    serde_json::to_string(&json).unwrap()
}

pub struct MaterialIdGet {
    pub subject: i32, // 物料主题类型, 1促销活动;2热门主题;3精选榜单;4行业频道等;5其他
    pub material_type: i32, // 物料类型，1: 商品；2:权益
    pub page_no: i32, // 页码，默认1
    pub page_size: i32, // 每页记录数，默认20，最大100
}
impl MaterialIdGet {
    pub fn new(subject: i32, material_type: i32, page_no: i32, page_size: i32) -> Self {
        MaterialIdGet {
            subject,
            material_type,
            page_no,
            page_size,
        }
    }
}
impl IRequest for MaterialIdGet {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.optimus.tou.material.ids.get".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        let mut query: HashMap<&str, String> = HashMap::new();
        query.insert("subject", self.subject.to_string());
        query.insert("material_type", self.material_type.to_string());

        if self.page_size > 0 {
            query.insert("page_size", self.page_size.to_string());
        }
        if self.page_no > 0 {
            query.insert("page_no", self.page_no.to_string());
        }
        map.insert("material_query", hashmap_to_string(query));
        map
    }
}
