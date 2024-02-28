use super::IRequest;
use std::collections::HashMap;

// taobao.tbk.dg.optimus.material(淘宝客-公用-淘宝客商品详情查询升级版（简易版）)
// https://open.taobao.com/api.htm?docId=64763&docType=2&scopeId=16189

pub struct ItemDetail {
    pub item_id: String, // 商品ID。多个用","分割，一次最多查询20个
}
impl ItemDetail {
    pub fn new(ids: String) -> Self {
        ItemDetail { item_id: ids }
    }
}
impl IRequest for ItemDetail {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.item.info.upgrade.get".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("item_id", self.item_id.to_string());
        map
    }
}
