use proa_core::text::text;
use proa_core::{DataLoader, MdRenderSync, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, md_sync};

use crate::books::Book;

pub struct BookCard {
    pub book: &'static Book,
}

impl BookCard {
    fn href(&self) -> String {
        format!("/books/{}", self.book.id)
    }
}

impl<L: DataLoader> WebRenderSync<L> for BookCard {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let href = self.href();
        html_sync! {
            <article class="card">
                <h2><a href={href}>{text(self.book.title)}</a></h2>
                <p class="meta">{text(self.book.author)}" - "{self.book.year}</p>
                <p>{text(self.book.summary)}</p>
            </article>
        }
        .render(cx)
    }
}

impl<L: DataLoader> MdRenderSync<L> for BookCard {
    fn render_md(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let href = self.href();
        md_sync! { r###"
            ## [{self.book.title}]({href})

            {self.book.author} - {self.book.year}

            {self.book.summary}
            "###
        }
        .render_md(cx)
    }
}
