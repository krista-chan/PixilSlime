use crate::{obj::{Object, ObjectKind}, utils::Rect};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// `(direction, location, kind)`
    pub interaction: Option<(Event, f64, ObjectKind)>,
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
                            self.interaction = Some((Event::Up, obj.rect.d(), obj.kind));
                            return true;
                        } else {
                            self.interaction = Some((Event::Down, obj.rect.u() - player.s, obj.kind));
                            return true;
                        }
                    } else {
                        if x > 0.0 {
                            self.interaction = Some((Event::Left, obj.rect.r(), obj.kind));
                            return true;
                        } else {
                            self.interaction = Some((Event::Right, obj.rect.l() - player.s, obj.kind));
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
