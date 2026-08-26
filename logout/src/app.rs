use iced::{
    ContentFit, Element, Event, Length, Subscription, Task, alignment, event,
    keyboard::{self, Key, key::Named},
    mouse,
    widget::{button, column, container, image, row, svg, text, tooltip},
    window,
};
use iced_exwlshell::to_exwlshell_message;
#[cfg(not(debug_assertions))]
use niri_ipc::{Action, Request};
use std::fmt::Display;

use crate::icon::IconManager;
use crate::{app::Message::SelectAction, icon::XdgIcon};

pub struct Logout {
    icon_manager: IconManager,
    text: String,
    focused: LogoutAction,
}

#[to_exwlshell_message]
#[derive(Debug, Clone)]
pub enum Message {
    SelectAction(LogoutAction),
    IcedEvent(Event),
}

// TODO: review partialeq
#[derive(PartialEq, Debug, Clone)]
pub enum LogoutAction {
    None,
    Poweroff,
    Reboot,
    Suspend,
    Logout,
    Lock,
}

impl Display for LogoutAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "Daywatch"),
            Self::Poweroff => write!(f, "Power off"),
            Self::Reboot => write!(f, "Reboot"),
            Self::Suspend => write!(f, "Suspend"),
            Self::Logout => write!(f, "Log out"),
            Self::Lock => write!(f, "Lock screen"),
        }
    }
}

impl Logout {
    pub fn new() -> Self {
        Self {
            text: LogoutAction::None.to_string(),
            focused: LogoutAction::None,
            icon_manager: IconManager::new(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::IcedEvent)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectAction(action) => {
                if self.focused == action {
                    #[cfg(not(debug_assertions))]
                    execute(&action);
                    #[cfg(debug_assertions)]
                    println!("{action}");
                    return iced::exit();
                } else {
                    self.text = action.to_string();
                    self.focused = action;
                }
            }
            Message::IcedEvent(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    if let Some(action) = identify_key(key) {
                        return Task::done(SelectAction(action));
                    }
                }
                Event::Mouse(iced::mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    return Task::done(SelectAction(LogoutAction::None));
                }
                _ => (),
            },
            _ => (),
        }
        Task::none()
    }

    pub fn view(&self, _id: window::Id) -> Element<'_, Message> {
        std::convert::Into::<Element<'_, Message>>::into(
            column![
                row![
                    tooltip(
                        button(self.find_icon("system-shutdown-symbolic", 12.0))
                            .width(300)
                            .height(300)
                            .on_press(Message::SelectAction(LogoutAction::Poweroff)),
                        "Shutdown",
                        tooltip::Position::Right,
                    ),
                    tooltip(
                        button(self.find_icon("system-reboot-symbolic", 12.0))
                            .width(300)
                            .height(300)
                            .on_press(Message::SelectAction(LogoutAction::Reboot)),
                        "Reboot",
                        tooltip::Position::Right,
                    ),
                ],
                text(&self.text).align_x(alignment::Horizontal::Center),
                row![
                    tooltip(
                        button(self.find_icon("system-suspend-symbolic", 12.0))
                            .width(300)
                            .height(300)
                            .on_press(Message::SelectAction(LogoutAction::Suspend)),
                        "Suspend",
                        tooltip::Position::Right,
                    ),
                    tooltip(
                        button(self.find_icon("system-log-out-symbolic", 12.0))
                            .width(300)
                            .height(300)
                            .on_press(Message::SelectAction(LogoutAction::Logout)),
                        "Logout",
                        tooltip::Position::Right,
                    ),
                    tooltip(
                        button(self.find_icon("system-lock-screen-symbolic", 96.0))
                            .width(300)
                            .height(300)
                            .on_press(Message::SelectAction(LogoutAction::Lock)),
                        "Lock Screen",
                        tooltip::Position::Right,
                    ),
                ]
            ]
            .spacing(50)
            .align_x(alignment::Horizontal::Center),
        )
        .explain(iced::Color::BLACK)
    }

    fn find_icon(&self, name: &str, size: f32) -> Element<'_, Message> {
        let icon = self.icon_manager.lookup(name).unwrap_or_default();
        match icon {
            XdgIcon::Image(handle) => image(handle).height(Length::Fixed(size)).into(),
            XdgIcon::Svg(handle) => container(svg(handle).content_fit(ContentFit::Cover))
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                .into(),
        }
    }

    // view! {
    //     gtk::Window {
    //         init_layer_shell: (),
    //         set_layer: Layer::Overlay,
    //         set_anchor: (Edge::Left, true),
    //         set_anchor: (Edge::Right, true),
    //         set_anchor: (Edge::Top, true),
    //         set_anchor: (Edge::Bottom, true),
    //         set_keyboard_mode: KeyboardMode::Exclusive,
    //         set_title: Some("dwsh-logout"),
    //
    //         add_controller = gtk::GestureClick {
    //             connect_released[sender] => move |_, _, _, _| sender.input(Message::SelectAction(LogoutAction::None)),
    //         },
    //         add_controller = gtk::EventControllerKey {
    //             connect_key_released[sender] => move |_, key, _, _| if let Some(action) = identify_key(key) {
    //                 sender.input(Message::SelectAction(action));
    //             },
    //         },
    //
    //         gtk::Box {
    //             set_orientation: gtk::Orientation::Vertical,
    //             set_align: gtk::Align::Center,
    //             set_spacing: 50,
    //
    //             gtk::Box {
    //                 set_align: gtk::Align::Center,
    //                 set_height_request: 300,
    //                 set_spacing: 50,
    //
    //                 gtk::Button {
    //                     set_tooltip_text: Some("Power Off"),
    //                     set_icon_name: "system-shutdown-symbolic",
    //                     set_size_request: (300, 350),
    //                     #[watch]
    //                     set_class_active: ("focused", model.focused == LogoutAction::Poweroff),
    //                     connect_clicked => Message::SelectAction(LogoutAction::Poweroff),
    //                 },
    //                 gtk::Button {
    //                     set_label: "Reboot",
    //                     set_icon_name: "system-reboot-symbolic",
    //                     set_size_request: (300, 350),
    //                     #[watch]
    //                     set_class_active: ("focused", model.focused == LogoutAction::Reboot),
    //                     connect_clicked => Message::SelectAction(LogoutAction::Reboot),
    //                 }
    //             },
    //
    //             gtk::Box {
    //                 set_orientation: gtk::Orientation::Vertical,
    //                 set_spacing: 10,
    //
    //                 gtk::Label {
    //                     #[watch]
    //                     set_label: &model.text,
    //                 },
    //                 gtk::Separator,
    //             },
    //
    //             gtk::Box {
    //                 set_align: gtk::Align::Center,
    //                 set_height_request: 300,
    //                 set_spacing: 30,
    //
    //                 gtk::Button {
    //                     set_label: "Suspend",
    //                     set_icon_name: "system-suspend-symbolic",
    //                     set_size_request: (300, 350),
    //                     #[watch]
    //                     set_class_active: ("focused", model.focused == LogoutAction::Suspend),
    //                     connect_clicked => Message::SelectAction(LogoutAction::Suspend),
    //                 },
    //                 gtk::Button {
    //                     set_label: "Logout",
    //                     set_icon_name: "system-log-out-symbolic",
    //                     set_size_request: (300, 350),
    //                     #[watch]
    //                     set_class_active: ("focused", model.focused == LogoutAction::Logout),
    //                     connect_clicked => Message::SelectAction(LogoutAction::Logout),
    //                 },
    //                 gtk::Button {
    //                     set_label: "Lock",
    //                     set_icon_name: "system-lock-screen-symbolic",
    //                     set_size_request: (300, 350),
    //                     #[watch]
    //                     set_class_active: ("focused", model.focused == LogoutAction::Lock),
    //                     connect_clicked => Message::SelectAction(LogoutAction::Lock),
    //                 }
    //             }
    //         }
    //     }
    // }
}

