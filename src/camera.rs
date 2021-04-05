use crate::{obj::Object, player::Player};
use piston_window::*;

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub m_w: f64,
    pub m_h: f64,
}

impl Camera {
    pub fn new(x: f64, y: f64, w: f64, h: f64, m_w: f64, m_h: f64) -> Camera {
        Camera {
            x,
            y,
            w,
            h,
            m_w,
            m_h,
        }
    }

    pub fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g, _d| {
            Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0).draw(
                [self.x, self.y, self.w, self.h],
                &DrawState::default(),
                c.transform,
                g,
            );
        });
    }

    pub fn update(&mut self, player: &mut Player, objects: &mut Vec<Object>) {
        if player.hitbox.0.x <= self.x {
            player.hitbox.0.x = self.x;
            for object in objects.iter_mut() {
                object.rect.x -= player.velocity.x;
            }
        }

        if player.hitbox.0.y <= self.y {
            player.hitbox.0.y = self.y;
            for object in objects.iter_mut() {
                object.rect.y -= player.velocity.y;
            }
        }

        if player.hitbox.0.x + player.hitbox.0.s >= self.x + self.w {
            player.hitbox.0.x = self.x + self.w - player.hitbox.0.s;
            for object in objects.iter_mut() {
                object.rect.x -= player.velocity.x;
            }
        }

        if player.hitbox.0.y + player.hitbox.0.s >= self.y + self.h {
            player.hitbox.0.y = self.y + self.h - player.hitbox.0.s;
            for object in objects.iter_mut() {
                object.rect.y -= player.velocity.y;
            }
        }
    }

    pub fn resize_to_window(&mut self, player: &mut Player) {
        self.h = player.hitbox.0.y/2.0;// - self.y*2.0;
    }
}
