use proa_core::{DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::books::Book;
use crate::components::book_card::BookCard;

pub struct BookPage {
    pub book: &'static Book,
}

impl<L: DataLoader> WebRenderSync<L> for BookPage {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let markdown = format!("/books/{}/md", self.book.id);
        html_sync! {
            <main>
                <p class="meta"><a href="/">"Back to shelf"</a>" | "<a href={markdown}>"Markdown"</a></p>
                {BookCard { book: self.book }}
            </main>
        }
        .render(cx)
    }
}
