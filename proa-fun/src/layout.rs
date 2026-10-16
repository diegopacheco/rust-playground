use std::future::Future;

use proa_core::{DataLoader, RenderOutcome, WebContext, WebRender, WriteError, raw_text_ok};
use proa_macros::html;

pub struct RootDocument<C> {
    pub title: &'static str,
    pub content: C,
}

impl<L, C> WebRender<L> for RootDocument<C>
where
    L: DataLoader,
    C: WebRender<L> + Send + 'static,
{
    fn render(
        self,
        cx: &mut WebContext<L>,
    ) -> RenderOutcome<impl Future<Output = Result<(), WriteError>>> {
        RenderOutcome::pending(async move {
            let RootDocument { title, content } = self;
            html! {
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="utf-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <title>{raw_text_ok(title)}</title>
                        <link rel="stylesheet" href="/styles.css" />
                    </head>
                    <body>{content}</body>
                </html>
            }
            .render(cx)
            .resolve()
            .await
        })
    }
}
