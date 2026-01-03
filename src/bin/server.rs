//! SSR server (Option A): SSR for landing/legal pages, SPA for `/app`.
//!
//! Usage (local):
//! - Build web assets first (required for CSS/JS links):
//!   - `dx build --release --platform web`
//! - Run server:
//!   - `cargo run --features server --bin server`
//!
//! Environment:
//! - `PUBLIC_DIR`: directory containing `index.html` + `assets/` (default: `target/dx/remind-me-pwa/release/web/public`)
//! - `HOST`: bind host (default: `127.0.0.1`)
//! - `PORT`: bind port (default: `8080`)

#![cfg(feature = "server")]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::http::{HeaderMap, HeaderValue, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use dioxus::prelude::*;

use remind_me_pwa::components::{LandingPage, PrivacyPolicyPage, TermsOfUsePage};
use remind_me_pwa::i18n::{use_init_i18n, use_i18n, Locale};
use remind_me_pwa::router::Route;

#[derive(Clone)]
struct AssetIndex {
    public_dir: PathBuf,
    base_path: String,
    // Map logical names -> hashed filename under /assets/
    // e.g. "base.css" -> "base-dxh....css"
    assets: HashMap<&'static str, String>,
    entry_js: String,
}

impl AssetIndex {
    fn new(public_dir: PathBuf, base_path: String) -> Self {
        let mut assets = HashMap::new();

        // CSS
        for (k, prefix, ext) in [
            ("base.css", "base-", ".css"),
            ("components.css", "components-", ".css"),
            ("app.css", "app-", ".css"),
            ("landing.css", "landing-", ".css"),
            ("layout.css", "layout-", ".css"),
            ("utilities.css", "utilities-", ".css"),
            ("responsive.css", "responsive-", ".css"),
        ] {
            if let Some(f) = find_asset(&public_dir, prefix, ext) {
                assets.insert(k, f);
            }
        }

        // Favicons/icons (best-effort)
        if let Some(f) = find_asset(&public_dir, "favicon-32x32-", ".avif") {
            assets.insert("favicon-32.avif", f);
        }
        if let Some(f) = find_asset(&public_dir, "favicon-16x16-", ".avif") {
            assets.insert("favicon-16.avif", f);
        }
        if let Some(f) = find_asset(&public_dir, "favicon-", ".ico") {
            assets.insert("favicon.ico", f);
        }

        let entry_js = find_asset(&public_dir, "remind-me-pwa-", ".js")
            .unwrap_or_else(|| "remind-me-pwa.js".to_string());

        Self {
            public_dir,
            base_path,
            assets,
            entry_js,
        }
    }

    fn asset_url(&self, hashed_filename: &str) -> String {
        // Served via /assets/*
        format!("{}{}", self.base_path, format!("/assets/{}", hashed_filename))
    }
}

fn find_asset(public_dir: &Path, prefix: &str, ext: &str) -> Option<String> {
    let assets_dir = public_dir.join("assets");
    let Ok(entries) = std::fs::read_dir(assets_dir) else {
        return None;
    };

    for ent in entries.flatten() {
        let path = ent.path();
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if name.starts_with(prefix) && name.ends_with(ext) {
            return Some(name.to_string());
        }
    }
    None
}

#[component]
fn SsrRoute(locale: Locale, route: Route) -> Element {
    // Provide i18n to the subtree and force the desired locale.
    use_init_i18n();
    let mut i18n = use_i18n();
    {
        let mut ctx = i18n.write();
        ctx.set_locale(locale.clone());
    }

    rsx! {
        match route {
            Route::Landing => rsx! {
                LandingPage {
                    on_enter_app: move |_| {},
                    on_navigate: move |_| {},
                }
            },
            Route::PrivacyPolicy => rsx! {
                PrivacyPolicyPage {
                    on_enter_app: move |_| {},
                    on_jump: move |_| {},
                    on_navigate: move |_| {},
                }
            },
            Route::TermsOfUse => rsx! {
                TermsOfUsePage {
                    on_enter_app: move |_| {},
                    on_jump: move |_| {},
                    on_navigate: move |_| {},
                }
            },
            // `/app` is kept as SPA (no SSR for localStorage-heavy UI)
            Route::App => rsx! {
                div { class: "app-shell-placeholder" }
            },
        }
    }
}

fn route_seo(route: &Route, locale: &Locale) -> (String, String) {
    // Keep this consistent with `src/app.rs` (get_route_seo).
    match (route, locale) {
        (Route::Landing, Locale::ZhHans) => (
            "提醒我 PWA - 您的个人提醒助手".to_string(),
            "一个简单优雅的提醒应用，帮助您保持条理。支持离线使用，可安装到设备，并保护您的数据隐私。"
                .to_string(),
        ),
        (Route::Landing, Locale::ZhHant) => (
            "提醒我 PWA - 您的個人提醒助手".to_string(),
            "一個簡單優雅的提醒應用，幫助您保持條理。支援離線使用，可安裝到裝置，並保護您的資料隱私。"
                .to_string(),
        ),
        (Route::Landing, _) => (
            "Remind Me PWA - Your Personal Reminder Assistant".to_string(),
            "A beautiful and functional Progressive Web App to help you manage your reminders. Works offline, installs on your device, and keeps your data private."
                .to_string(),
        ),
        (Route::PrivacyPolicy, _) => (
            "Remind Me - Privacy Policy".to_string(),
            "Privacy Policy for Remind Me PWA. Reminders are stored locally on your device and are not collected by us.".to_string(),
        ),
        (Route::TermsOfUse, _) => (
            "Remind Me - Terms of Use".to_string(),
            "Terms of Use for Remind Me PWA. Free, open-source, offline-first reminder app.".to_string(),
        ),
        (Route::App, Locale::ZhHans) => (
            "提醒我 - 管理您的提醒事项".to_string(),
            "一个简单优雅的提醒应用，帮助您保持条理。支持离线使用，数据存储在本地设备。".to_string(),
        ),
        (Route::App, Locale::ZhHant) => (
            "提醒我 - 管理您的提醒事項".to_string(),
            "一個簡單優雅的提醒應用，幫助您保持條理。支援離線使用，資料儲存在本地裝置。".to_string(),
        ),
        (Route::App, _) => (
            "Remind Me - Manage Your Reminders".to_string(),
            "A simple and elegant reminder app to help you stay organized. Works offline, data stored locally on your device.".to_string(),
        ),
    }
}

