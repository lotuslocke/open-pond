use open_pond_api::{get_public_key, new_interface, RequestEndpoint, ResponseEndpoint};
use open_pond_protocol::parse_config;

use std::env;
use web_view::*;

pub struct ClientHandler {
    pub requester: RequestEndpoint,
    pub response: ResponseEndpoint,
}

fn main() {
    // Attempt parse of Open Pond configuration file
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());
    let config = parse_config(config_file).unwrap();

    // Create application interface objects for making requests and servicing requests
    let (request_endpoint, _service_endpoint, response_endpoint) =
        new_interface(config.settings, 0).unwrap();

    let user_data = ClientHandler {
        requester: request_endpoint,
        response: response_endpoint,
    };

    web_view::builder()
        .title("Beavercreek")
        .content(Content::Html(include_str!("beavercreek.html")))
        .size(800, 600)
        .resizable(true)
        .user_data(user_data)
        .invoke_handler(handler)
        .run()
        .unwrap();
}

fn handler(webview: &mut WebView<ClientHandler>, arg: &str) -> WVResult {
    if arg == "getKey" {
        let key = get_public_key().unwrap();
        let script = format!("displayKey(\"{}\")", key);
        webview.eval(&script)?;
    }
    Ok(())
}
