#[derive(Clone, Debug)]
pub enum Message {}

pub struct Logging {}

impl Logging {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Widget for Logging {
    type Message = Message;
    fn update(&mut self, _: Self::Message, _: &mut crate::State) -> iced::Task<Self::Message> {
        iced::Task::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![iced::widget::text("Logging")].into()
    }
}

impl crate::SidebarTab for Logging {
    fn tab_label(&self) -> iced_aw::sidebar::TabLabel {
        iced_aw::sidebar::TabLabel::Text("Logging".to_string())
    }
}
