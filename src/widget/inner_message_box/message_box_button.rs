// ! the message box button settings

use iced::widget::{button, row, text};
use iced::{Alignment, Padding};

use crate::core::{DialogResult, Message};
use crate::widget::CharIcon;

#[derive(Debug, Clone)]
pub struct MessageBoxButton {
    pub visible: bool,
    pub message: DialogResult,
    pub text: String,
    pub icon: Option<char>,
    pub font: Option<iced::Font>,
    pub icon_font: Option<iced::Font>,
    pub size: f32,
}

impl Default for MessageBoxButton {
    fn default() -> Self {
        Self {
            visible: true,
            message: DialogResult::None,
            text: "确定".into(),
            icon: None,
            font: None,
            icon_font: None,
            size: 16.0,
        }
    }
}

impl MessageBoxButton {
    /// new button with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// set button's char icon
    /// * `icon` - char icon
    pub fn icon(mut self, icon: impl Into<char>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn icon_font(mut self, font: iced::Font) -> Self {
        self.icon_font = Some(font);
        self
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// set dialog result when click this button
    /// * `dialog_result` - the dialog result
    pub fn dialog_result(mut self, dialog_result: DialogResult) -> Self {
        self.message = dialog_result;
        self
    }

    pub fn to_button<'a>(&'a self) -> iced::widget::Button<'a, Message> {
        button(
            row![]
                .push_maybe(if let Some(icon) = self.icon {
                    Some(
                        CharIcon::new(icon)
                            .font_maybe(self.icon_font)
                            .icon()
                            .size(self.size),
                    )
                } else {
                    None
                })
                .push(if let Some(font) = self.font {
                    text(&self.text).font(font).size(self.size)
                } else {
                    text(&self.text).size(self.size)
                })
                .spacing(5)
                .align_y(Alignment::Center)
                .padding(Padding::ZERO),
        )
    }
}
