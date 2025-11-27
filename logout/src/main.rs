use iced_layershell::build_pattern::{MainSettings, application};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, StartMode};

use dwsh_logout::app::LogoutWindow;

fn main() -> Result<(), iced_layershell::Error> {
    application(
        LogoutWindow::namespace,
        LogoutWindow::update,
        LogoutWindow::view,
    )
    .theme(LogoutWindow::theme)
    .subscription(LogoutWindow::subscription)
    .settings(MainSettings {
        layer_settings: LayerShellSettings {
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,
            start_mode: StartMode::TargetScreen(String::from("eDP-1")),
            ..Default::default()
        },
        ..Default::default()
    })
    .run()
}
