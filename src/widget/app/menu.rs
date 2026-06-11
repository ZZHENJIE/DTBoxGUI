#[derive(Clone, Debug)]
pub enum Message {}

pub struct AppMenu {}

impl crate::Widget for AppMenu {
    type Message = Message;

    fn update(&mut self, _: Self::Message, _: &mut crate::State) -> iced::Task<Self::Message> {
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![].into()
    }
}
