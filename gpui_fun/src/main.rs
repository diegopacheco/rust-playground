use gpui::*;
use gpui_component::*;
use serde::Deserialize;

const API_KEY: &str = "RIBXT3XYTV0GM0QL";

#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Debug, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

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

fn fetch_stock_data(symbol: &str) -> Option<(f64, f64, f64)> {
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, API_KEY
    );

    let response = reqwest::blocking::get(&url).ok()?;
    let data: AlphaVantageResponse = response.json().ok()?;

    let price = data.global_quote.price.parse::<f64>().ok()?;
    let change = data.global_quote.change.parse::<f64>().ok()?;
    let change_percent_str = data.global_quote.change_percent.trim_end_matches('%');
    let change_percent = change_percent_str.parse::<f64>().ok()?;

    Some((price, change, change_percent))
}

impl StockMonitor {
    fn new() -> Self {
        let companies = vec![
            ("AAPL", "Apple"),
            ("GOOGL", "Google"),
            ("MSFT", "Microsoft"),
            ("META", "Meta"),
            ("NVDA", "Nvidia"),
            ("TSLA", "Tesla"),
            ("CMG", "Chipotle"),
        ];

        let mut stocks = Vec::new();

        for (symbol, company) in companies.iter() {
            if let Some((price, change, change_percent)) = fetch_stock_data(symbol) {
                stocks.push(Stock {
                    symbol: symbol.to_string(),
                    company: company.to_string(),
                    price,
                    change,
                    change_percent,
                });
            } else {
                stocks.push(Stock {
                    symbol: symbol.to_string(),
                    company: company.to_string(),
                    price: 0.00,
                    change: 0.00,
                    change_percent: 0.00,
                });
            }
        }

        stocks.push(Stock {
            symbol: "N/A".to_string(),
            company: "OpenAI".to_string(),
            price: 0.00,
            change: 0.00,
            change_percent: 0.00,
        });

        stocks.push(Stock {
            symbol: "N/A".to_string(),
            company: "Anthropic".to_string(),
            price: 0.00,
            change: 0.00,
            change_percent: 0.00,
        });

        Self { stocks }
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
