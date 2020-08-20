use std::ops;
use itertools::Itertools;
use rayon::prelude::*;

const SCALING_FACTOR: u32 = 255;
const PPM_LINE_LEN: usize = 70;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }

    pub fn fmap<F>(&self, f: F) -> Self
        where F: Fn(f32) -> f32 {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.0 - other.0).abs() < f32::EPSILON && 
        (self.1 - other.1).abs() < f32::EPSILON && 
        (self.2 - other.2).abs() < f32::EPSILON
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(
            self.0 + other.0, 
            self.1 + other.1, 
            self.2 + other.2, 
            )
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
         Color(
            self.0 - other.0, 
            self.1 - other.1, 
            self.2 - other.2, 
            )
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color(
            self.0 * other, 
            self.1 * other, 
            self.2 * other, 
            )
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(
            self.0 * other.0, 
            self.1 * other.1, 
            self.2 * other.2, 
            )
    }
}

pub struct Canvas {
    colors: Vec<Color>,
    w: usize,
    h: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let colors = vec![Color(0_f32, 0_f32, 0_f32); w * h];
        Self { colors, w, h }
    }

    pub fn new_fn<F>(w: usize, h: usize, f: F) -> Self 
    where
        F: Fn(usize, usize) -> Color {
        let mut colors = vec![Color(0_f32, 0_f32, 0_f32); w * h];
        for i in 0..w {
            for j in 0..h {
                colors[i + j * w] = f(i, j);
            }
        }
        Self { colors, w, h }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.w
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let i = self.get_index(x, y);
        self.colors[i] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.colors[self.get_index(x, y)]
    }

    pub fn write_ppm(&self) -> String {
        let header = format!("P3 {} {} {}", self.w, self.h, SCALING_FACTOR);
        let content = (0..self.h).cartesian_product(0..self.w)
            .map(|(y, x)| {
                let c = (self.pixel_at(x, y) * (SCALING_FACTOR as f32))
                    .fmap(|x| clamp(x, 0.0, SCALING_FACTOR as f32) + 0.5); // the 0.5 is for rounding
                [format!("{}", c.0 as u32), format!("{}", c.1 as u32), format!("{}", c.2 as u32)]
            }).collect::<Vec<[String; 3]>>().concat().iter()
            .fold((0, String::from("")), |(line_len, all_str), s| {
                if all_str.len() == 0 {
                    (s.len(), s.to_string())
                } else {
                    if line_len + s.len() + 1 >= PPM_LINE_LEN {
                        (s.len(), format!("{}\n{}", all_str, s))
                    } else {
                        (line_len + s.len() + 1, format!("{} {}", all_str, s))
                    }
                }
            }).1;
        // we need a trailing newline, some programs (like imagemagick) need that in order to work
        format!("{}\n{}\n", header, content)
    }
}

#[test]
fn test() {
    let c1 = Color(1.0, 0.2, 0.4);
    let c2 = Color(0.9, 1.0, 0.1);
    assert!(c1 * c2 == Color(0.9, 0.2, 0.04));

    let mut canvas = Canvas::new(10, 20);
    let red = Color(1.0, 0.0, 0.0);
    canvas.write_pixel(2, 3, red);
    assert!(canvas.pixel_at(2, 3) == red);

    let mut canvas = Canvas::new(5, 3);
    let c1 = Color( 1.5, 0.0, 0.0);
    let c2 = Color( 0.0, 0.5, 0.0);
    let c3 = Color(-0.5, 0.0, 1.0);
    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);
    let s = canvas.write_ppm();
    println!("{}", s);
    assert!(s == "P3 5 3 255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 255
");

    let canvas = Canvas::new_fn(10, 2, |_, _| Color(1.0, 0.8, 0.6));
    let s = canvas.write_ppm();
    println!("{}", s);
    assert!(s =="P3 10 2 255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255
204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153
");
}
