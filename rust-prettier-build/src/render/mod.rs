//! Turning a snapshot into glyphs, and glyphs into a terminal.
//!
//! Every panel paints onto the [`canvas::Canvas`] trait rather than onto a
//! terminal, so the entire visual layer is exercised in tests against an
//! in-memory canvas and asserted on character by character.

pub mod board_panel;
pub mod canvas;
pub mod chrome;
pub mod color;
pub mod help_panel;
pub mod overlay_panel;
pub mod screen;
pub mod sidebar_panel;
pub mod terminal;
pub mod view;

pub use board_panel::BoardPanel;
pub use canvas::{Canvas, Glyph, TextCanvas};
pub use color::Color;
pub use help_panel::HelpPanel;
pub use overlay_panel::OverlayPanel;
pub use screen::Screen;
pub use sidebar_panel::SidebarPanel;
pub use terminal::TerminalCanvas;
pub use view::View;
