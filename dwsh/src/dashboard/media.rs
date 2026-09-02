//! Media controls from MPRIS.

use gtk4::{glib::Propagation, prelude::*};
use relm4::prelude::*;

use std::time::Duration;
use wayle_media::{core::player::Player, types::PlaybackState};

static PAUSE_ICON_NAME: &str = "media-playback-pause-symbolic";
static PLAY_ICON_NAME: &str = "media-playback-start-symbolic";

pub struct Media {
    players: Vec<Player>,
    cover_art: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    status: PlaybackState,
    position: Duration,
    duration: Option<Duration>,
    can_go_previous: bool,
    can_control: bool,
    can_go_next: bool,
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

            if model.players.len() > 0 {
                gtk::Grid {
                    set_margin_top: 20,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_row_spacing: 10,
                    set_column_spacing: 10,

                    attach[0, 0, 1, 2] = &gtk::Image {
                        #[watch]
                        set_from_file: model.cover_art.as_deref(),
                        set_valign: gtk::Align::Center,
                        set_pixel_size: 120,
                    },
                    attach[1, 0, 1, 1] = &gtk::Label {
                        set_max_width_chars: 20,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        #[watch]
                        set_label: model.title.as_deref().unwrap_or_default(),
                        #[watch]
                        set_tooltip_text: model.title.as_deref(),
                    },
                    attach[1, 1, 1, 1] = &gtk::Label {
                        set_max_width_chars: 20,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        #[watch]
                        set_label: model.artist.as_deref().unwrap_or_default(),
                        #[watch]
                        set_tooltip_text: model.artist.as_deref(),
                    },

                    attach[0, 2, 2, 1] = &gtk::Scale {
                        set_width_request: 610,

                        #[watch]
                        set_value: model.position.as_secs_f64(),
                        #[watch]
                        set_range: (0.0, model.duration.unwrap_or_default().as_secs_f64()),

                        connect_change_value[sender] => move |_, _, value| {
                            sender.input(MediaMsg::Position(Duration::from_secs_f64(value)));
                            Propagation::Proceed
                        }
                    },

                    attach[0, 3, 2, 1] = &gtk::CenterBox {
                        #[wrap(Some)]
                        set_start_widget = &gtk::Label {
                            #[watch]
                            set_visible: model.duration.is_some(),
                            #[watch]
                            set_label: &length_str(model.position),
                        },

                        #[wrap(Some)]
                        set_center_widget = &gtk::Box {
                            gtk::Button {
                                set_height_request: 30,
                                set_icon_name: "media-skip-backward-symbolic",

                                #[watch]
                                set_visible: model.can_go_previous,

                                connect_clicked => MediaMsg::Prev,
                            },
                            gtk::Button {
                                set_height_request: 30,

                                #[watch]
                                set_icon_name: playback_icon_name(model.status),
                                #[watch]
                                set_visible: model.can_control,

                                connect_clicked => MediaMsg::PlayPause,
                            },
                            gtk::Button {
                                set_height_request: 30,
                                set_icon_name: "media-skip-forward-symbolic",

                                #[watch]
                                set_visible: model.can_go_next,

                                connect_clicked => MediaMsg::Next,
                            }
                        },

                        #[wrap(Some)]
                        set_end_widget = &gtk::Label {
                            #[watch]
                            set_visible: model.duration.is_some(),
                            #[watch]
                            set_label: &length_str(model.duration.unwrap_or_default()),
                        },
                    },
                }
            } else {
                gtk::Label {
                    set_size_request: (420, 200),
                    set_label: "No Players Found",
                }
            }
        },
    }

    async fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            players: Vec::new(),
            cover_art: None,
            artist: None,
            title: None,
            status: PlaybackState::Stopped,
            position: Duration::default(),
            duration: None,
            can_go_previous: false,
            can_control: false,
            can_go_next: false,
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

fn length_str(duration: Duration) -> String {
    format!(
        "{:02}:{:02}",
        duration.as_secs_f32() / 60.0,
        duration.as_secs_f32() % 60.0
    )
}

fn playback_icon_name(state: PlaybackState) -> &'static str {
    if state == PlaybackState::Playing {
        PAUSE_ICON_NAME
    } else {
        PLAY_ICON_NAME
    }
}
