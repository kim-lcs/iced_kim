// ! as a core, you shoud never change it.
// ! you need inherit [IWindow] and [IWindowMessage] for new window

use iced::{window, Element, Theme};
use std::{any::Any, fmt::Debug};

/// ! window base data
#[derive(Debug, Clone)]
pub struct Window {
    /// the window title
    pub title: String,
    /// the iced window settings
    pub settings: window::Settings,
    /// parent window id, for transfer data
    pub parent_id: Option<window::Id>,
    /// child window id, current window will show modal when child id is some
    pub child_id: Option<window::Id>,
    /// user data for current window
    pub data: Box<dyn IWindow>,
    /// the current window data type id, every window data is different
    // TODO 本想做私有字段，但是到其他窗口使用 default的时候报错
    pub data_type_id: std::any::TypeId,
    /// window close callback
    /// * it will create callback automatically when call show_dialog
    pub window_closed_callback: Option<Box<Box<WindowCloseCallback>>>,
}

/// ! a trait for window message, every window message will inherit it
pub trait IWindowMessage: dyn_clone::DynClone + downcast_rs::Downcast {}
dyn_clone::clone_trait_object!(IWindowMessage);
downcast_rs::impl_downcast!(IWindowMessage);

impl std::fmt::Debug for dyn IWindowMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Do not implemented IWindowMessage Debug drive!")
    }
}

/// a trait for new window
pub trait IWindow: dyn_clone::DynClone + downcast_rs::Downcast {
    /// use the window data to create a new window
    fn new_window(&self) -> Window;

    /// get the view of the window
    /// ! do not edit here
    /// # Arguments
    /// * `window` - the current window
    /// * `id` - the current window id
    fn view(&self, window: &Window, id: window::Id) -> Element<Message>;

    /// this is where the window message is handled
    /// # Arguments
    /// * `message` - the global message, you need check the message type
    /// # Example
    /// ```rust, no_run
    ///  if let Some(msg) = message.downcast_ref::<Msg>() {
    ///      match msg {
    ///          Msg::Save => {  }, // execute your code here
    ///          Msg::Cancel => Message::None,
    ///      }
    /// }
    /// ```
    fn update(
        &mut self,
        id: &window::Id,
        message: &std::boxed::Box<dyn IWindowMessage>,
    ) -> Message {
        let _ = message;
        let _ = id;
        Message::None
    }

    /// the window opened callback
    /// # Arguments
    /// * `id` - the id of the window
    /// * `position` - the position of the window
    /// * `size` - the size of the window
    fn on_opened(
        &self,
        id: window::Id,
        position: Option<iced::Point>,
        size: iced::Size,
    ) -> Message {
        let _ = id;
        let _ = position;
        let _ = size;
        Message::None
    }

    /// the window close callback
    /// # Arguments
    /// * `id` - current window id
    /// * `dialog_result` - window close dialog result
    /// # Return
    fn on_window_closed(&self, id: window::Id, dialog_result: DialogResult) -> Message {
        let _ = id;
        let _ = dialog_result;
        Message::None
    }

    /// the window closes for inquiry
    /// # Return
    /// * `Message::None` refuse to close
    /// * `any` close the window
    fn on_close_request(&self, id: window::Id) -> Message {
        EventMessage::Close(id, DialogResult::None).into()
    }

    /// the window get focus
    /// # Arguments
    /// * `id` - the current window id
    fn on_focus(&self, id: window::Id) -> Message {
        let _ = id;
        Message::None
    }

    /// the window lose focus
    /// # Arguments
    /// * `id` - the current window id
    fn on_unfocus(&self, id: window::Id) -> Message {
        let _ = id;
        Message::None
    }

    /// the window moved
    /// # Arguments
    /// * `id` - the current window id
    /// * `x` - the x coordinate
    /// * `y` - the y coordinate
    fn on_moved(&self, id: window::Id, x: f32, y: f32) -> Message {
        let _ = id;
        let _ = x;
        let _ = y;
        Message::None
    }

