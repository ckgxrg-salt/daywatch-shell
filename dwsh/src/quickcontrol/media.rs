//! Media controls via MPRIS

use mpris::{Player, PlayerFinder};
use std::time::Duration;

use gtk4::prelude::*;
use relm4::prelude::*;

struct MediaPanel {
    player_finder: PlayerFinder,
    current_player: Option<Player>,
    track: Option<TrackInfo>,
}

struct TrackInfo {
    cover_art: String,
    title: String,
    artist: String,
}

struct PlaybackInfo {
    current: Duration,
    length: Duration,
}

#[derive(Debug)]
enum MediaMsg {}

#[relm4::component]
impl SimpleComponent for MediaPanel {
    type Init = ();
    type Input = MediaMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            // Cover art
            gtk::Image {
                #[watch]
                set_from_file: model.track.as_ref().map(|i| &i.cover_art),
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                // Title label
                gtk::Label {
                    #[watch]
                    set_label: model.track.as_ref().map(|i| i.title.as_str()).unwrap_or_default(),
                },
                // Artist label
                gtk::Label {
                    #[watch]
                    set_label: model.track.as_ref().map(|i| i.artist.as_str()).unwrap_or_default(),
                },
                // Progress bar
                gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 5.0) {

                },
                // Control buttons
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    // Current position
                    gtk::Box {

                    }
                    // Previous track
                    // Play / Pause
                    // Next track
                    // Length
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // TODO: Will panic here, too lazy to do error handling for now
        let player_finder = PlayerFinder::new().expect("Cannot connect to MPRIS");
        let current_player = player_finder.find_active().ok();
        let model = MediaPanel {
            player_finder,
            current_player,
            track: None,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
