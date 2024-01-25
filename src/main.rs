use tiny_skia::*;

// Based on https://fiddle.skia.org/c/@compose_path

const WIDTH: u32 = 1920;
const HEIGTH: u32 = 1080;

fn main() {
    let mut pixmap = Pixmap::new(WIDTH, HEIGTH).unwrap();

    let mut stroke_paint = Paint::default();
    stroke_paint.set_color_rgba8(0, 127, 0, 200);
    stroke_paint.anti_alias = true;

    let stroke_path = {
        let mut pb = PathBuilder::new();
        const RADIUS: f32 = 250.0;
        const CENTER: f32 = 250.0;
        pb.move_to(CENTER + RADIUS, CENTER);
        for i in 1..8 {
            let a = 2.6927937 * i as f32;
            pb.line_to(CENTER + RADIUS * a.cos(), CENTER + RADIUS * a.sin());
        }
        pb.finish().unwrap()
    };

    let mut stroke = Stroke::default();
    stroke.width = 6.0;
    stroke.line_cap = LineCap::Round;
    stroke.dash = StrokeDash::new(vec![20.0, 40.0], 0.0);

    pixmap.fill(Color::WHITE);
    pixmap.stroke_path(
        &stroke_path,
        &stroke_paint,
        &stroke,
        Transform::identity(),
        None,
    );

    pixmap.save_png("image.png").unwrap();
}
