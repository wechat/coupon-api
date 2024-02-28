use super::IRequest;
use std::collections::HashMap;

// taobao.tbk.dg.optimus.material(淘宝客-推广者-物料搜索)
// https://open.taobao.com/api.htm?docId=35896&docType=2&scopeId=16516
// taobao.tbk.dg.material.optional.upgrade(淘宝客-推广者-物料搜索升级版)
// https://open.taobao.com/api.htm?docId=64759&docType=2&scopeId=27939

pub struct MaterialSearch {
    pub adzone_id: String, // 推广位id
    pub material_id: i32, // 不传时默认物料id=2836；如果直接对消费者投放，可使用官方个性化算法优化的搜索物料id=17004
    pub page_size: i32,   // 每页记录数，默认20，最大100
    pub page_no: i32,     // 页码，默认1
    pub start_dsr: i32,   // 商品筛选(特定媒体支持)-店铺dsr评分
    pub q: String,        // 查询词
    // pub is_tmall: bool, // 是否是天猫商品，默认false
    pub sort: String, // 排序_des（降序），排序_asc（升序），销量（total_sales），淘客佣金比率（tk_rate），累计推广量（tk_total_sales），总支出佣金（tk_total_commi），价格（price），匹配分（match）
}
impl MaterialSearch {
    pub fn new(
        adzone_id: String,
        q: String,
        page_no: i32,
        page_size: i32,
        material_id: i32,
    ) -> Self {
        MaterialSearch {
            adzone_id,
            q,
            page_no,
            page_size,
            material_id,
            start_dsr: 45000,
            sort: "total_sales_des".to_string(),
        }
    }
}
impl IRequest for MaterialSearch {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.dg.material.optional.upgrade".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("adzone_id", self.adzone_id.to_string());
        map.insert("material_id", self.material_id.to_string());
        map.insert("q", self.q.to_string());
        map.insert("sort", self.sort.to_string());
        map.insert("page_size", self.page_size.to_string());
        if self.page_no > 0 {
            map.insert("page_no", self.page_no.to_string());
        }
        map
    }
}
