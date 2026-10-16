use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use proa_core::to_async;
use proa_framework_axum::{
    IslandManifest, MarkdownAlternate, Route, RouteContextLayer, RouteResponse, explicit_markdown,
    negotiate, router_from_spec,
};

use crate::books::find_book;
use crate::components::book_card::BookCard;
use crate::components::book_list::BookList;
use crate::components::book_page::BookPage;
use crate::layout::RootDocument;

const STYLES: &str = include_str!("../public/styles.css");

pub fn router() -> Router {
    let (router, metadata) = router_from_spec(vec![
        Route::endpoint("/", get(home)),
        Route::endpoint("/index.md", get(home_md)),
        Route::endpoint("/books/:id", get(book)),
        Route::endpoint("/books/:id/md", get(book_md)),
    ]);
    let manifest = Arc::new(IslandManifest {
        islands: HashMap::new(),
        assets: Default::default(),
        import_map: HashMap::new(),
        ssr_bundle: None,
    });
    router
        .route("/healthz", get(|| async { "ok" }))
        .route("/styles.css", get(styles))
        .layer(RouteContextLayer::new(manifest, metadata))
}

async fn home(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    let html = RouteResponse::ssr_async(RootDocument {
        title: "Proa Bookshelf",
        content: to_async(BookList),
    });
    negotiate(&headers, html, BookList, MarkdownAlternate::new("/index.md")).map_err(internal)
}

async fn home_md() -> Result<impl IntoResponse, StatusCode> {
    explicit_markdown(BookList).map_err(internal)
}

async fn book(Path(id): Path<u32>) -> Result<impl IntoResponse, StatusCode> {
    let book = find_book(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(RouteResponse::ssr_async(RootDocument {
        title: book.title,
        content: to_async(BookPage { book }),
    }))
}

async fn book_md(Path(id): Path<u32>) -> Result<impl IntoResponse, StatusCode> {
    let book = find_book(id).ok_or(StatusCode::NOT_FOUND)?;
    explicit_markdown(BookCard { book }).map_err(internal)
}

async fn styles() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], STYLES)
}

fn internal<E>(_: E) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
