use crate::{
    Message, Widget, Window, app::window::WindowHandle, window::MainWindow, window::SettingsWindow,
};
use iced::{Task, window};
use std::collections::BTreeMap;

pub struct Application {
    state: crate::State,
    windows: BTreeMap<window::Id, WindowHandle>,
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        let project_dir = crate::infra::dir::project_dir()
            .unwrap_or_else(|err| panic!("create project dir err:{}", err));
        let config = crate::Config::load(&project_dir).unwrap_or_default();

        crate::infra::logging::setup(
            crate::infra::dir::logging_dir(&project_dir)
                .unwrap_or_else(|err| panic!("create log dir err:{}", err)),
            &config.logging,
        );
        tracing::info!("logging initialized, starting application");

        let state = crate::State::new(project_dir, config)
            .unwrap_or_else(|err| panic!("create state err:{}", err));
        let (_, open) = window::open(MainWindow::settings(&state.config.window));

        (
            Self {
                windows: BTreeMap::new(),
                state,
            },
            open.map(Message::OpenMainWindow),
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = match message {
            Message::OpenMainWindow(id) => {
                let _ = self
                    .windows
                    .insert(id, WindowHandle::MainWindow(MainWindow::new(id)));
                Task::done(Message::MainWindowMessage(
                    id,
                    crate::window::MainWindowMessage::Open,
                ))
            }
            Message::OpenSettingsWindow => {
                let (id, open) = window::open(SettingsWindow::settings(&self.state.config.window));
                let _ = self
                    .windows
                    .insert(id, WindowHandle::SettingsWindow(SettingsWindow::new(id)));
                open.map(|id| {
                    Message::SettingsWindowMessage(id, crate::window::SettingsWindowMessage::Open)
                })
            }
            Message::CloseRequestWindow(id) => self.handle_close_request_window(id),
            Message::CloseWindow(id) => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    let _ = self.state.config_save();
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::MainWindowMessage(id, message) => self.handle_main_window_message(id, message),
            Message::SettingsWindowMessage(id, message) => {
                self.handle_settings_window_message(id, message)
            }
        };

        let pending = std::mem::take(&mut self.state.pending_messages);
        pending
            .into_iter()
            .fold(task, |acc, msg| acc.chain(Task::done(msg)))
    }
    pub fn theme(&self, window: window::Id) -> Option<iced::Theme> {
        if let Some(window) = self.windows.get(&window) {
            Some(window.theme())
        } else {
            None
        }
    }
    pub fn title(&self, window: window::Id) -> String {
        if let Some(window) = self.windows.get(&window) {
            window.title()
        } else {
            String::new()
        }
    }
    pub fn view(&self, window: window::Id) -> iced::Element<'_, Message> {
        self.windows
            .get(&window)
            .map(|w| w.view_and_wrap(window))
            .unwrap_or_else(|| iced::widget::column![].into())
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        window::close_requests().map(Message::CloseRequestWindow)
    }
}

crate::define_window_dispatch! {
    MainWindow => MainWindowMessage(crate::window::MainWindowMessage) as handle_main_window_message,
    SettingsWindow => SettingsWindowMessage(crate::window::SettingsWindowMessage) as handle_settings_window_message,
}
