use proa_core::{DataLoader, MdRenderSync, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, md_sync};

use crate::books::BOOKS;
use crate::components::book_card::BookCard;

pub struct BookList;

impl<L: DataLoader> WebRenderSync<L> for BookList {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <main>
                <h1>"Proa Bookshelf"</h1>
                <p class="meta">"Same components, two targets: this page and "<a href="/index.md">"/index.md"</a></p>
                {for book in BOOKS.iter() {
                    html_sync! { {BookCard { book }} }
                }}
            </main>
        }
        .render(cx)
    }
}

impl<L: DataLoader> MdRenderSync<L> for BookList {
    fn render_md(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        md_sync! { r###"
            # Proa Bookshelf
            "###
        }
        .render_md(cx)?;
        for book in BOOKS.iter() {
            cx.md_block_boundary()?;
            BookCard { book }.render_md(cx)?;
        }
        Ok(())
    }
}
