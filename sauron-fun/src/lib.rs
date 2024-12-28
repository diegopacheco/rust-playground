use sauron::{
    html::text, html::units::px, jss, node, wasm_bindgen, Application, Cmd, Node, Program,
};

enum Msg {
    Increment,
    Decrement,
    Reset,
}

struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Application for App {

    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body":{
                font_family: "verdana, arial, monospace",
            },

            "main":{
                width: px(30),
                height: px(100),
                margin: "auto",
                text_align: "center",
            },

            "input, .count":{
                font_size: px(40),
                padding: px(30),
                margin: px(30),
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}