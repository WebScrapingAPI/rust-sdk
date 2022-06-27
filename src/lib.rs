extern crate reqwest;

use std::collections::HashMap;
use urlencoding::encode;
use reqwest::{Response, Client, header::HeaderMap};
use std::error::Error;
use std::convert::TryInto;

pub struct QueryBuilder<'a> {
    params: HashMap<&'a str, &'a str>,
    headers: HashMap<String, String>,
    body: HashMap<&'a str, &'a str>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new() -> QueryBuilder<'a> {
        QueryBuilder { 
            params: HashMap::new(), 
            headers: HashMap::new(),
            body: HashMap::new(),
        }
    }

    pub fn get_params(&self) -> HashMap<&str, &str> {
        self.params.clone()
    }

    pub fn headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn body(&mut self, body: HashMap<&'a str, &'a str>) {
        self.body = body;
    }

    pub fn get_body(&self) -> HashMap<&str, &str> {
        self.body.clone()
    }

    pub fn url(&mut self, value: &'a str) {
        self.params.insert("url", value);
    }

    pub fn get_url(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("url").unwrap();
        Ok(*result)
    }

    pub fn render_js(&mut self, value: &'a str) {
        self.params.insert("render_js", value);
    }

    pub fn get_render_js(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("render_js").unwrap();
        Ok(*result)
    }

    pub fn proxy_type(&mut self, value: &'a str) {
        self.params.insert("proxy_type", value);
    }

    pub fn get_proxy_type(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("proxy_type").unwrap();
        Ok(*result)
    }

    pub fn country(&mut self, value: &'a str) {
        self.params.insert("country", value);
    }

    pub fn get_country(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("country").unwrap();
        Ok(*result)
    }

    pub fn keep_headers(&mut self, value: &'a str) {
        self.params.insert("keep_headers", value);
    }

    pub fn get_keep_headers(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("keep_headers").unwrap();
        Ok(*result)
    }

    pub fn session(&mut self, value: &'a str) {
        self.params.insert("session", value);
    }

    pub fn get_session(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("session").unwrap();
        Ok(*result)
    }

    pub fn timeout(&mut self, value: &'a str) {
        self.params.insert("timeout", value);
    }

    pub fn get_timeout(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("timeout").unwrap();
        Ok(*result)
    }

    pub fn device(&mut self, value: &'a str) {
        self.params.insert("device", value);
    }

    pub fn get_device(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("device").unwrap();
        Ok(*result)
    }

    pub fn wait_until(&mut self, value: &'a str) {
        self.params.insert("wait_until", value);
    }

    pub fn get_wait_until(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_until").unwrap();
        Ok(*result)
    }

    pub fn wait_for(&mut self, value: &'a str) {
        self.params.insert("wait_for", value);
    }

    pub fn get_wait_for(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_for").unwrap();
        Ok(*result)
    }

    pub fn wait_for_css(&mut self, value: &'a str) {
        self.params.insert("wait_for_css", value);
    }

    pub fn get_wait_for_css(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_for_css").unwrap();
        Ok(*result)
    }

    pub fn screenshot(&mut self, value: &'a str) {
        self.params.insert("screenshot", value);
    }

    pub fn get_screenshot(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("screenshot").unwrap();
        Ok(*result)
    }

    pub fn extract_rules(&mut self, value: &'a str) {
        self.params.insert("extract_rules", value);
    }

    pub fn get_extract_rules(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("extract_rules").unwrap();
        Ok(*result)
    }

    pub fn disable_stealth(&mut self, value: &'a str) {
        self.params.insert("disable_stealth", value);
    }

    pub fn get_disable_stealth(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("disable_stealth").unwrap();
        Ok(*result)
    }

    pub fn auto_parser(&mut self, value: &'a str) {
        self.params.insert("auto_parser", value);
    }

    pub fn get_auto_parser(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("auto_parser").unwrap();
        Ok(*result)
    }

    pub fn js_instructions(&mut self, value: &'a str) {
        self.params.insert("js_instructions", value);
    }

    pub fn get_js_instructions(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("js_instructions").unwrap();
        Ok(*result)
    }
}

pub struct WebScrapingAPI<'a> {
    key: &'a str,
    client: Client,
}

impl<'a> WebScrapingAPI<'a> {
    pub fn new(api_key: &str) -> WebScrapingAPI {
        WebScrapingAPI {
            key: api_key,
            client: Client::new(),
        }
    }

    fn params_to_api_url(&self, params: HashMap<&str, &str>) -> String {
        let mut query_string: String = String::from("");

        for (key, value) in params.into_iter() {
            let mut final_value = String::from(value);

            if key == "url" {
                final_value = encode(value).into_owned();
            }

            let parameter_query_string = format!("&{}={}", key, final_value);
            query_string.push_str(&parameter_query_string);
        }

        format!("https://api.webscrapingapi.com/v1?api_key={}{}", self.key, query_string)
    }

    pub async fn get(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.get(api_url).headers(headers).send().await?;
        Ok(response)
    }

    pub async fn post(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.post(api_url).json(&query_builder.get_body()).headers(headers).send().await?;
        Ok(response)
    }

    pub async fn put(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.put(api_url).json(&query_builder.get_body()).headers(headers).send().await?;
        Ok(response)
    }
    
    pub async fn raw_get(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.get(api_url).headers(headers).send().await?;
        Ok(response)
    }

    pub async fn raw_post(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>, body: HashMap<&str, &str>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.post(api_url).json(&body).headers(headers).send().await?;
        Ok(response)
    }

    pub async fn raw_put(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>, body: HashMap<&str, &str>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.put(api_url).json(&body).headers(headers).send().await?;
        Ok(response)
    }
}