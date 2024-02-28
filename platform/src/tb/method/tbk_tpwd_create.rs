use super::IRequest;
use std::collections::HashMap;

// taobao.tbk.tpwd.create( 淘宝客-公用-淘口令生成 )
// https://open.taobao.com/api.htm?docId=31127&docType=2

pub struct CreateTPwd {
    pub url: String, // 联盟官方渠道获取的淘客推广链接
    pub text: String,
    pub logo: String,
    pub ext: String,
    pub user_id: String,
}
impl CreateTPwd {
    pub fn new(url: &str) -> Self {
        CreateTPwd {
            url: url.to_string(),
            text: "".to_string(),
            logo: "".to_string(),
            ext: "".to_string(),
            user_id: "".to_string(),
        }
    }
}
impl IRequest for CreateTPwd {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.tpwd.create".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("url", self.url.clone());

        if self.text.len() > 0 {
            map.insert("text", self.text.clone());
        }
        if self.logo.len() > 0 {
            map.insert("logo", self.logo.clone());
        }
        if self.ext.len() > 0 {
            map.insert("ext", self.ext.clone());
        }
        if self.user_id.len() > 0 {
            map.insert("user_id", self.user_id.clone());
        }
        map
    }
}
