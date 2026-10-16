use proa_core::{MdRenderSync, WebContext, WebRenderSync};

use crate::books::{BOOKS, Book, find_book};
use crate::components::book_card::BookCard;
use crate::components::book_list::BookList;

fn html<C: WebRenderSync>(component: C) -> String {
    let mut cx = WebContext::new();
    component.render(&mut cx).unwrap();
    cx.into_output().into_string()
}

fn markdown<C: MdRenderSync>(component: C) -> String {
    let mut cx = WebContext::new();
    component.render_md(&mut cx).unwrap();
    cx.into_output().into_string()
}

#[test]
fn one_card_renders_the_same_book_on_both_targets() {
    let book = find_book(1).unwrap();
    let page = html(BookCard { book });
    let doc = markdown(BookCard { book });
    for value in [book.author, &book.year.to_string(), "/books/1"] {
        assert!(page.contains(value), "html missing {value}");
        assert!(doc.contains(value), "markdown missing {value}");
    }
    assert!(page.contains("<a href=\"/books/1\">The Rust Programming Language</a>"));
    assert!(doc.starts_with("## [The Rust Programming Language](/books/1)"));
}

#[test]
fn markdown_target_has_no_html_markup() {
    let doc = markdown(BookList);
    assert!(!doc.contains('<'), "markdown leaked html: {doc}");
}

#[test]
fn markdown_escapes_special_characters_from_data() {
    let doc = markdown(BookCard { book: find_book(2).unwrap() });
    assert!(doc.contains("Designing Data\\-Intensive Applications"));
}

#[test]
fn html_escapes_markup_coming_from_data() {
    static HOSTILE: Book = Book {
        id: 7,
        title: "<script>alert(1)</script>",
        author: "a & b",
        year: 2000,
        summary: "x",
    };
    let page = html(BookCard { book: &HOSTILE });
    assert!(!page.contains("<script>"));
    assert!(page.contains("&lt;script&gt;"));
    assert!(page.contains("a &amp; b"));
}

#[test]
fn list_shows_every_book_in_catalog_order_on_both_targets() {
    let page = html(BookList);
    let doc = markdown(BookList);
    let positions = |text: &str| -> Vec<usize> {
        BOOKS.iter().map(|book| text.find(&format!("/books/{}", book.id)).unwrap()).collect()
    };
    let in_order = |p: Vec<usize>| p.windows(2).all(|w| w[0] < w[1]);
    assert!(in_order(positions(&page)));
    assert!(in_order(positions(&doc)));
}

#[test]
fn unknown_book_is_not_found() {
    assert!(find_book(0).is_none());
    assert!(find_book(99).is_none());
}
