//! Displays a quote from `fortune`.

use gtk4::prelude::*;
use relm4::prelude::*;
use std::process::Command;

pub struct Quote {
    text: String,
}

#[derive(Debug)]
pub enum QuoteMsg {
    RefreshQuote,
}

#[relm4::component(pub)]
impl SimpleComponent for Quote {
    type Init = ();
    type Input = QuoteMsg;
    type Output = ();

    view! {
        gtk::Box {
            gtk::Button {
                set_tooltip_text: Some("Refresh quote"),
                set_icon_name: "messenger-indicator",
                connect_clicked => QuoteMsg::RefreshQuote
            },
            gtk::Label {
                set_wrap: true,

                #[watch]
                set_label: &model.text,
                #[watch]
                set_tooltip_text: Some(&model.text)
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let model = Self {
            text: get_fortune(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            QuoteMsg::RefreshQuote => self.text = get_fortune(),
        }
    }
}

// TODO: Consider make this async?
fn get_fortune() -> String {
    Command::new("fortune")
        .output()
        .map(|o| {
            String::from_utf8(o.stdout).unwrap_or(String::from("Error parsing quote from fortune"))
        })
        .unwrap_or(String::from("Error fetching quote from fortune"))
}
