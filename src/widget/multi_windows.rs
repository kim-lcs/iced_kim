// ! multiple windows

use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::{column, container, opaque, stack};
use iced::{event, window, Color, Element, Length, Subscription, Task, Theme};
use std::collections::BTreeMap;

use crate::core::WindowCloseCallbackData;
use crate::core::{DialogResult, EventMessage, IWindow, Message, Window};

/// run multiple window with a main window data
pub fn run<W>(data: W, default_font: iced::Font) -> iced::Result
where
    W: IWindow,
{
    // TODO: you should install font before starting
    iced::daemon(Program::title, Program::update, Program::view)
        .subscription(Program::subscription)
        .theme(Program::theme)
        .scale_factor(Program::scale_factor)
        .settings(iced::Settings {
            default_font, //iced::Font::with_name("微软雅黑"),
            ..Default::default()
        })
        .run_with(move || Program::new(data))
}

/// multiple windows data
pub struct Program {
    windows: BTreeMap<window::Id, Window>,
    theme: Theme,
    scale: f64,
    icon: Option<iced::window::Icon>,
}

/// execute the multiple windows program
#[allow(unused_variables)]
impl Program {
    /// open the main window
    pub fn new<W>(data: W) -> (Self, Task<Message>)
    where
        W: IWindow,
    {
        let new_window = data.new_window();
        let icon = new_window.settings.icon.clone();
        let (id, open) = window::open(new_window.settings.clone());
        (
            Self {
                windows: BTreeMap::from([(id, new_window)]),
                theme: iced::Theme::CatppuccinLatte,
                scale: 1.0,
                icon: icon,
            },
            open.map(|id| Message::None), // !  must do this step, otherwise the window won't open.
        )
    }

