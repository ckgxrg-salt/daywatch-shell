use iced_exwlshell::{
    Settings, daemon,
    reexport::Anchor,
    settings::{LayerShellSettings, StartMode},
};

use dwsh_logout::app::Logout;

fn main() -> Result<(), iced_exwlshell::Error> {
    daemon(Logout::new, "dwsh-logout", Logout::update, Logout::view)
        .subscription(Logout::subscription)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((0, 400)),
                exclusive_zone: 400,
                anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
                start_mode: StartMode::AllScreens,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}
