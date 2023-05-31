mod app;

mod hdr;
mod skill;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
