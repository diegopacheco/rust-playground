//! The [`View`] abstraction shared by every panel.

use crate::game::Snapshot;
use crate::render::canvas::Canvas;

/// Something that can draw a game snapshot onto a canvas.
///
/// Views are pure painters: they read a snapshot and write glyphs, holding no
/// state and owning no terminal. Composing a screen is therefore a matter of
/// listing views, and testing one is a matter of painting it onto a
/// [`crate::render::canvas::TextCanvas`] and reading the result back.
pub trait View {
    /// Draws `snapshot` onto `canvas`.
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas);
}

impl<V: View + ?Sized> View for &V {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        (**self).paint(snapshot, canvas);
    }
}

impl<V: View> View for Vec<V> {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        for view in self {
            view.paint(snapshot, canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::canvas::{Glyph, TextCanvas};
    use crate::render::color::Color;
    use crate::test_support::sample_snapshot;

    struct Mark(u16, char);

    impl View for Mark {
        fn paint(&self, _snapshot: &Snapshot, canvas: &mut dyn Canvas) {
            canvas.put(self.0, 0, Glyph::new(self.1, Color::Default));
        }
    }

    #[test]
    fn a_list_of_views_paints_every_one_of_them() {
        let mut canvas = TextCanvas::new(4, 1);
        vec![Mark(0, 'a'), Mark(2, 'c')].paint(&sample_snapshot(), &mut canvas);
        assert_eq!(canvas.row(0), "a c ");
    }

    #[test]
    fn later_views_paint_over_earlier_ones_so_overlays_win() {
        let mut canvas = TextCanvas::new(2, 1);
        vec![Mark(0, 'a'), Mark(0, 'z')].paint(&sample_snapshot(), &mut canvas);
        assert_eq!(canvas.row(0), "z ");
    }

    #[test]
    fn an_empty_composition_draws_nothing_at_all() {
        let mut canvas = TextCanvas::new(3, 1);
        Vec::<Mark>::new().paint(&sample_snapshot(), &mut canvas);
        assert_eq!(canvas.row(0), "   ");
    }
}
