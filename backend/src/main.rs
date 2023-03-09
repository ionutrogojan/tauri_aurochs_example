#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{ WindowUrl, utils::{ config::AppUrl, assets::EmbeddedAssets } };
use aurochs::Document;

fn aurochs_context() -> tauri::Context<EmbeddedAssets> {
    let mut aurochs_context = tauri::generate_context!();
    let window_url = WindowUrl::External("aurochs://localhost".parse().expect("error while parsing url"));
    aurochs_context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    aurochs_context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    return aurochs_context
}

fn aurochs_protocol(text: &str) -> Result<tauri::http::Response, Box<(dyn std::error::Error + 'static)>> {
  let buf = Vec::from(text.as_bytes());
  tauri::http::ResponseBuilder::new()
    .header("Origin", "*")
    .mimetype("text/html")
    .header("Content-Length", buf.len())
    .status(200)
    .body(buf)
}

fn aurochs_html() -> String {
  let mut css = Document::create_element("link");
  css.set_attribute_list(vec![("rel", "stylesheet"), ("href", "http://127.0.0.1:1430/style.css")]);
  let mut head = Document::create_element("head");
  head.append_child(css);
  let mut h1 = Document::create_element("h1");
  h1.inner_text("Hello world, from Aurochs!");
  let mut body = Document::create_element("body");
  body.append_child(h1);
  let mut html = Document::create_element("html");
  html.set_attribute("lang", "en");
  html.append_child_list(vec![head, body]);
  html.render()
}

fn main() {
  tauri::Builder::default()
    .register_uri_scheme_protocol("aurochs", move |_app, _req| { aurochs_protocol(&aurochs_html()) })
    .invoke_handler(tauri::generate_handler![])
    .run(aurochs_context())
    .expect("error while running tauri application");
}