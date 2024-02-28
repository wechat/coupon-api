use chrono;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use utils;

const API_URL: &str = "https://eco.taobao.com/router/rest";

pub struct TaobaoRequest {
    method: String,      // 接口名称
    app_key: String,     // 应用key
    app_secret: String,  // 应用secret
    session: String,     // 不需要授权
    timestamp: String,   // 时间戳
    v: String,           // 版本号
    sign_method: String, // 签名算法
    sign: String,        // 签名
    format: String,      // 响应格式
    simplify: bool,      // 是否简化响应
}
impl TaobaoRequest {
    pub fn new(app_key: &str, app_secret: &str, session: &str) -> Self {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        TaobaoRequest {
            method: "".to_string(),
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
            session: session.to_string(),
            timestamp: timestamp,
            v: "2.0".to_string(),
            sign_method: "hmac-sha256".to_string(),
            sign: "".to_string(),
            format: "json".to_string(),
            simplify: true,
        }
    }
    // 发送请求
    pub async fn send_request(
        &mut self,
        req: impl super::IRequest,
    ) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        if self.app_key.len() == 0 || self.app_secret.len() == 0 {
            return Ok(HashMap::new());
        }
        self.method = req.get_method_name();
        let req_param = req.get_map_param();
        self.make_sign(req_param.clone());

        let response = Client::new()
            .request(
                reqwest::Method::from_bytes("POST".as_bytes()).unwrap(),
                reqwest::Url::parse(API_URL).expect("not a valid url"),
            )
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            // .body(req.get_req_body())
            .query(&self.get_map_param())
            .form(&req_param.clone())
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        // println!("response: {:?}", response);
        Ok(response)
    }

    // 获取公共参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map = HashMap::from([
            ("method", self.method.clone()),
            ("app_key", self.app_key.clone()),
            ("v", self.v.clone()),
            ("sign_method", self.sign_method.clone()),
            ("format", self.format.clone()),
            ("simplify", self.simplify.to_string()),
            ("timestamp", self.timestamp.clone()),
        ]);
        if self.sign.len() > 0 {
            map.insert("sign", self.sign.clone());
        }
        if self.session.len() > 0 {
            map.insert("session", self.session.clone());
        }
        map
    }

    // 生成签名
    fn make_sign(&mut self, req_param: HashMap<&str, String>) {
        let pub_param = self.get_map_param();
        let mut key_list: Vec<&str> = Vec::new();
        let mut all_param_map: HashMap<&str, String> = HashMap::new();

        for (key, val) in pub_param {
            all_param_map.insert(key, val);
            key_list.push(key)
        }
        for (key, val) in req_param {
            all_param_map.insert(key, val);
            key_list.push(key)
        }
        key_list.sort();
        let mut sign_str = String::new();
        for key in key_list {
            let value = all_param_map[key].clone();
            sign_str = format!("{}{}{}", sign_str, key, value)
        }

        self.sign = utils::hmac_sha256_hex(
            sign_str.as_str().as_bytes(),
            self.app_secret.as_str().as_bytes(),
        )
        .to_ascii_uppercase();
    }
}