    /// the window resized
    /// # Arguments
    /// * `id` - the crrent window id
    /// * `width` - the window width
    /// * `height` - the window height
    fn on_resized(&self, id: window::Id, width: f32, height: f32) -> Message {
        let _ = id;
        let _ = width;
        let _ = height;
        Message::None
    }
}
dyn_clone::clone_trait_object!(IWindow);
downcast_rs::impl_downcast!(IWindow);
impl std::fmt::Debug for dyn IWindow {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

/********************the window parameters*********************/

impl Default for Window {
    fn default() -> Self {
        let data = EmptyWindowData::default();
        let type_id = data.type_id();
        Self {
            title: Default::default(),
            settings: window::Settings {
                icon: None,
                ..Default::default()
            },
            parent_id: None,
            child_id: None,
            data: Box::new(data),
            data_type_id: type_id,
            window_closed_callback: None,
        }
    }
}

/// window data
/// * `WindowData::new(parent_id, data)`
#[derive(Debug, Clone)]
// #[allow(missing_debug_implementations)]
pub struct WindowData {
    pub parent_id: Option<window::Id>,
    pub data: Box<dyn IWindow>,
    pub data_type_id: std::any::TypeId,
    pub callback: Option<*mut Box<WindowCloseCallback>>,
}

impl WindowData {
    pub fn new<T>(parent_id: Option<window::Id>, data: T) -> Self
    where
        T: IWindow,
    {
        let type_id = data.type_id();
        WindowData {
            parent_id,
            data: Box::new(data),
            data_type_id: type_id,
            callback: None,
        }
    }
}

/// ! an empty window data structure, use for the default window data
#[derive(Debug, Clone)]
struct EmptyWindowData;

impl Default for EmptyWindowData {
    fn default() -> Self {
        Self {}
    }
}

impl IWindow for EmptyWindowData {
    fn new_window(&self) -> Window {
        todo!()
    }

    fn view(&self, _window: &Window, _id: window::Id) -> Element<Message> {
        todo!()
    }
}

/// ! window close callback
pub type WindowCloseCallback = fn(data: WindowCloseCallbackData) -> Message;
/// the window close callback data
pub struct WindowCloseCallbackData<'a> {
    /// the window id of close window
    pub id: window::Id,
    /// the state of close window
    pub dialog_result: DialogResult,
    /// the current window data
    /// * you can call window.clone().downcast::<Data>()
    /// * or call get_window_data::<T>()
    pub window: &'a Box<dyn IWindow>,
}

impl<'a> WindowCloseCallbackData<'a> {
    /// get the close window data
    /// # Example
    /// ```no_run
    /// r.get_window_data::<Your Data>()
    /// ```
    pub fn get_window_data<T>(&self) -> Option<T>
    where
        T: IWindow,
    {
        let r = self.window.clone().downcast::<T>();
        match r {
            Ok(data) => Some(*data),
            Err(_) => None,
        }
    }
}

/********************Window Message*********************/

/// ! all base Message
#[derive(Debug, Clone)]
pub enum Message {
    /// do nothing
    None,
    /// change window theme
    Theme(Theme),
    /// change window scale
    Scale(f64),
    /// change the window title
    TitleChanged(window::Id, String),
    /// the window event message by iced
    EventMessage(EventMessage),
    /// `user define`
    ///
    /// every window define each message
    ///
    /// you don't need to define all the message together
    /// # Examples
    /// ``` no_run
    /// // define your message in your window
    /// #[derive(Message, Clone)]
    /// enum Msg {
    ///     Save,
    ///     Cancel,
    /// }
    /// // then use Msg
    /// let msg: Message = Msg::Save.into();
    /// // or like this
    /// button("save").on_pressed(Msg::Save.into());
    ///
    /// ```
    WindowMessage(WindowMessage),
    /// `user define` a new window message
    /// * `Message::show_dialog()` - you can use this
    /// * `Message::new_window()`- or use this
    NewWindow(WindowData),
    /// multiple message
    /// * `Message::events()` - you can use this
    /// * `Message::window_messages()` - or use this
    MultMessage(Vec<Message>),
    /// exit the program
    Exit,
}

