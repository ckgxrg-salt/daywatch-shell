//! Dashboard

mod calendar;
mod quote;
mod stats;

use gtk4::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::prelude::*;

use calendar::Calendar;
use quote::Quote;
use stats::Stats;

pub struct Dashboard {
    quote: Controller<Quote>,
    info_bars: Controller<Stats>,
    calendar: Controller<Calendar>,
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
            set_title: Some("dwsh-dashboard"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                append = model.info_bars.widget(),
                append = model.quote.widget(),
                append = model.calendar.widget(),
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

        let model = Dashboard {
            quote,
            info_bars,
            calendar,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
