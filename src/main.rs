mod app;
mod skill;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
