use iced::window;

use crate::IntoMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Add,
    Subtract,
    OpenSettings,
    WindowPos(Option<iced::Point>),
    WindowSize(iced::Size),
}

impl IntoMessage for Message {
    fn into_message(&self, id: iced::window::Id) -> crate::Message {
        crate::Message::MainWindowMessage(id, self.clone())
    }
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

impl crate::Window for MainWindow {
    type Message = Message;

    fn settings(settings: &crate::Settings) -> iced::window::Settings {
        let size = iced::Size::new(
            settings.windows.main_window.width,
            settings.windows.main_window.height,
        );
        let position = iced::Point::new(
            settings.windows.main_window.pos_x,
            settings.windows.main_window.pos_y,
        );
        iced::window::Settings {
            size,
            position: iced::window::Position::Specific(position),
            exit_on_close_request: false,
            ..Default::default()
        }
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
            Message::WindowSize(size) => {
                settings.windows.main_window.width = size.width;
                settings.windows.main_window.height = size.height;
                iced::Task::none()
            }
            Message::WindowPos(pos) => {
                if let Some(pos) = pos {
                    settings.windows.main_window.pos_x = pos.x;
                    settings.windows.main_window.pos_y = pos.y;
                }
                iced::Task::none()
            }

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
        let window_id = self.id.clone();
        window::size(window_id.clone())
            .map(Message::WindowSize)
            .chain(window::position(window_id.clone()).map(Message::WindowPos))
            .map(move |message| crate::Message::MainWindowMessage(window_id, message))
    }
    fn title(&self) -> String {
        "DTBox".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