unsafe impl std::marker::Send for Message {}

// Message functions
impl Message {
    /// create a single window event message
    pub fn event(msg: EventMessage) -> Self {
        msg.into()
    }

    /// create multiple window event messages
    pub fn events(msgs: impl IntoIterator<Item = EventMessage>) -> Self {
        let mut batch = Vec::new();

        for msg in msgs {
            batch.push(msg.into());
        }
        Message::MultMessage(batch)
    }

    /// create a window message
    pub fn window_message<T>(msg: T) -> Self
    where
        T: IWindowMessage,
    {
        Message::WindowMessage(WindowMessage::new(msg))
    }

    /// create multiple window message
    pub fn window_messages<T>(msgs: Vec<T>) -> Self
    where
        T: IWindowMessage,
    {
        let mut batch = Vec::new();

        for msg in msgs {
            batch.push(Message::window_message(msg));
        }
        Message::MultMessage(batch)
    }

    /// a message to create a new window
    /// # Arguments
    /// * `id` - current window id, this will be a parent id for the new window
    /// * `data` - data for new window
    pub fn new_window<T>(id: window::Id, data: T) -> Self
    where
        T: IWindow,
    {
        Message::NewWindow(WindowData::new(Some(id), data))
    }

    /// a message to create a new window with a callback when the window closed
    /// # Arguments
    /// * `id` - current window id, this will be a parent id for the new window
    /// * `data` - data for new window
    /// * `window_close_callback` - a callback that will be called when the window is closed
    pub fn show_dialog<T>(
        id: window::Id,
        data: T,
        window_close_callback: WindowCloseCallback,
    ) -> Self
    where
        T: IWindow,
    {
        let a = Box::new(Box::new(window_close_callback));
        let p = Box::into_raw(a);
        let type_id = data.type_id();
        Message::NewWindow(WindowData {
            parent_id: Some(id),
            data: Box::new(data),
            data_type_id: type_id,
            callback: Some(p),
        })
    }
}

impl From<Vec<Message>> for Message {
    fn from(value: Vec<Message>) -> Self {
        Message::MultMessage(value)
    }
}

impl From<EventMessage> for Message {
    fn from(value: EventMessage) -> Self {
        Message::EventMessage(value)
    }
}

impl From<WindowMessage> for Message {
    fn from(value: WindowMessage) -> Self {
        Message::WindowMessage(value)
    }
}

/// you can convert a window msg to a Message like this : msg.into()
///
/// a trait implement from into need do like this 😓
impl<T: IWindowMessage> From<T> for Message {
    fn from(value: T) -> Self {
        Message::window_message(value)
    }
}

/// window event message
#[derive(Debug, Clone)]
pub enum EventMessage {
    Close(window::Id, DialogResult),
    Opened {
        id: window::Id,
        position: Option<iced::Point>,
        size: iced::Size,
    },
    Closed(window::Id, DialogResult),
    Moved {
        id: window::Id,
        /// The new logical x location of the window
        x: f32,
        /// The new logical y location of the window
        y: f32,
    },
    Resized {
        id: window::Id,
        width: f32,
        height: f32,
    },
    CloseRequest(window::Id),
    Focused(window::Id),
    Unfocused(window::Id),
}

/// dialog result like winform
///
/// of course you can define your own dialog result by DialogResult::Custom
///
/// https://learn.microsoft.com/zh-cn/dotnet/api/system.windows.forms.dialogresult?view=windowsdesktop-8.0
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DialogResult {
    None,
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
    TryAgain,
    Continue,
    Custom(u32),
}

/// the message for window
#[derive(Debug, Clone)]
pub struct WindowMessage {
    pub msg: Box<dyn IWindowMessage>,
    pub msg_type_id: std::any::TypeId,
}

/// simplify production window messages
impl WindowMessage {
    /// create a new window message
    pub fn new<T>(msg: T) -> Self
    where
        T: IWindowMessage,
    {
        let msg_type_id = msg.type_id();
        WindowMessage {
            msg: Box::new(msg),
            msg_type_id,
        }
    }
}
