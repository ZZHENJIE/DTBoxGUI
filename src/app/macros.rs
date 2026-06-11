#[macro_export]
macro_rules! define_window_dispatch {
    (
        $(
            $variant:ident => $msg_variant:ident($msg_ty:ty) as $handler:ident
        ),* $(,)?
    ) => {
        impl crate::app::window::WindowHandle {
            pub fn view_and_wrap(&self, id: iced::window::Id) -> iced::Element<'_, crate::Message> {
                match self {
                    $(Self::$variant(w) => {
                        let el: iced::Element<'_, $msg_ty> = w.view();
                        el.map(move |m| crate::Message::$msg_variant(id, m))
                    })*
                }
            }

            pub fn close_and_wrap(&mut self, id: iced::window::Id) -> iced::Task<crate::Message> {
                match self {
                    $(Self::$variant(w) => {
                        let task: iced::Task<$msg_ty> = w.close();
                        task.map(move |m| crate::Message::$msg_variant(id, m))
                    })*
                }
            }
        }

        impl crate::app::application::Application {
            pub fn handle_close_request_window(&mut self, id: iced::window::Id) -> iced::Task<crate::Message> {
                if let Some(handle) = self.windows.get_mut(&id) {
                    match handle {
                        $(
                            crate::app::window::WindowHandle::$variant(w) => {
                                if w.close_request() {
                                    let task: iced::Task<$msg_ty> = w.close();
                                    task
                                        .map(move |m| crate::Message::$msg_variant(id, m))
                                        .chain(iced::window::close(id))
                                        .chain(iced::Task::done(crate::Message::CloseWindow(id)))
                                } else {
                                    iced::Task::none()
                                }
                            }
                        )*
                    }
                } else {
                    iced::Task::none()
                }
            }

            $(
                pub fn $handler(&mut self, id: iced::window::Id, msg: $msg_ty) -> iced::Task<crate::Message> {
                    if let Some(handle) = self.windows.get_mut(&id) {
                        match handle {
                            crate::app::window::WindowHandle::$variant(w) => {
                                let task: iced::Task<$msg_ty> = w.update(msg, &mut self.state);
                                task.map(move |m| crate::Message::$msg_variant(id, m))
                            }
                            _ => iced::Task::none(),
                        }
                    } else {
                        iced::Task::none()
                    }
                }
            )*
        }
    };
}
