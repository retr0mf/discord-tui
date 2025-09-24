mod app;

use app::app::App;

fn main() {
    App::default().run(ratatui::init());
    ratatui::restore();
}
