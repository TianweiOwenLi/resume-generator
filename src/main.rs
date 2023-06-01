mod app;

mod dim;
mod hdr;
mod skill;
mod edu;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
