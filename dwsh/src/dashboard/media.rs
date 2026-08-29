//! Media controls from MPRIS.

use gtk4::prelude::*;
use relm4::prelude::*;

use std::time::Duration;
use wayle_media::core::player::Player;

pub struct Media {
    players: Vec<Player>,
    active_player: Option<Player>,
}

#[derive(Debug)]
pub enum MediaMsg {
    PlayPause,
    Next,
    Prev,
    Position(Duration),
    NextPlayer,
    PrevPlayer,
}

#[derive(Debug)]
pub enum MediaCmd {
    UpdatePlayerList(Vec<Player>),
    UpdateActivePlayer(Box<Option<Player>>),
}

#[relm4::component(async, pub)]
impl AsyncComponent for Media {
    type Init = ();
    type Input = MediaMsg;
    type Output = ();
    type CommandOutput = MediaCmd;

    view! {
        gtk::Box {
            set_orientation: gtk4::Orientation::Vertical,

            gtk::CenterBox {
                set_size_request: (630, 60),

                #[wrap(Some)]
                set_start_widget = &gtk::Button {
                    set_tooltip_text: Some("Previous Player"),
                    #[watch]
                    set_visible: model.players.len() > 1,

                    connect_clicked => MediaMsg::PrevPlayer,
                },
                #[wrap(Some)]
                set_center_widget = &gtk::Box {
                    set_spacing: 20,

                    gtk::Image {
                        // TODO:
                        #[watch]
                        set_icon_name: Some("todo"),
                    }
                },
                #[wrap(Some)]
                set_end_widget = &gtk::Button {
                    set_tooltip_text: Some("Next Player"),
                    #[watch]
                    set_visible: model.players.len() > 1,

                    connect_clicked => MediaMsg::NextPlayer,
                },
            },
            gtk::Separator,
        },
    }

    async fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            players: Vec::new(),
            active_player: None,
        };
        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            _ => todo!(),
        }
    }
}
