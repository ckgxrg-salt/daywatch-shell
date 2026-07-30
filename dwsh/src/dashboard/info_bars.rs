//! Display battery level, cpu and memory usage.

use futures::stream::StreamExt;
use gtk4::prelude::*;
use relm4::prelude::*;

use crate::services::{battery_service, sysinfo_service};

pub struct InfoBars {
    battery: f64,
    cpu: f64,
    mem: f64,
}

#[derive(Debug)]
pub enum InfoBarsMsg {
    BatteryUpdated(f64),
    CpuUpdated(f64),
}

#[relm4::component(pub)]
impl Component for InfoBars {
    type Init = ();
    type Input = InfoBarsMsg;
    type CommandOutput = InfoBarsMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                gtk::Image {
                    set_icon_name: Some("battery-symbolic"),
                },
                gtk::ProgressBar {
                    #[watch]
                    set_fraction: model.battery
                },
            },

            gtk::Box {
                gtk::Image {
                    set_icon_name: Some("cpu-symbolic"),
                },
                gtk::ProgressBar {
                    #[watch]
                    set_fraction: model.cpu
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        watch_battery(&sender);
        watch_sysinfo(&sender);
        let model = InfoBars {
            battery: 0.0,
            cpu: 0.0,
            mem: 0.0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            InfoBarsMsg::BatteryUpdated(percentage) => self.battery = percentage,
            InfoBarsMsg::CpuUpdated(percentage) => self.cpu = percentage,
        }
    }
}

fn watch_battery(sender: &ComponentSender<InfoBars>) {
    let service = battery_service();
    let percentage = service.device.percentage.clone();
    let mut stream = percentage.watch();
    // let state = service.device.state.clone();
    // let is_present = service.device.is_present.clone();

    sender.command(|out, shutdown| async move {
        tokio::select! {
            _ = shutdown.wait() => {},
            _ = async {
                while let Some(value) = stream.next().await {
                    let _ = out.send(InfoBarsMsg::BatteryUpdated(value / 100.0));
                }
            } => {}
        }
    })
}

fn watch_sysinfo(sender: &ComponentSender<InfoBars>) {
    let service = sysinfo_service();
    let mut stream = service.cpu.watch();
    // let state = service.device.state.clone();
    // let is_present = service.device.is_present.clone();

    sender.command(|out, shutdown| async move {
        tokio::select! {
            _ = shutdown.wait() => {},
            _ = async {
                while let Some(value) = stream.next().await {
                    let _ = out.send(InfoBarsMsg::CpuUpdated((value.usage_percent / 100.0).into()));
                }
            } => {}
        }
    })
}
