use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};
#[wasm_bindgen]
extern "C" {
    // pub fn alert(s: &str);
    // #[wasm_bindgen(js_namespace = console)]

}
use serde::{Serialize};
#[derive(Serialize)]
struct UserCredentials {
    user_id: String,
    // user_pw: String,
}

#[wasm_bindgen]
pub async fn run(test:&str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    let credentials = UserCredentials {
        user_id: test.to_string(),
        // user_pw: "1234".to_string(),
    };
    let json_body = serde_json::to_string(&credentials).unwrap();

    // let json_body = r#"{"user_id":"hkh3045","user_pw":"1234"}"#;

    let json_js_value = wasm_bindgen::JsValue::from_str(&json_body);
    opts.set_body(&json_js_value);


    let url = format!("http://localhost:8080/ip");

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?; // 수정된 부분
    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;
    // console::log_1(&format!("{:?}", json).into());


    Ok(json)
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     let window = window().unwrap();
//     window.alert_with_message(name);
// }
// #[wasm_bindgen]
// pub fn log(name: &str) {
//     console::log_1(&format!("{}", name).into());
// }
