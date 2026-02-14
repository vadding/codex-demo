use gpui::{App, AppContext, Context, Render, WindowOptions, div};

struct HelloWindow;

impl Render for HelloWindow {
    fn render(&mut self, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        div().child("Hello from GPUI window!")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| cx.new(|_| HelloWindow))
            .expect("failed to open window");
    });
}
