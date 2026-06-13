use iced_aw::widget::SidebarWithContent;

use crate::SidebarTab;

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    ServerMessage(super::tabs::server::Message),
    LoggingMessage(super::tabs::logging::Message),
    TabSelected(TabId),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TabId {
    Server,
    Logging,
}

pub struct SettingsWindow {
    pub id: iced::window::Id,
    pub active_tab: TabId,
    pub server: super::tabs::server::Server,
    pub logging: super::tabs::logging::Logging,
}

impl SettingsWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self {
            id,
            active_tab: TabId::Server,
            server: super::tabs::server::Server::new(),
            logging: super::tabs::logging::Logging::new(),
        }
    }
}

impl crate::Widget for SettingsWindow {
    type Message = Message;

    fn update(&mut self, message: Self::Message, state: &mut crate::State) -> iced::Task<Message> {
        match message {
            Message::Open => iced::Task::none(),
            Message::TabSelected(tab_id) => {
                self.active_tab = tab_id;
                iced::Task::none()
            }
            Message::ServerMessage(msg) => {
                self.server.update(msg, state).map(Message::ServerMessage)
            }
            Message::LoggingMessage(msg) => {
                self.logging.update(msg, state).map(Message::LoggingMessage)
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = SidebarWithContent::new(Message::TabSelected)
            .push(
                TabId::Server,
                self.server.tab_label(),
                self.server.view().map(Message::ServerMessage),
            )
            .push(
                TabId::Logging,
                self.logging.tab_label(),
                self.logging.view().map(Message::LoggingMessage),
            )
            .sidebar_style(Box::new(iced_aw::style::sidebar::blue))
            .set_active_tab(&self.active_tab);
        iced::widget::container(content).padding(10).into()
    }
}

impl crate::Window for SettingsWindow {
    fn settings(_: &crate::domain::config::WindowConfig) -> iced::window::Settings {
        iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            position: iced::window::Position::Centered,
            exit_on_close_request: false,
            ..Default::default()
        }
    }

    fn close_request(&self) -> bool {
        true
    }
    fn close(&mut self) -> iced::Task<Message> {
        iced::Task::none()
    }
    fn title(&self) -> String {
        "Settings".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