    /// you don't need care
    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or("".to_string())
    }

    /// 数据交互的核心部分，因为只有此处有 mut ,所以修改参数全部通过此处转发
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // * you don't need care
            Message::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }
                Task::none()
            }
            Message::EventMessage(msg) => {
                let mut tasks = vec![];
                match msg {
                    EventMessage::Close(id, dialog_result) => {
                        // 从当前窗口找到父窗口相关信息
                        let (parent_id, call_back) = if let Some(window) = self.windows.get(&id) {
                            (window.parent_id, window.window_closed_callback.clone())
                        } else {
                            (None, None)
                        };
                        // 执行子窗口窗口关闭前，父窗口回调
                        if let Some(parent_id) = parent_id {
                            if let Some(parent) = self.windows.get_mut(&parent_id) {
                                // 检查回调
                                if let Some(callback) = &call_back {
                                    let data = WindowCloseCallbackData {
                                        id: parent_id,
                                        dialog_result: dialog_result.clone(),
                                        window: &parent.data,
                                    };
                                    let m1 = (*callback)(data);
                                    tasks.push(self.update(m1));
                                }
                            }
                        }
                        // 关闭子窗口
                        let message = EventMessage::Closed(id, dialog_result).into();
                        let cmd = self.update(message);
                        tasks.push(cmd);
                        tasks.push(window::close(id));
                    }
                    EventMessage::Opened { id, position, size } => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_opened(id, position, size);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                    EventMessage::Closed(id, dialog_result) => {
                        let parent_id = {
                            let window = self.windows.get(&id);
                            if let Some(window) = window {
                                window.parent_id
                            } else {
                                None
                            }
                        };
                        // ! 2024-09-24 Kim: 移除父窗口的子窗口id
                        if let Some(parent_id) = parent_id {
                            if let Some(parent) = self.windows.get_mut(&parent_id) {
                                parent.child_id = None;
                            }
                        }
                        // 移除缓存
                        let window = self.windows.get(&id);
                        if let Some(window) = window {
                            let m1 = window.data.on_window_closed(id, dialog_result.to_owned());
                            let cmd1 = self.update(m1);
                            self.windows.remove(&id);
                            tasks.push(cmd1);
                            if self.windows.len() == 0 {
                                return iced::exit();
                            }
                        } else if self.windows.len() == 0 {
                            return iced::exit();
                        }
                    }
                    EventMessage::Moved { id, x, y } => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_moved(id, x, y);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                    EventMessage::Resized { id, width, height } => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_resized(id, width, height);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                    EventMessage::CloseRequest(id) => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_close_request(id);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                    EventMessage::Focused(id) => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_focus(id);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                    EventMessage::Unfocused(id) => match self.windows.get(&id) {
                        Some(window) => {
                            let message = window.data.on_unfocus(id);
                            let cmd = self.update(message);
                            tasks.push(cmd);
                        }
                        None => {}
                    },
                }

                Task::batch(tasks)
            }
            // ! 自定义：窗口新建，此处限制了所有窗口当前仅可以打开一次
            Message::NewWindow(window_data) => {
                let data_type_id = window_data.data_type_id;
                let next = self
                    .windows
                    .iter()
                    .filter(|a| a.1.data_type_id == data_type_id)
                    .next();
                if next.is_none() {
                    let parent_id = window_data.parent_id;
                    let data = window_data.data;
                    let mut new_window = data.new_window();
                    // * if the new window doesn't have an icon, then use the main window icon instead.
                    if new_window.settings.icon.is_none() {
                        new_window.settings.icon = self.icon.clone();
                    }
                    new_window.parent_id = parent_id;
                    new_window.data_type_id = data_type_id;
                    // check whether need callback when the window is destroyed
                    if let Some(p) = window_data.callback {
                        let callback = unsafe { Box::from_raw(p) };
                        new_window.window_closed_callback = Some(callback);
                        // TODO disable the parent window when show a alert dialog
                        // if let Some(parent) = self.windows.get_mut(&parent_id) {
                        //     // parent.settings.resizable = false;
                        // }
                    }
                    // open a new window
                    let (id, open) = window::open(new_window.settings.clone());
                    self.windows.insert(id, new_window);
                    // add child id to the parent window
                    if let Some(parent_id) = parent_id {
                        if let Some(parent) = self.windows.get_mut(&parent_id) {
                            parent.child_id = Some(id);
                        }
                    }
                    // ! to show the window
                    open.map(|id| Message::None)
                } else {
                    Task::none()
                }
            }
            // ! 2024-03-29 Kim 在所有页面遍历传递消息，这样发送的时候就不用管ID了，只要发送对应数据就可以了，页面需要什么数据就监控什么数据。
            Message::WindowMessage(msg) => {
                let mut msgs = Vec::new();
                for (id, window) in self.windows.iter_mut() {
                    let msg = window.data.update(&id.clone(), &msg.msg);
                    match msg {
                        Message::None => {}
                        _ => {
                            msgs.push(msg);
                        }
                    }
                }
                self.update(msgs.into())
            }
            Message::MultMessage(msgs) => {
                let mut tasks = vec![];
                for msg in msgs {
                    tasks.push(self.update(msg));
                }
                Task::batch(tasks)
            }
            Message::Theme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::Scale(scale) => {
                self.scale = scale;
                Task::none()
            }
            Message::Exit => iced::exit(),
            Message::None => Task::none(),
        }
    }

    /// you don't need care
    pub fn view(&self, id: window::Id) -> Element<Message> {
        if let Some(window) = self.windows.get(&id) {
            let content = window.data.view(window, id);
            if let Some(child_id) = window.child_id {
                // ! 2024-09-24增加窗口模态
                stack![
                    content,
                    opaque(
                        container("")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(|theme| {
                                container::Style {
                                    background: Some(
                                        Color {
                                            a: 0.8,
                                            ..Color::BLACK
                                        }
                                        .into(),
                                    ),
                                    ..container::Style::default()
                                }
                            })
                    )
                ]
                .into()
            } else {
                content
            }
        } else {
            column!().into()
        }
    }

    /// 统一修改主题
    pub fn theme(&self, _window: window::Id) -> Theme {
        self.theme.clone()
    }

    /// 窗口统一缩放
    pub fn scale_factor(&self, _window: window::Id) -> f64 {
        self.scale.clone()
    }

    /// 订阅窗口事件，不用关注
    /// todo 输入法切换
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, status, id| {
            if let iced::Event::Window(window_event) = event {
                match window_event {
                    window::Event::Opened { position, size } => {
                        let msg = EventMessage::Opened { id, position, size };
                        Some(msg.into())
                    }
                    // ! 需要窗口设置参数 exit_on_close_request = false 才会触发
                    window::Event::CloseRequested => {
                        let msg = EventMessage::CloseRequest(id);
                        Some(msg.into())
                    }
                    window::Event::Closed => {
                        let msg = EventMessage::Closed(id, DialogResult::None);
                        Some(msg.into())
                    }
                    window::Event::Focused => {
                        let msg = EventMessage::Focused(id);
                        Some(msg.into())
                    }
                    window::Event::Unfocused => {
                        let msg = EventMessage::Unfocused(id);
                        Some(msg.into())
                    }
                    window::Event::Moved(point) => {
                        let msg = EventMessage::Moved {
                            id,
                            x: point.x,
                            y: point.y,
                        };
                        Some(msg.into())
                    }
                    window::Event::Resized(size) => {
                        let msg = EventMessage::Resized {
                            id,
                            width: size.width,
                            height: size.height,
                        };
                        Some(msg.into())
                    }
                    _ => None,
                }
            } else {
                match event {
                    iced::Event::Keyboard(ke) => match ke {
                        iced::keyboard::Event::KeyPressed {
                            key,
                            modified_key,
                            physical_key,
                            location,
                            modifiers,
                            text,
                        } => {
                            if key == Key::Named(Named::Shift) {
                                // 处理粘贴事件
                                if let Some(clipboard) = text {
                                    println!("clipboad -> {}", clipboard);
                                }
                                // WindowEvent::Ime(ime) => match ime {
                                //     winit::event::Ime::Enabled => {
                                //         Some(Event::Keyboard(keyboard::Event::IMEEnabled))
                                //     }
                                //     winit::event::Ime::Preedit(text, range) => {
                                //         // range parameter is used to mark converting position.

                                //         Some(Event::Keyboard(keyboard::Event::IMEPreedit(
                                //             text.clone(),
                                //             *range,
                                //         )))
                                //     }
                                //     winit::event::Ime::Commit(text) => {
                                //         Some(Event::Keyboard(keyboard::Event::IMECommit(text.clone())))
                                //     }
                                //     winit::event::Ime::Disabled => None,
                                // },
                            }
                            None
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
        })
    }
}
