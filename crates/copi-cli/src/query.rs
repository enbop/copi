use copi_core::generated::RequestBody;
use copi_core::generated::ResponseBody;
use copi_core::generated::request_body;
use prost::Message;

fn parse_message(args: Vec<String>) -> Result<request_body::Message, serde_json::Error> {
    let type_name = &args[0];
    let mut map = serde_json::Map::new();

    let mut i = 1;
    while i < args.len() {
        if !args[i].contains('=') {
            panic!("Invalid argument format: {}", args[i]);
        }
        let parts: Vec<&str> = args[i].splitn(2, '=').collect();
        if parts.len() != 2 {
            panic!("Invalid argument format: {}", args[i]);
        }
        let key = parts[0].to_string();
        let value = parts[1].to_string();
        if key.is_empty() || value.is_empty() {
            panic!("Key or value cannot be empty: {}", args[i]);
        }

        let val: serde_json::Value = match serde_json::from_str(&value) {
            Ok(v) => v,
            Err(_) => serde_json::Value::String(value.clone()),
        };

        map.insert(key.clone(), val);
        i += 1;
    }

    serde_json::from_value(serde_json::json!({
        type_name: map
    }))
}

pub async fn start_query(args: Vec<String>) {
    let parsed = parse_message(args).unwrap_or_else(|e| {
        panic!("Failed to parse message: {}", e);
    });

    let mut request_body = RequestBody::default();
    request_body.message = Some(parsed);
    let data = request_body.encode_to_vec();

    // reqwest send request_body protoful message
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8899/query")
        .header("Content-Type", "application/protobuf")
        .body(data)
        .send()
        .await
        .expect("Failed to send request");
    if !response.status().is_success() {
        panic!("Request failed with status: {}", response.status());
    }
    let response_bytes = response.bytes().await.expect("Failed to read response");
    let response_body =
        ResponseBody::decode(response_bytes.as_ref()).expect("Failed to decode response body");
    println!("Response: {:?}", response_body);
}
