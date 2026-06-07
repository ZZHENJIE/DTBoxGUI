use dtboxgui::State;

fn app_settings() -> iced::Settings {
    iced::Settings {
        vsync: false,
        ..Default::default()
    }
}

fn main() -> iced::Result {
    iced::daemon(State::new, State::update, State::view)
        .title(State::title)
        .theme(State::theme)
        .subscription(State::subscription)
        .settings(app_settings())
        .run()
}
