use gpui::{
    div, prelude::*, rgb, App, AppContext, Length, SharedString, ViewContext, WindowOptions,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .bg(rgb(0x222222))
                    .size(Length::Definite(gpui::DefiniteLength::Absolute(
                        gpui::AbsoluteLength::Pixels(gpui::Pixels(100 as f32)),
                    )))
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .child(format!("Mouse button"))
                    .text_color(rgb(0xffffff))
                    .cursor(gpui::CursorStyle::PointingHand)
                    .on_mouse_down(gpui::MouseButton::Left, |evt, _cx| {
                        println!("Button clicked");
                        println!("Mouse button: {:?}", evt.button);
                    }),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
