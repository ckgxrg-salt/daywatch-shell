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
    relm4::set_global_css(include_str!("../assets/style.css"));
    tokio_rt().block_on(async {
        let _ = init_services().await;
    });
    app.run::<Dashboard>(());
}
