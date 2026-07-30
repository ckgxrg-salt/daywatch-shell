//! Status area

use chrono::{DateTime, Local};
use std::process::Command;

use gtk4::ApplicationInhibitFlags;
use gtk4::prelude::*;
use relm4::prelude::*;

pub struct StatusPanel {
    battery_manager: battery::Manager,
    battery: battery::Battery,
    clock: DateTime<Local>,
    volume: f64,
    brightness: f64,
    coffee: bool,
    inhibit_cookie: u32,
    rotation: bool,
    osk: bool,
}

#[derive(Debug)]
pub enum StatusMsg {
    UpdateBattery,
    UpdateClock,
    SetVolume(f64),
    SetBrightness(f64),
    SetCoffee(bool),
    SetRotation(bool),
    SetOSK(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for StatusPanel {
    type Init = (battery::Manager, battery::Battery);
    type Input = StatusMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            // Upper part
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                // Brightness slider
                gtk::Scale::with_range(gtk::Orientation::Vertical, 0.0, 100.0, 10.0) {
                    set_value: model.brightness,
                    connect_value_changed[sender] => move |scale| {
                        sender.input(StatusMsg::SetBrightness(scale.value()));
                    }
                },
                // Volume slider
                gtk::Scale::with_range(gtk::Orientation::Vertical, 0.0, 100.0, 10.0) {
                    set_value: model.volume,
                    connect_value_changed[sender] => move |scale| {
                        sender.input(StatusMsg::SetVolume(scale.value()));
                    }
                },
            },

            // Lower part
            gtk::Grid {
                // Inhibitor
                attach[0, 0, 1, 1] = &gtk::Button {
                    set_tooltip_text: Some("Inhibit System Idle"),
                    set_icon_name: "coffee-symbolic",
                    connect_clicked => StatusMsg::SetCoffee(!&model.coffee),
                },
                // Rotation lock
                attach[1, 0, 1, 1] = &gtk::Button {
                    set_tooltip_text: Some("Toggle Auto Rotation"),
                    set_icon_name: "rotate-symbolic",
                    connect_clicked => StatusMsg::SetRotation(!&model.coffee),
                },
                // OSK
                attach[0, 1, 1, 1] = &gtk::Button {
                    set_tooltip_text: Some("Toggle On-screen Keyboard"),
                    set_icon_name: "keyboard-symbolic",
                    connect_clicked => StatusMsg::SetOSK(!&model.coffee),
                },
                // Dummy
                attach[1, 1, 1, 1] = &gtk::Button {
                    set_tooltip_text: Some("Dummy for now"),
                    set_icon_name: "box-dotted",
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = StatusPanel {
            battery_manager: init.0,
            battery: init.1,
            clock: Local::now(),
            volume: 0.0,
            brightness: 0.0,
            coffee: false,
            inhibit_cookie: 0,
            rotation: false,
            osk: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            StatusMsg::UpdateBattery => {
                if let Err(err) = self.battery_manager.refresh(&mut self.battery) {
                    eprintln!("{err} when refreshing battery");
                }
            }
            StatusMsg::UpdateClock => self.clock = Local::now(),
            StatusMsg::SetVolume(val) => {
                self.volume = val;
                set_volume(self.volume);
            }
            StatusMsg::SetBrightness(val) => {
                self.brightness = val;
                set_brightness(self.brightness);
            }
            StatusMsg::SetCoffee(val) => {
                self.coffee = val;
                let app = relm4::main_application();
                if self.coffee {
                    let cookie = app.inhibit(
                        None::<&gtk::Window>,
                        ApplicationInhibitFlags::IDLE,
                        Some("inhibited by user"),
                    );
                    self.inhibit_cookie = cookie;
                } else {
                    app.uninhibit(self.inhibit_cookie);
                }
            }
            StatusMsg::SetRotation(val) => self.rotation = val,
            StatusMsg::SetOSK(val) => self.osk = val,
        }
    }
}

/// Hardcoded `wpctl` call.
/// `volume` is a percentage from 0 to 100.
fn set_volume(volume: f64) {
    let volume_str = format!("{:.2}", volume / 100.0);
    // Don't care the result
    let _ = Command::new("wpctl")
        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume_str])
        .spawn();
}

/// Hardcoded `brightnessctl` call.
/// `brightness` is a percentage from 0 to 100.
fn set_brightness(brightness: f64) {
    let brightness_str = format!("{:.0}%", brightness);
    // Don't care the result
    let _ = Command::new("brightnessctl")
        .args(["set", &brightness_str])
        .spawn();
}
