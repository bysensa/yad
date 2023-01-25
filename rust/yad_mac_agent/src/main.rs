mod app;

use cacao::appkit::*;

fn main() {
    let app = App::new("dev.bysensa.yad.agent", app::YadAgentApp::new());
    app.run();
}
