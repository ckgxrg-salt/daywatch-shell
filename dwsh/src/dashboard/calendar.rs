//! Display time and calendar

use gtk4::prelude::*;
use relm4::prelude::*;

use std::time::Duration;
// TODO: `chrono` since wayle uses it?
use time::{OffsetDateTime, macros::format_description};

pub struct Calendar {
    time: OffsetDateTime,
}

#[derive(Debug)]
pub enum CalendarCmd {
    UpdateTime(OffsetDateTime),
}

#[relm4::component(pub)]
impl Component for Calendar {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = CalendarCmd;

    view! {
        gtk::Box {
            gtk::Label {
                #[watch]
                set_label: &model.time.format(format_description!("[hour]")).unwrap_or_default()
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    #[watch]
                    set_label: &model.time.format(format_description!("[weekday repr:short]")).unwrap_or_default()
                },
                gtk::Label {
                    #[watch]
                    set_label: &model.time.format(format_description!("[month]/[day]")).unwrap_or_default()
                },
            },
            gtk::Label {
                #[watch]
                set_label: &model.time.format(format_description!("[minute]")).unwrap_or_default()
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        watch_time(&sender);
        let model = Calendar {
            time: OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()),
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
            CalendarCmd::UpdateTime(now) => self.time = now,
        }
    }
}

fn watch_time(sender: &ComponentSender<Calendar>) {
    sender.command(|out, shutdown| {
        shutdown
            .register(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
                    let _ = out.send(CalendarCmd::UpdateTime(now));
                }
            })
            .drop_on_shutdown()
    });
}
