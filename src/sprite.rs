use piston_window::*;
use std::{ops::Index, path::PathBuf};

pub trait SpriteEv {
	fn render(&mut self, e: &Event, w: &mut PistonWindow);
}

#[derive(Clone, Debug)]
pub struct Sprite {
    img_texture: Vec<G2dTexture>,
}

impl Sprite {
    pub fn texture_load(path: PathBuf, w: &mut PistonWindow, flip: Flip) -> G2dTexture {
        Texture::from_path(
            &mut w.create_texture_context(),
            path,
            flip,
            &TextureSettings::new(),
        )
        .expect("Unable to load texture for sprite")
    }

	pub fn new(texture: G2dTexture) -> Sprite {
		Sprite {
			img_texture: vec![texture]
		}
	}

	pub fn add_texture(&mut self, texture: G2dTexture) {
		self.img_texture.push(texture);
	}
}

impl Index<usize> for Sprite {
    type Output = G2dTexture;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.img_texture[idx]
    }
}
