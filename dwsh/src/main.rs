use std::sync::OnceLock;

use relm4::prelude::*;

use dwsh::{dashboard::Dashboard, services::init_services};
use tokio::runtime::Runtime;

static TOKIO_RT: OnceLock<Runtime> = OnceLock::new();

fn tokio_rt() -> &'static Runtime {
    TOKIO_RT.get_or_init(|| Runtime::new().expect("tokio runtime"))
}

fn main() {
    let app = RelmApp::new("io.ckgxrg.dwsh");

    // You couln't find
    //
    // your CSS.
    // Fix css being overridden by something else
    relm4::set_global_css_with_priority(include_str!("../assets/style.css"), 1225);
    tokio_rt().block_on(async {
        let _ = init_services().await;
    });
    app.run::<Dashboard>(());
}
