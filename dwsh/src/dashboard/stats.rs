//! Display battery level, cpu and memory usage.

use gtk4::prelude::*;
use relm4::prelude::*;

use futures::stream::StreamExt;
use wayle_core::watch_all;

use crate::services::{battery_service, sysinfo_service};

pub struct Stats {
    battery: f64,
    cpu: f64,
    mem: f64,
}

#[derive(Debug)]
pub enum StatsCmd {
    UpdateBattery(f64),
    UpdateCpuMem(f64, f64),
}

#[relm4::component(pub)]
impl Component for Stats {
    type Init = ();
    type Input = ();
    type CommandOutput = StatsCmd;
    type Output = ();

    view! {
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
            },

            gtk::Box {
                gtk::Image {
                    set_icon_name: Some("drive-virtual-symbolic"),
                },
                gtk::ProgressBar {
                    #[watch]
                    set_fraction: model.mem
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        watch_battery(&sender);
        watch_sysinfo(&sender);
        let model = Stats {
            battery: 0.0,
            cpu: 0.0,
            mem: 0.0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            StatsCmd::UpdateBattery(percentage) => self.battery = percentage,
            StatsCmd::UpdateCpuMem(cpu, mem) => {
                self.cpu = cpu;
                self.mem = mem;
            }
        }
    }
}

fn watch_battery(sender: &ComponentSender<Stats>) {
    let service = battery_service();
    let percentage = service.device.percentage.clone();
    let mut stream = percentage.watch();
    // let state = service.device.state.clone();
    // let is_present = service.device.is_present.clone();

    sender.command(|out, shutdown| {
        shutdown
            .register(async move {
                while let Some(value) = stream.next().await {
                    let _ = out.send(StatsCmd::UpdateBattery(value / 100.0));
                }
            })
            .drop_on_shutdown()
    });
}

fn watch_sysinfo(sender: &ComponentSender<Stats>) {
    let service = sysinfo_service();
    let mut stream = watch_all!(service, cpu, memory);

    sender.command(|out, shutdown| {
        shutdown
            .register(async move {
                while let Some(value) = stream.next().await {
                    let cpu_val = (value.cpu.get().usage_percent / 100.0) as f64;
                    let mem_val = (value.memory.get().usage_percent / 100.0) as f64;
                    let _ = out.send(StatsCmd::UpdateCpuMem(cpu_val, mem_val));
                }
            })
            .drop_on_shutdown()
    });
}
