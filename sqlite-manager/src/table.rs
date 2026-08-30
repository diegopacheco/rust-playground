const LIMIT: usize = 48;

pub struct Grid {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Grid {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }

    pub fn push(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn render(&self) -> String {
        let widths = self.widths();
        let mut out = String::new();

        out.push_str(&rule(&widths, '┌', '┬', '┐'));
        out.push_str(&line(&self.headers, &widths));
        out.push_str(&rule(&widths, '├', '┼', '┤'));
        for row in &self.rows {
            out.push_str(&line(row, &widths));
        }
        out.push_str(&rule(&widths, '└', '┴', '┘'));
        out
    }

    fn widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| width(h)).collect();
        for row in &self.rows {
            for (index, cell) in row.iter().enumerate() {
                if index < widths.len() {
                    widths[index] = widths[index].max(width(cell));
                }
            }
        }
        widths
    }
}

pub fn cell(text: &str) -> String {
    let flat = text.replace(['\n', '\r'], " ");
    if width(&flat) <= LIMIT {
        return flat;
    }
    let head: String = flat.chars().take(LIMIT - 1).collect();
    format!("{head}…")
}

fn width(text: &str) -> usize {
    text.chars().count()
}

fn rule(widths: &[usize], left: char, middle: char, right: char) -> String {
    let parts: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
    format!("{left}{}{right}\n", parts.join(&middle.to_string()))
}

fn line(cells: &[String], widths: &[usize]) -> String {
    let mut out = String::from("│");
    for (index, size) in widths.iter().enumerate() {
        let text = cells.get(index).map(String::as_str).unwrap_or("");
        let padding = size.saturating_sub(width(text));
        out.push_str(&format!(" {text}{} │", " ".repeat(padding)));
    }
    out.push('\n');
    out
}
