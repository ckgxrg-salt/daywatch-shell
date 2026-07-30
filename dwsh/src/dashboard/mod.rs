//! Dashboard

mod info_bars;

use std::process::Command;

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::prelude::*;

use info_bars::InfoBars;

pub struct Dashboard {
    info_bars: Controller<InfoBars>,
    fortune: String,
    time: String,
}

#[derive(Debug)]
pub enum DashboardMsg {
    RefreshFortune,
    UpdateStatus,
}

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
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Bottom, true),
            set_anchor: (Edge::Right, true),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                append = model.info_bars.widget(),

                gtk::Label {
                    set_wrap: true,
                    #[watch]
                    set_label: &model.fortune,
                    #[watch]
                    set_tooltip_text: Some(&model.fortune),
                },
                gtk::Button {
                    set_tooltip_text: Some("Refresh Quote"),
                    set_icon_name: "dialog-information-symbolic",
                    connect_clicked => DashboardMsg::RefreshFortune,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let info_bars = InfoBars::builder().launch(()).detach();
        let model = Dashboard {
            fortune: get_fortune(),
            info_bars,
            time: String::new(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            DashboardMsg::RefreshFortune => {
                self.fortune = get_fortune();
            }
            DashboardMsg::UpdateStatus => todo!(),
        }
    }
}

fn get_fortune() -> String {
    Command::new("fortune")
        .output()
        .map(|o| {
            String::from_utf8(o.stdout).unwrap_or(String::from("Error parsing quote from fortune"))
        })
        .unwrap_or(String::from("Error fetching quote from fortune"))
}
