mod core;
mod widget;

#[cfg(feature = "multi-windows")]
#[doc(no_inline)]
pub use crate::widget::multi_windows::{run, Program};
#[cfg(feature = "multi-windows")]
#[doc(no_inline)]
pub use core::{DialogResult, EventMessage, IWindow, IWindowMessage, Message, Window};
#[cfg(feature = "multi-windows")]
pub use iced_kim_macro::Message;

#[cfg(feature = "message-box")]
#[doc(no_inline)]
pub use crate::widget::message_box::{message_box, MessageBox};
#[cfg(feature = "message-box")]
#[doc(no_inline)]
pub use crate::widget::{message_box_button, MessageBoxButton};

#[cfg(feature = "table")]
#[doc(no_inline)]
pub use crate::widget::table::{
    table, table_head, table_index, Table, TableHead, TableHeadType, TableRow,
};
#[cfg(feature = "table")]
#[doc(no_inline)]
pub use iced_kim_macro::TableRow;
