// ! main window to display when the application is started.
// ! this will show you how to create a new application, and create a new window by user interaction.

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container},
    Length,
};
use iced_kim::{IWindow, Message};

/// each window has its own [Message]
#[derive(Message, Clone)]
pub enum Msg {
    OpenSubWindow,
}

/// each window has its own data
#[derive(Default, Clone)]
pub struct Data {}

/// you may want use other functions, detail to see [IWindow]
impl IWindow for Data {
    fn new_window(&self) -> iced_kim::Window {
        iced_kim::Window {
            title: "Main Window".to_string(),
            data: Box::new(self.clone()),
            // set the window config for the iced origin window
            settings: iced::window::Settings {
                min_size: Some(iced::Size::new(800.0, 400.0)),
                size: iced::Size::new(1024.0, 768.0),
                position: iced::window::Position::Centered,
                resizable: true,
                exit_on_close_request: false, // if you want to close the window immediately, you should set exit_on_close_request to false.
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn update(
        &mut self,
        id: &iced::window::Id,
        message: &std::boxed::Box<dyn iced_kim::IWindowMessage>,
    ) -> iced_kim::Message {
        // check if the message is matching this window [Msg]
        if let Some(msg) = message.downcast_ref::<Msg>() {
            match msg {
                Msg::OpenSubWindow => {
                    // create a new window, by parent id and new window data
                    Message::new_window(*id, super::set::Data::default())
                }
            }
        } else {
            Message::None
        }
    }

    // when user close this window, you may want to exit the program immediately
    fn on_close_request(&self, _id: iced::window::Id) -> Message {
        Message::Exit
    }

    fn view(
        &self,
        _window: &iced_kim::Window,
        _id: iced::window::Id,
    ) -> iced::Element<iced_kim::Message> {
        // create a button with an event message to open a new window
        let btn1 = button("open sub window").on_press(Msg::OpenSubWindow.into());

        // layout
        container(btn1)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
