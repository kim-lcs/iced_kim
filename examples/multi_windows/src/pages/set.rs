// ! this is a sub window.
// ! you may want to setting something by a new window.

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, text},
    Length,
};
use iced_kim::{IWindow, Message};

/// each window has its own [Message]
#[derive(Message, Clone)]
pub enum Msg {
    AddOne,
}

/// each window has its own data
#[derive(Default, Clone)]
pub struct Data {
    value: i32,
}

/// you may want use other functions, detail to see [IWindow]
impl IWindow for Data {
    fn new_window(&self) -> iced_kim::Window {
        iced_kim::Window {
            title: "Set Window".to_string(),
            data: Box::new(self.clone()),
            settings: iced::window::Settings {
                size: iced::Size::new(400.0, 300.0),
                position: iced::window::Position::Centered,
                resizable: false,
                level: iced::window::Level::AlwaysOnTop,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn update(
        &mut self,
        _id: &iced::window::Id,
        message: &std::boxed::Box<dyn iced_kim::IWindowMessage>,
    ) -> Message {
        // check if the message is matching this window [Msg]
        if let Some(msg) = message.downcast_ref::<Msg>() {
            match msg {
                Msg::AddOne => {
                    self.value += 1;
                }
            }
        }
        Message::None
    }

    fn view(
        &self,
        _window: &iced_kim::Window,
        _id: iced::window::Id,
    ) -> iced::Element<iced_kim::Message> {
        // create a new button and text element, when the button is clicked, the value is updated by plus one
        let group = column![
            button("Add One").on_press(Msg::AddOne.into()),
            text(self.value.to_string()).size(24)
        ]
        .spacing(20)
        .align_x(Horizontal::Center);

        // layout
        container(group)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
