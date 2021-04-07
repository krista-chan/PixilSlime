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
    pub kind: ObjectKind
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectKind {
    GROUND,
    // BACKGROUND,
    SPIKE
}

impl Object {
    pub fn new(sprite: Sprite, rect: Rect, solid: bool, kind: ObjectKind) -> Object {
        Object { sprite, rect, solid, kind }
    }
}

impl SpriteEv for Object {
    fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        let texture = &self.sprite[0];
        let rect = &self.rect;

        w.draw_2d(e, |c,g,_| {
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
