use std::collections::BTreeMap;

use iced::{Task, window};

use crate::{Config, MainWindow, Message, SettingsWindow, Window, app::window::WindowHandle};

pub struct State {
    project_dir: directories::ProjectDirs,
    config: Config,
    windows: BTreeMap<window::Id, WindowHandle>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let project_dir = crate::utils::dir::project_dir()
            .unwrap_or_else(|err| panic!("Project Dir Error: {}", err.to_string()));
        let config = Config::load(&project_dir).unwrap_or_default();
        let (_, open) = window::open(MainWindow::settings(&config.window));

        (
            Self {
                project_dir,
                windows: BTreeMap::new(),
                config,
            },
            open.map(Message::OpenMainWindow),
        )
    }
    pub fn windows_close_request(id: iced::window::Id, window: &mut impl Window) -> Task<Message> {
        if window.close_request() {
            window
                .close()
                .chain(window::close(id))
                .chain(Task::done(Message::CloseWindow(id)))
        } else {
            Task::none()
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenMainWindow(id) => {
                let _ = self
                    .windows
                    .insert(id, WindowHandle::MainWindow(MainWindow::new(id)));
                Task::done(Message::MainWindowMessage(
                    id,
                    crate::MainWindowMessage::Open,
                ))
            }
            Message::OpenSettingsWindow => {
                let (id, open) = window::open(SettingsWindow::settings(&self.config.window));
                let _ = self
                    .windows
                    .insert(id, WindowHandle::SettingsWindow(SettingsWindow::new(id)));
                open.map(|id| {
                    Message::SettingsWindowMessage(id, crate::SettingsWindowMessage::Open)
                })
            }
            Message::CloseRequestWindow(id) => {
                if let Some(handle) = self.windows.get_mut(&id) {
                    match handle {
                        WindowHandle::MainWindow(window) => Self::windows_close_request(id, window),
                        WindowHandle::SettingsWindow(window) => {
                            Self::windows_close_request(id, window)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::CloseWindow(id) => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    let _ = self.config.save(&self.project_dir);
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::MainWindowMessage(id, message) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    match window {
                        WindowHandle::MainWindow(window) => {
                            window.update(message, &mut self.config)
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::SettingsWindowMessage(id, message) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    match window {
                        WindowHandle::SettingsWindow(window) => {
                            window.update(message, &mut self.config)
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
        }
    }
    pub fn theme(&self, window: window::Id) -> Option<iced::Theme> {
        if let Some(window) = self.windows.get(&window) {
            match window {
                WindowHandle::MainWindow(window) => Some(window.theme()),
                WindowHandle::SettingsWindow(window) => Some(window.theme()),
            }
        } else {
            None
        }
    }
    pub fn title(&self, window: window::Id) -> String {
        if let Some(window) = self.windows.get(&window) {
            match window {
                WindowHandle::MainWindow(window) => window.title(),
                WindowHandle::SettingsWindow(window) => window.title(),
            }
        } else {
            String::new()
        }
    }
    pub fn view(&self, window: window::Id) -> iced::Element<'_, Message> {
        if let Some(window) = self.windows.get(&window) {
            match window {
                WindowHandle::MainWindow(window) => window
                    .view()
                    .map(|message| Message::MainWindowMessage(window.id, message)),
                WindowHandle::SettingsWindow(window) => window
                    .view()
                    .map(|message| Message::SettingsWindowMessage(window.id, message)),
            }
        } else {
            iced::widget::column![].into()
        }
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        window::close_requests().map(Message::CloseRequestWindow)
    }
}
