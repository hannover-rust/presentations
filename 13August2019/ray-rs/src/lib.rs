extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render() -> Vec<u8> {
    utils::set_panic_hook();

    let width: usize = 400;
    let height: usize = 400;

    log!("Rendering a {} by {} image", width, height);

    render_test(width, height)
}

fn render_test(width: usize, height: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    for y in 0..width {
        for x in 0..height {
            buf.push((x % 255) as u8);
            buf.push((y % 255) as u8);
            buf.push(128u8);
            buf.push(255u8);
        }
    }

    buf
}

/*
trait DotProduct<T> {
    fn dot(a: T, b: T) -> f32;
}

trait AMinusBK<T> {
    fn a_minus_bk(a: T, b: T) -> T;
}

#[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl DotProduct<Pixel> for Pixel {
    fn dot(a: Pixel, b: Pixel) -> f32 {
        a.r as f32 * b.r as f32 + a.g as f32 * b.g as f32 + a.b as f32 * b.b as f32 
    }
}
#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Image {
        let mut img = Image {
            width: 600,
            height: 480,
            pixels: Vec::new(),
        };
        for x in 0..img.width {
            for y in 0..img.height {
                img.pixels.push( Pixel {
                    r: x as u8,
                    g: y as u8,
                    b: (x + y) as u8,
                    a: 0,
                    })
            }
        }
        img
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}

const w: f32 = 600.0;

struct Coord {
    x: f32,
    y: f32,
    z: f32,
}

struct Sphere {
    radius: f32,
    coord: Coord,
    rgb: Pixel,
    spec: f32, // specular
    refl: f32, // reflectiveness
}

const SCENE: [Sphere; 4] = [
    Sphere { radius: w,   coord: Coord {x: 0.0,  y: -w,  z: 0.0}, rgb: Pixel { r: 9, g: 9, b: 0, a: 255}, spec: w, refl: 2.0},
    Sphere { radius: 1.0, coord: Coord {x: 0.0,  y: 0.0, z: 3.0}, rgb: Pixel { r: 9, g: 0, b: 0, a: 255}, spec: w, refl: 3.0},
    Sphere { radius: 1.0, coord: Coord {x: -2.0, y: 1.0, z: 4.0}, rgb: Pixel { r: 0, g: 9, b: 0, a: 255}, spec: 9.0, refl: 4.0},
    Sphere { radius: 1.0, coord: Coord {x: 2.0,  y: 1.0, z: 4.0}, rgb: Pixel { r: 0, g: 0, b: 9, a: 255}, spec: w, refl: 5.0},
];

const AMBIENT_LIGHT: f32 = 2.0;

struct Light {
    intensity: f32,
    coord: Coord,
}

const LIGHTS: [Light; 1] = [
    Light {
        intensity: 8.0,
        coord: Coord {
            x: 2.0,
            y: 2.0,
            z: 0.0,
        }
    }
];
*/