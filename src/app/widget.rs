use std::fmt::Debug;

pub trait Widget {
    type Message: Clone + Debug;

    fn update(
        &mut self,
        message: Self::Message,
        state: &mut crate::State,
    ) -> iced::Task<Self::Message>;
    fn view(&self) -> iced::Element<'_, Self::Message>;
}

pub trait SidebarTab: crate::Widget {
    fn tab_label(&self) -> iced_aw::sidebar::TabLabel;
}
