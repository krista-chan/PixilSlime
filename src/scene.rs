use crate::{
    camera::Camera,
    obj::Object,
    player::Player,
    sprite::{Sprite, SpriteEv},
    utils::{self, Rect},
};
use piston_window::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Scene {
    pub player: Player,
    objects: Vec<Object>,
    camera: Camera,
}

impl Scene {
    pub fn new(assets: PathBuf, level_name: String, w: &mut PistonWindow) -> Scene {
        let (objects, m_w, m_h) = utils::parse_level_tiles(assets.clone(), level_name.as_str(), w);
        // let (width, height) = (w.size().width, w.size().height);

        let sprite = Sprite::texture_load(assets.join("placeholder-tile.png"), w, Flip::None);
        let sprite_l =
            Sprite::texture_load(assets.join("placeholder-tile.png"), w, Flip::Horizontal);

        let mut player_sprite = Sprite::new(sprite);
        player_sprite.add_texture(sprite_l);

        let player_rect = Rect::new(0.0, 0.0, 5.0, 0.0, 40.0);
        let player = Player::new(player_sprite, player_rect);

        let cam_x = 800.0 / 2.0 - 50.0;
        let cam_y = 600.0 / 2.0 - 50.0;

        let camera = Camera::new(cam_x, cam_y, 100.0, 100.0, m_w, m_h);

        Scene {
            player,
            objects,
            camera,
        }
    }

    pub fn update(&mut self, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |_, g, _| {
            clear(color::hex("aaeeffff"), g);
        });

		let width = w.size().width;

        for object in self.objects.iter_mut().filter(|o| {
            o.rect.x.round() >= -40.0
                && o.rect.x.round() <= width
                && o.rect.y.round() >= -40.0
        }) {
            object.render(e, w);
        }

        self.player.render(e, w);
        self.player.handle_keypress(e);

        self.camera.resize_to_window(&mut self.player);
        self.camera.render(e, w);

        if let Some(u) = e.update_args() {
            self.player.update(u.dt, &mut self.objects);
            self.camera.update(&mut self.player, &mut self.objects);
        }
    }
}
