use crate::{obj::Object, utils::Rect};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub interaction: Option<(Event, f64)>,
}

impl Collider {
    pub fn collision(&mut self, player: &Rect, objects: &Vec<Object>) -> bool {
        for obj in objects.iter() {
            if obj.solid {
                if player.r() >= obj.rect.l()
                    && player.l() <= obj.rect.r()
                    && player.d() >= obj.rect.u()
                    && player.u() <= obj.rect.d()
                {
                    let x = player.c().x - obj.rect.c().x;
                    let y = player.c().y - obj.rect.c().y;

                    if y * y > x * x {
                        if y > 0.0 {
                            self.interaction = Some((Event::Up, obj.rect.d()));
                            return true;
                        } else {
                            self.interaction = Some((Event::Down, obj.rect.u() - player.s));
                            return true;
                        }
                    } else {
                        if x > 0.0 {
                            self.interaction = Some((Event::Left, obj.rect.r()));
                            return true;
                        } else {
                            self.interaction = Some((Event::Right, obj.rect.l() - player.s));
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