fn render_ssr_html(asset_index: &AssetIndex, locale: &Locale, route: &Route) -> String {
    let (title, description) = route_seo(route, locale);
    let lang = locale.as_str();

    let mut vdom = VirtualDom::new_with_props(
        SsrRoute,
        SsrRouteProps {
            locale: locale.clone(),
            route: route.clone(),
        },
    );
    vdom.rebuild_in_place();
    let body_markup = dioxus_ssr::render(&vdom);

    let css_order = [
        "base.css",
        "components.css",
        "app.css",
        "landing.css",
        "layout.css",
        "utilities.css",
        "responsive.css",
    ];

    let css_links = css_order
        .iter()
        .filter_map(|k| asset_index.assets.get(k))
        .map(|f| format!(r#"<link rel="stylesheet" href="{}">"#, asset_index.asset_url(f)))
        .collect::<Vec<_>>()
        .join("\n        ");

    let mut favicon_parts: Vec<String> = Vec::new();
    if let Some(f) = asset_index.assets.get("favicon-32.avif") {
        favicon_parts.push(format!(
            r#"<link rel="icon" type="image/avif" sizes="32x32" href="{}">"#,
            asset_index.asset_url(f)
        ));
    }
    if let Some(f) = asset_index.assets.get("favicon-16.avif") {
        favicon_parts.push(format!(
            r#"<link rel="icon" type="image/avif" sizes="16x16" href="{}">"#,
            asset_index.asset_url(f)
        ));
    }
    if let Some(f) = asset_index.assets.get("favicon.ico") {
        favicon_parts.push(format!(
            r#"<link rel="shortcut icon" type="image/x-icon" href="{}">"#,
            asset_index.asset_url(f)
        ));
    }
    let favicon_links = favicon_parts.join("\n        ");

    let entry_js_url = format!("{}{}", asset_index.base_path, format!("/assets/{}", asset_index.entry_js));

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title}</title>
    <meta name="description" content="{description}">

    {favicon_links}
    {css_links}
  </head>
  <body>
    <div id="main">{body_markup}</div>
    <script type="module" async src="{entry_js_url}"></script>
  </body>
</html>
"#
    )
}

fn parse_locale_and_route(path: &str) -> (Locale, Route) {
    // Accept:
    // - /en/
    // - /zh-Hans/privacy
    // - /terms
    // - /app
    // Also accept GitHub Pages style hashes but these are mostly for the client.
    let trimmed = path.trim_start_matches('#');
    let parts: Vec<&str> = trimmed
        .trim_start_matches('/')
        .split('/')
        .filter(|p| !p.is_empty())
        .collect();

    let (maybe_locale, rest) = match parts.as_slice() {
        [first, rest @ ..] if *first == "en" || *first == "zh" || first.starts_with("zh-") => {
            (Some(*first), rest)
        }
        _ => (None, parts.as_slice()),
    };

    let locale = maybe_locale.map(Locale::from_str).unwrap_or(Locale::En);

    let route = match rest.first().copied().unwrap_or("") {
        "app" => Route::App,
        "privacy" => Route::PrivacyPolicy,
        "terms" => Route::TermsOfUse,
        _ => Route::Landing,
    };

    (locale, route)
}

async fn ssr_handler(
    axum::extract::State(state): axum::extract::State<Arc<AssetIndex>>,
    uri: Uri,
) -> impl IntoResponse {
    let path = uri.path();
    let (locale, route) = parse_locale_and_route(path);

    // SSR only for landing/legal; `/app` stays SPA but we still return a useful shell with SEO tags.
    let html = render_ssr_html(&state, &locale, &route);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));
    (headers, Html(html))
}

fn build_router(asset_index: Arc<AssetIndex>) -> Router {
    let public_dir = asset_index.public_dir.clone();
    let assets_dir = public_dir.join("assets");

    // Serve hashed assets under /assets/*
    let assets_service = ServeDir::new(assets_dir);

    // Serve index/SSR for all non-asset paths (SPA + SSR)
    Router::new()
        .nest_service("/assets", assets_service)
        .route("/*path", get(ssr_handler))
        .with_state(asset_index)
}

#[tokio::main]
async fn main() {
    let public_dir = std::env::var("PUBLIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target/dx/remind-me-pwa/release/web/public"));

    // Optional base path if you deploy under a subdirectory.
    // Example: "/remind-me-pwa"
    let base_path = std::env::var("BASE_PATH").unwrap_or_default();
    let base_path = base_path.trim_end_matches('/').to_string();

    let asset_index = Arc::new(AssetIndex::new(public_dir, base_path));
    let app = build_router(asset_index);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("HOST/PORT should form a valid socket address");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind server");

    println!("SSR server listening on http://{}", addr);
    axum::serve(listener, app).await.expect("Server crashed");
}


