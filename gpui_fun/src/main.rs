use gpui::*;
use gpui_component::*;

struct Stock {
    symbol: String,
    company: String,
    price: f64,
    change: f64,
    change_percent: f64,
}

struct StockMonitor {
    stocks: Vec<Stock>,
}

impl StockMonitor {
    fn new() -> Self {
        Self {
            stocks: vec![
                Stock { symbol: "AAPL".to_string(), company: "Apple".to_string(), price: 178.50, change: 2.30, change_percent: 1.31 },
                Stock { symbol: "GOOGL".to_string(), company: "Google".to_string(), price: 141.20, change: -0.80, change_percent: -0.56 },
                Stock { symbol: "MSFT".to_string(), company: "Microsoft".to_string(), price: 378.90, change: 4.50, change_percent: 1.20 },
                Stock { symbol: "META".to_string(), company: "Meta".to_string(), price: 485.30, change: 6.70, change_percent: 1.40 },
                Stock { symbol: "OPENAI".to_string(), company: "OpenAI".to_string(), price: 0.00, change: 0.00, change_percent: 0.00 },
                Stock { symbol: "ANTHR".to_string(), company: "Anthropic".to_string(), price: 0.00, change: 0.00, change_percent: 0.00 },
                Stock { symbol: "NVDA".to_string(), company: "Nvidia".to_string(), price: 875.60, change: 15.20, change_percent: 1.77 },
                Stock { symbol: "TSLA".to_string(), company: "Tesla".to_string(), price: 242.80, change: -3.40, change_percent: -1.38 },
                Stock { symbol: "CMG".to_string(), company: "Chipotle".to_string(), price: 58.75, change: 1.25, change_percent: 2.17 },
            ],
        }
    }
}

impl Render for StockMonitor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .p_4()
            .child(
                div()
                    .text_xl()
                    .font_bold()
                    .text_color(rgb(0xffffff))
                    .mb_4()
                    .child("Stock Monitor")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(self.stocks.iter().map(|stock| {
                        let color = if stock.change >= 0.0 { rgb(0x22c55e) } else { rgb(0xef4444) };
                        let sign = if stock.change >= 0.0 { "+" } else { "" };

                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_3()
                            .bg(rgb(0x2a2a2a))
                            .rounded_md()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_color(rgb(0xffffff))
                                            .font_bold()
                                            .child(format!("{} - {}", stock.symbol, stock.company))
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x9ca3af))
                                            .text_sm()
                                            .child(format!("${:.2}", stock.price))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_end()
                                    .child(
                                        div()
                                            .text_color(color)
                                            .font_bold()
                                            .child(format!("{}{:.2}", sign, stock.change))
                                    )
                                    .child(
                                        div()
                                            .text_color(color)
                                            .text_sm()
                                            .child(format!("({}{:.2}%)", sign, stock.change_percent))
                                    )
                            )
                    }))
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        gpui_component::init(cx);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| StockMonitor::new())
        }).unwrap();
    });
}
