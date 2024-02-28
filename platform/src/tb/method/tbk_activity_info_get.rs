use super::IRequest;
use std::collections::HashMap;

// taobao.tbk.activity.info.get(淘宝客-推广者-官方活动转链)
// https://open.taobao.com/api.htm?docId=48340&docType=2

pub struct GetActivityInfo {
    pub activity_material_id: String,
    pub adzone_id: i32,
    pub sub_pid: String,
    pub relation_id: i32,
    pub union_id: String,
}
impl GetActivityInfo {
    pub fn new(
        activity_material_id: &str,
        adzone_id: i32,
        sub_pid: &str,
        relation_id: i32,
        union_id: &str,
    ) -> Self {
        GetActivityInfo {
            activity_material_id: activity_material_id.to_string(),
            adzone_id: adzone_id,
            sub_pid: sub_pid.to_string(),
            relation_id: relation_id,
            union_id: union_id.to_string(),
        }
    }
}
impl IRequest for GetActivityInfo {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.activity.info.get".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("activity_material_id", self.activity_material_id.clone());
        map.insert("adzone_id", self.adzone_id.to_string());
        if self.sub_pid.len() > 0 {
            map.insert("sub_pid", self.sub_pid.clone());
        }
        if self.relation_id > 0 {
            map.insert("relation_id", self.relation_id.to_string());
        }
        if self.union_id.len() > 0 {
            map.insert("union_id", self.union_id.clone());
        }
        map
    }
}
