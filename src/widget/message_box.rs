// ! the message box
use super::inner_message_box::{MessageBoxButton, MessageBoxData};
use crate::core::{Message, WindowCloseCallback};

/// create a new message box with a new window
/// * `title` - the title of the message box
/// * `content` - the content of the message box
pub fn message_box(title: impl Into<String>, content: impl Into<String>) -> MessageBox {
    MessageBox::new(title, content)
}

pub struct MessageBox {
    data: MessageBoxData,
    callback_closed: Option<WindowCloseCallback>,
}

#[allow(unused)]
impl MessageBox {
    /// create a new dialog with the title and content
    /// * `title` - the title of the dialog
    /// * `content` - the content of the dialog
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            data: MessageBoxData {
                title: title.into(),
                msg: content.into(),
                ..Default::default()
            },
            callback_closed: None,
        }
    }

    /// set the size of the dialog
    pub fn size(mut self, size: iced::Size) -> Self {
        self.data.window_settings.size = size;
        self
    }

    /// set the primary button
    /// * `button` - the primary button
    pub fn primary_button(mut self, button: MessageBoxButton) -> Self {
        self.data.primary = button;
        self
    }

    /// set the secondary button
    /// * `button` - the secondary button
    pub fn secondary_button(mut self, button: MessageBoxButton) -> Self {
        self.data.secondary = button;
        self
    }

    /// hide the primary button,
    pub fn hide_primary_button(mut self) -> Self {
        self.data.primary.visible = false;
        self
    }

    /// hide the secondary button,
    pub fn hide_secondary_button(mut self) -> Self {
        self.data.secondary.visible = false;
        self
    }

    /// set the callback function when the dialog is closed
    pub fn on_closed(mut self, callback: WindowCloseCallback) -> Self {
        self.callback_closed = Some(callback);
        self
    }

    /// show the dialog window
    /// * `id` - current window id, this will be a parent id for the new window
    pub fn show(&self, id: iced::window::Id) -> Message {
        if let Some(callback) = &self.callback_closed {
            Message::show_dialog(id, self.data.clone(), *callback)
        } else {
            Message::show_dialog(id, self.data.clone(), |_| Message::None)
        }
    }
}
