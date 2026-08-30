use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const STAGE: &str =
    "{spinner:.green} {prefix:<8.bold.cyan} [{bar:34.cyan/blue}] {pos:>5}/{len:<5} {msg}";
const BYTES: &str =
    "{spinner:.green} {prefix:<8.bold.cyan} [{bar:34.cyan/blue}] {bytes:>9}/{total_bytes:<9} {msg}";
const TASK: &str = "   {prefix:<24.dim} [{bar:28.green/blue}] {human_pos:>9}/{human_len:<9} rows";

pub struct Progress {
    multi: MultiProgress,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    pub fn stage(&self, label: &str, total: u64) -> ProgressBar {
        let bar = self.multi.add(ProgressBar::new(total));
        bar.set_style(style(STAGE));
        bar.set_prefix(label.to_string());
        bar.enable_steady_tick(Duration::from_millis(120));
        bar
    }

    pub fn bytes(&self, label: &str, total: u64) -> ProgressBar {
        let bar = self.multi.add(ProgressBar::new(total));
        bar.set_style(style(BYTES));
        bar.set_prefix(label.to_string());
        bar.enable_steady_tick(Duration::from_millis(120));
        bar
    }

    pub fn task(&self, label: &str, total: u64) -> ProgressBar {
        let bar = self.multi.add(ProgressBar::new(total));
        bar.set_style(style(TASK));
        bar.set_prefix(shorten(label, 24));
        bar
    }

    pub fn note(&self, line: &str) {
        if self.multi.is_hidden() {
            eprintln!("{line}");
        } else {
            let _ = self.multi.println(line);
        }
    }
}

fn style(template: &str) -> ProgressStyle {
    ProgressStyle::with_template(template)
        .expect("progress template is valid")
        .progress_chars("━━╌")
}

fn shorten(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    let head: String = text.chars().take(width.saturating_sub(1)).collect();
    format!("{head}…")
}
