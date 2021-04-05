use crate::{
    sprite::{Sprite, SpriteEv},
    utils::Rect,
};
use piston_window::*;

#[derive(Clone, Debug)]
pub struct Object {
	pub sprite: Sprite,
	pub solid: bool,
	pub rect: Rect,
}

impl Object {
    pub fn new(sprite: Sprite, rect: Rect, solid: bool) -> Object {
        Object { sprite, rect, solid }
    }
}

impl SpriteEv for Object {
    fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        let texture = &self.sprite[0];
        let rect = &self.rect;

        w.draw_2d(e, |c,g,_d| {
			image(
				texture, 
				c.trans(rect.x, rect.y)
				.scale(rect.s / texture.get_width() as f64, rect.s / texture.get_height() as f64)
				.transform, 
                g
			);
		});
    }
}
