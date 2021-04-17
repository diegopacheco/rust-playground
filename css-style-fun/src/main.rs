use css_style::{Style, Color, unit::{ms, px}};
fn main() {
    let style = Style::default()
        .and_transition(|conf| {
            conf
                .insert("opacity", |conf| conf.duration(ms(150.)).ease())
                .insert("transform", |conf| conf.duration(ms(150.)).ease())
                .insert("visibility", |conf| conf.duration(ms(150.)).ease())
        })
        .and_position(|conf| conf.absolute())
        .and_background(|conf| conf.color(Color::White))
        .and_border(|conf| {
            conf.none()
                .width(px(0))
                .radius(px(4))
        })
        .and_padding(|conf| conf.x(px(4)).y(px(2)))
        .and_margin(|conf| conf.top(px(2)))
        .insert("box-shadow", "0 2px 8px rgba(0, 35, 11, 0.15)");

    println!("{}", style);
}
