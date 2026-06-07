use crate::IntoMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Add,
    Subtract,
    OpenSettings,
}

pub struct MainWindow {
    pub id: iced::window::Id,
    pub count: i32,
}

impl MainWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self { id, count: 0 }
    }
}

impl IntoMessage for MainWindow {
    type Message = Message;

    fn into_message(self, message: Self::Message) -> crate::Message {
        crate::Message::MainWindowMessage(self.id.clone(), message)
    }
}

impl crate::Window for MainWindow {
    fn settings(settings: &crate::Settings) -> iced::window::Settings {
        let mut window_settings = iced::window::Settings::default();
        window_settings.exit_on_close_request = true;
        window_settings
    }
    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut crate::Settings,
    ) -> iced::Task<crate::Message> {
        match message {
            Message::Add => {
                self.count += 1;
                iced::Task::none()
            }
            Message::Subtract => {
                self.count -= 1;
                iced::Task::none()
            }
            Message::OpenSettings => iced::Task::done(crate::Message::OpenSettingsWindow),
            _ => iced::Task::none(),
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![
            iced::widget::button("Add").on_press(Message::Add),
            iced::widget::button("Subtract").on_press(Message::Subtract),
            iced::widget::button("Settings").on_press(Message::OpenSettings),
            iced::widget::text(self.count.to_string()),
        ]
        .into()
    }
    fn close_request(&self) -> bool {
        true
    }
    fn close(&mut self) -> iced::Task<crate::Message> {
        println!("close");
        iced::Task::none()
    }
    fn title(&self) -> String {
        "DTBox".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
