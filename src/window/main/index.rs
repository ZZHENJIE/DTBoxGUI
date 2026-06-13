use iced::window;

use crate::view::StartPage;

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    StartPage(crate::view::start::Message),
    WindowPos(Option<iced::Point>),
    WindowSize(iced::Size),
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

impl crate::Widget for MainWindow {
    type Message = Message;
    fn update(&mut self, message: Self::Message, state: &mut crate::State) -> iced::Task<Message> {
        match message {
            Message::Open => self.start_page.check_login_state().map(Message::StartPage),
            Message::StartPage(message) => self
                .start_page
                .update(message, state)
                .map(Message::StartPage),
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
        iced::widget::container(if self.start_page.user_login_state {
            iced::widget::column![].into()
        } else {
            self.start_page.view().map(Message::StartPage)
        })
        .padding(10)
        .into()
    }
}

impl crate::Window for MainWindow {
    fn settings(config: &crate::domain::config::WindowConfig) -> iced::window::Settings {
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
    fn close_request(&self) -> bool {
        true
    }
    fn close(&mut self) -> iced::Task<Message> {
        let window_id = self.id.clone();
        window::size(window_id.clone())
            .map(Message::WindowSize)
            .chain(window::position(window_id.clone()).map(Message::WindowPos))
    }
    fn title(&self) -> String {
        "DTBox".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