impl Default for Logout {
    fn default() -> Self {
        Self::new()
    }
}

fn identify_key(key: Key) -> Option<LogoutAction> {
    match key {
        Key::Named(Named::Escape) => Some(LogoutAction::None),
        Key::Character(char) => match char.as_str() {
            "c" => Some(LogoutAction::Poweroff),
            "r" => Some(LogoutAction::Reboot),
            "l" => Some(LogoutAction::Lock),
            "e" => Some(LogoutAction::Logout),
            "u" => Some(LogoutAction::Suspend),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(not(debug_assertions))]
// Executes the given logout action.
fn execute(action: &LogoutAction) {
    let Ok(mut socket) = niri_ipc::socket::Socket::connect() else {
        // TODO: Log
        return;
    };
    match *action {
        LogoutAction::Poweroff => {
            let _ = socket.send(Request::Action(Action::Spawn {
                command: vec![String::from("systemctl"), String::from("poweroff")],
            }));
        }
        LogoutAction::Reboot => {
            let _ = socket.send(Request::Action(Action::Spawn {
                command: vec![String::from("systemctl"), String::from("reboot")],
            }));
        }
        LogoutAction::Logout => {
            let _ = socket.send(Request::Action(Action::Quit {
                skip_confirmation: false,
            }));
        }
        LogoutAction::Lock => {
            let _ = socket.send(Request::Action(Action::Spawn {
                command: vec![String::from("hyprlock")],
            }));
        }
        LogoutAction::Suspend => {
            let _ = socket.send(Request::Action(Action::Spawn {
                command: vec![String::from("systemctl"), String::from("suspend")],
            }));
        }
        LogoutAction::None => (),
    }
}
