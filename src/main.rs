#![feature(unique_rc_arc)]

mod tree_tree;

use tiny_skia::*;

const WIDTH: u32 = 500;
const HEIGTH: u32 = 500;

fn draw_tree() {
    let mut pixmap = Pixmap::new(WIDTH, HEIGTH).unwrap();

    // Set stroke color
    let mut stroke_paint = Paint::default();
    stroke_paint.set_color_rgba8(0, 127, 0, 200);
    stroke_paint.anti_alias = true;

    // Set stroke path
    let stroke_path = {
        let mut pb = PathBuilder::new();
        const RADIUS: f32 = 250.0;
        const CENTER: f32 = 250.0;
        pb.move_to(CENTER + RADIUS, CENTER);
        for i in 1..15 {
            let a = 2. * i as f32;
            pb.line_to(CENTER + RADIUS * a.cos(), CENTER + RADIUS * a.sin());
        }
        pb.finish().unwrap()
    };

    // Set stroke properties
    let mut stroke = Stroke::default();
    stroke.width = 6.0;
    stroke.line_cap = LineCap::Round;

    // Draw stroke
    pixmap.fill(Color::WHITE);
    pixmap.stroke_path(
        &stroke_path,
        &stroke_paint,
        &stroke,
        Transform::identity(),
        None,
    );

    // Save image
    pixmap.save_png("image.png").unwrap();
}

fn main() {
    draw_tree();
}
