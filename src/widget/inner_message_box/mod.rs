mod message_box_button;
mod message_box_window;
pub use message_box_button::MessageBoxButton;
pub use message_box_window::Data as MessageBoxData;

pub fn message_box_button(text: impl Into<String>) -> MessageBoxButton {
    MessageBoxButton::new(text)
}
