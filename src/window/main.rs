use iced::window;

use crate::{IntoMessage, view::StartPage};

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    StartPage(crate::view::start_page::Message),
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
    pub start_page: StartPage,
}

impl MainWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self {
            id,
            start_page: StartPage::new(),
        }
    }
}

impl crate::Window for MainWindow {
    type Message = Message;
    fn settings(config: &crate::app::config::WindowConfig) -> iced::window::Settings {
        let size = iced::Size::new(config.main_window.width, config.main_window.height);
        let position: iced::window::Position = {
            if config.main_window.pos_x.is_none() || config.main_window.pos_y.is_none() {
                iced::window::Position::Centered
            } else {
                iced::window::Position::Specific(iced::Point::new(
                    config.main_window.pos_x.unwrap_or_default(),
                    config.main_window.pos_y.unwrap_or_default(),
                ))
            }
        };
        iced::window::Settings {
            size,
            position,
            exit_on_close_request: false,
            ..Default::default()
        }
    }
    fn update(
        &mut self,
        message: Self::Message,
        state: &mut crate::State,
    ) -> iced::Task<crate::Message> {
        let id = self.id.clone();
        match message {
            Message::Open => self
                .start_page
                .check_login_state()
                .map(move |message| Message::StartPage(message).into_message(id)),
            Message::StartPage(message) => self
                .start_page
                .update(message, state)
                .map(move |message| Message::StartPage(message).into_message(id)),
            Message::WindowSize(size) => {
                state.config.window.main_window.width = size.width;
                state.config.window.main_window.height = size.height;
                iced::Task::none()
            }
            Message::WindowPos(pos) => {
                if let Some(pos) = pos {
                    state.config.window.main_window.pos_x = Some(pos.x);
                    state.config.window.main_window.pos_y = Some(pos.y);
                }
                iced::Task::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        if self.start_page.user_login_state {
            iced::widget::column![].into()
        } else {
            self.start_page
                .view()
                .map(|message| Message::StartPage(message))
        }
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
