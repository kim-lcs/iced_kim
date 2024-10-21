// ! alert a dialog like winform

use std::fmt::Debug;

use iced::widget::{button, column, row, text};
use iced::window::Level;
use iced::{window, Alignment, Element, Length};

use crate::core::{DialogResult, EventMessage, IWindow, Message, Window};

use super::{message_box_button, MessageBoxButton};

/// dialog config
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Data {
    pub title: String,
    pub msg: String,
    pub primary: MessageBoxButton,
    pub secondary: MessageBoxButton,
    pub window_settings: iced::window::Settings,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            title: "确认窗口".into(),
            msg: "确定关闭窗口?".into(),
            primary: message_box_button("确定").dialog_result(DialogResult::Ok),
            secondary: message_box_button("取消").dialog_result(DialogResult::Cancel),
            window_settings: iced::window::Settings {
                min_size: Some(iced::Size::new(400.0, 200.0)),
                size: iced::Size::new(400.0, 200.0),
                position: iced::window::Position::Centered,
                level: Level::AlwaysOnTop,
                resizable: true,
                icon: None,
                ..Default::default()
            },
        }
    }
}

impl IWindow for Data {
    fn new_window(&self) -> Window {
        Window {
            title: "确认窗口".into(),
            data: Box::new(self.to_owned()),
            settings: self.window_settings.clone(),
            ..Default::default()
        }
    }

    fn view(&self, _window: &Window, id: window::Id) -> Element<Message> {
        let button_primary = self
            .primary
            .to_button()
            .on_press(EventMessage::Close(id, self.primary.message.clone()).into())
            .style(button::primary);

        let button_secondary = self
            .secondary
            .to_button()
            .on_press(EventMessage::Close(id, self.secondary.message.clone()).into())
            .style(button::secondary);

        let msg = text(self.msg.to_string()).height(Length::Fill);

        let content = row![msg]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10);

        let button_group = row![]
            .push_maybe(if self.primary.visible {
                Some(button_primary)
            } else {
                None
            })
            .push_maybe(if self.secondary.visible {
                Some(button_secondary)
            } else {
                None
            })
            .align_y(Alignment::Center)
            .spacing(20)
            .padding(10);

        let button_layout = column![button_group]
            .align_x(Alignment::End)
            .width(Length::Fill);

        let group = column![content, button_layout]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill);

        // container(group).width(200).center_x().into()
        group.into()
    }
}
