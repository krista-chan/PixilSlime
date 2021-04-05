use find_folder;
use fps_counter::FPSCounter;
use piston_window::*;

mod scene;
mod player;
mod sprite;
mod utils;
mod obj;
mod collision;
mod camera;
use scene::Scene;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("PixilSlime", (800, 600))
        .build()
        .expect("Unable to open window. Exiting...");
    window.set_ups(60);

    let assets = find_folder::Search::Kids(1).for_folder("assets").unwrap();

    let mut glyphs = window
        .load_font(assets.join("Comfortaa-Regular.ttf"))
        .expect("ASSET ERROR: Unable to load font");
    let mut fps_counter = FPSCounter::new();

    let mut scene = Scene::new(assets, "1.txt".to_string(), &mut window);

    while let Some(e) = window.next() {
        let fps = format!("{} fps", fps_counter.tick().to_string());
        scene.update(&e, &mut window);

        window.draw_2d(&e, |c, g, d| {
            let transform = c.transform.trans(10.0, 30.0);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 30)
                .draw(
                    &fps, 
                    &mut glyphs, 
                    &c.draw_state, 
                    transform, 
                    g)
                .unwrap();

            glyphs.factory.encoder.flush(d);
        });
    }
}
