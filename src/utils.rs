use std::fs::File;
use std::{io::prelude::Read, path::PathBuf};

use piston_window::{Flip, PistonWindow};

use crate::{obj::{Object, ObjectKind}, sprite::Sprite};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// X pos
    pub x: f64,
    /// Y pos
    pub y: f64,
    /// Width of rect
    pub w: f64,
    /// Height of rect
    pub h: f64,
    /// Overall scale of rect
    pub s: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64, s: f64) -> Rect {
        Rect { x, y, w, h, s }
    }

    pub fn l(&self) -> f64 {
        self.x + self.w
    }

    pub fn r(&self) -> f64 {
        self.x + self.s - self.w
    }

    pub fn u(&self) -> f64 {
        self.y + self.h
    }

    pub fn d(&self) -> f64 {
        self.y + self.s - self.h
    }

    pub fn c(&self) -> Vector2 {
        Vector2 {
            x: self.x + self.s / 2.0,
            y: self.y + self.s / 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn evaluate_accel(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Clone, Debug)]
pub struct PlayerController {
    pub up: bool,
    pub down: bool,
    pub left: bool, 
    pub right: bool
}

// pub enum Directions {
//     UP,
//     DOWN,
//     LEFT,
//     RIGHT,
// }

/// Returns `(Vec<Object>, f64, f64)`
///
/// `=` - ground tile
///
/// `-` - underground tile
///
/// `^` - spike
pub fn parse_level_tiles(assets: PathBuf, level_name: &str, w: &mut PistonWindow) -> (Vec<Object>, f64, f64) {
    let mut file = File::open(assets.join(level_name)).expect("Unable to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read file");

    let mut tilemap: Vec<Vec<char>> = Vec::new();

    for line in file_content.lines() {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        tilemap.push(row);
    }

    let (m_w, m_h) = (tilemap.first().unwrap().len() as f64 * 40.0, tilemap.len() as f64 * 40.0);

    let ground_texture = Sprite::texture_load(assets.join("placeholder-tile.png"), w, Flip::None);
    let underground_tx = Sprite::texture_load(assets.join("placeholder-tile.png"), w, Flip::None);

    let mut objects: Vec<Object> = Vec::new();

    for (row, tiles) in tilemap.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            match tile {
                '=' => {
                    let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
                    let ground_tile = Sprite::new(ground_texture.clone());
                    let object = Object::new(ground_tile, rect, true, ObjectKind::GROUND);
                    objects.push(object);
                },
                '-' => {
                    let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
                    let ground_tile = Sprite::new(underground_tx.clone());
                    let object = Object::new(ground_tile, rect, true, ObjectKind::GROUND);
                    objects.push(object);
                },
                '^' => {
                    let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
                    let ground_tile = Sprite::new(underground_tx.clone());
                    let object = Object::new(ground_tile, rect, true, ObjectKind::SPIKE);
                    objects.push(object);
                }
                _ => (),
            }
        }
    }

    (objects, m_w, m_h)
}

#[derive(Clone, Debug)]
pub enum PowerupState {
    None
}
