//! Main dashboard interface

use gtk4::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::prelude::*;

mod calendar;
mod media;
mod quote;
mod stats;
mod switches;
mod systray;

use calendar::Calendar;
use quote::Quote;
use stats::Stats;
use switches::Switches;
use systray::Systray;

pub struct Dashboard {
    quote: AsyncController<Quote>,
    info_bars: AsyncController<Stats>,
    calendar: Controller<Calendar>,
    switches: AsyncController<Switches>,
    systray: AsyncController<Systray>,
}

#[derive(Debug)]
pub enum DashboardMsg {}

#[relm4::component(pub)]
impl SimpleComponent for Dashboard {
    type Init = ();
    type Input = DashboardMsg;
    type Output = ();

    view! {
        gtk::Window {
            init_layer_shell: (),
            set_layer: Layer::Bottom,
            set_keyboard_mode: gtk4_layer_shell::KeyboardMode::OnDemand,
            set_title: Some("dwsh-dashboard"),

            gtk::Grid {
                attach[0, 0, 4, 6] = model.info_bars.widget(),
                attach[4, 0, 6, 6] = model.calendar.widget(),
                attach[10, 0, 4, 6] = model.switches.widget(),
                attach[0, 6, 14, 4] = model.quote.widget(),
                attach[0, 10, 7, 6] = &gtk::Box,
                attach[7, 10, 7, 6] = model.systray.widget(),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let quote = Quote::builder().launch(()).detach();
        let info_bars = Stats::builder().launch(()).detach();
        let calendar = Calendar::builder().launch(()).detach();
        let switches = Switches::builder().launch(()).detach();
        let systray = Systray::builder().launch(()).detach();

        let model = Dashboard {
            quote,
            info_bars,
            calendar,
            switches,
            systray,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
