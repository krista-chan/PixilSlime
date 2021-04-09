use crate::{
    collision::{Collider, Event},
    obj::{Object, ObjectKind},
    sprite::{Sprite, SpriteEv},
    utils::{PlayerController, PowerupState, Rect, Vector2},
};

use piston_window::{Event as WindowEvent, *};

#[derive(Clone, Debug)]
pub struct Player {
    pub flip: bool,
    pub grounded: bool,
    pub powerup: PowerupState,
    pub sprite: Sprite,
    pub hitbox: (Rect, Collider),
    pub accel: Vector2,
    pub velocity: Vector2,
    pub controller: PlayerController,
    pub friction: f64,
    pub dead: bool,
    pub win: bool,
    pub anim_frame: u32
}

impl Player {
    pub fn new(sprite: Sprite, rect: Rect) -> Player {
        Player {
            flip: false,
            grounded: false,
            powerup: PowerupState::None,
            sprite,
            hitbox: (rect, Collider { interaction: None }),
            accel: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            controller: PlayerController {
                down: false,
                up: false,
                left: false,
                right: false,
            },
            friction: 0.0,
            dead: false,
            win: false,
            anim_frame: 0
        }
    }

    pub fn update(&mut self, dt: f64, object: &mut Vec<Object>) {
        self.accel = Vector2::new(0.0, 20.0);
        self.grounded = false;

        if self.controller.left {
            self.accel.x = -10.0;
            self.flip = true;
        } else if self.controller.right {
            self.accel.x = 10.0;
            self.flip = false;
        };

        let collision = self.hitbox.1.collision(&self.hitbox.0, &object.to_vec());

        if collision {
            if let Some((interact, pos, kind)) = &self.hitbox.1.interaction {
                if *kind == ObjectKind::SPIKE {
                    self.dead = true;
                }
                match interact {
                    Event::Down => {
                        self.grounded = true;
                        self.velocity.y = 0.0;
                        self.hitbox.0.y = *pos;
                    }
                    Event::Up => {
                        self.velocity.y = 0.0;
                        self.hitbox.0.y = *pos;
                    }
                    Event::Right => {
                        self.velocity.x = 0.0;
                        self.hitbox.0.x = *pos;
                    }
                    Event::Left => {
                        self.velocity.x = 0.0;
                        self.hitbox.0.x = *pos;
                    }
                }
            }
        }

        if self.controller.up {
            if self.grounded {
                self.velocity.y = -10.0;
                self.grounded = false;
            };
        };

        self.accel.x += self.velocity.x * -self.friction;
        self.velocity
            .evaluate_accel(self.accel.x * dt, self.accel.y * dt);

        self.hitbox.0.x += self.velocity.x;
        self.hitbox.0.y += self.velocity.y;
    }

    pub fn handle_keypress(&mut self, e: &piston_window::Event) {
        if let Some(b) = e.press_args() {
            if let Button::Keyboard(key) = b {
                match key {
                    Key::Left => self.controller.left = true,
                    Key::Right => self.controller.right = true,
                    Key::Space => {
                        if self.dead {
                            self.dead = false
                        } else {
                            self.controller.up = true
                        }
                    }
                    Key::Up => self.controller.up = true,
                    Key::Down => self.controller.down = true,
                    Key::K => self.dead = true,
                    Key::W => self.win = true,
                    _ => {}
                }
            }
        }

        if let Some(b) = e.release_args() {
            if let Button::Keyboard(key) = b {
                match key {
                    Key::Left => self.controller.left = false,
                    Key::Right => self.controller.right = false,
                    Key::Space | Key::Up => self.controller.up = false,
                    Key::Down => self.controller.down = false,
                    _ => {}
                }
            }
        }
    }
}

impl SpriteEv for Player {
    fn render(&mut self, e: &WindowEvent, w: &mut PistonWindow) {
        // Normal walk frames
        let texture = &self.sprite[(if self.flip { 1 } else { 0 })];
        let rect = &self.hitbox.0;
        w.draw_2d(e, |c, g, _| {
            image(
                texture,
                c.trans(rect.x, rect.y)
                    .scale(
                        rect.s  / texture.get_width() as f64,
                        rect.s / texture.get_height() as f64,
                    )
                    .transform,
                g,
            );
        });
    }
}
