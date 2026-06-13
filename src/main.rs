use dtboxgui::Application;

fn app_settings() -> iced::Settings {
    iced::Settings {
        id: Some("zzhenjie.detbox_gui".to_string()),
        vsync: false,
        ..Default::default()
    }
}

fn main() -> iced::Result {
    iced::daemon(Application::new, Application::update, Application::view)
        .title(Application::title)
        .theme(Application::theme)
        .subscription(Application::subscription)
        .settings(app_settings())
        .run()
}
