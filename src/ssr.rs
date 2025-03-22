use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use tower_http::services::ServeDir;
mod mock_server;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3005")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3005");

    axum::serve(
        listener,
        Router::new()
            .route("/", get(app_endpoint))
            .nest_service("/assets", ServeDir::new("assets"))
            .into_make_service(),
    )
    .await
    .unwrap();
}

async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let content = match mock_server::mock_server().await {
        Ok(data) => data,
        Err(e) => {
            println!("データのフェッチに失敗しました: {:?}", e);
            "フェッチエラー: サーバーからデータを取得できませんでした".to_string()
        }
    };
    
    // 成功した場合はcontent（String）をOkでラップして返す
    Ok(content)
}

async fn app_endpoint() -> Html<String> {
    let content = match fetch_data().await {
        Ok(data) => data,
        Err(e) => {
            println!("エラーが発生しました: {:?}", e);
            "エラーが発生しました".to_string()
        }
    };

    // render the rsx! macro to HTML
    let rendered = dioxus_ssr::render_element(rsx! {
        div { class: "p-4 text-blue-500", "{content}" }
    });

    // HTML全体として返す（<html>, <head>, <body> を含む）
    Html(format!(
        r#"<!DOCTYPE html>
    <html>
        <head>
            <link rel="stylesheet" href="/assets/tailwind.css">
        </head>
        <body>
            <h1 class="text-4xl text-red-500">Hello from Dioxus SSR</h1>
            <div>{rendered}</div>
        </body>
    </html>"#
    ))
    
}