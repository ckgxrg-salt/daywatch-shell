use relm4::prelude::*;

use dwsh::app::Dwsh;
use dwsh::app::quickcontrol::Status;

fn main() {
    let app = RelmApp::new("io.ckgxrg.dwsh");
    // relm4::set_global_css(include_str!("../assets/style.css"));
    app.run::<Status>(());
}
