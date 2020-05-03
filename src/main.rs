use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let points = (0..50).map(|i| {
        let x = i as f32 - 25.0;
        let point = pt2(x, x.sin()) * 20.0;
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);
    draw.to_frame(app, &frame).unwrap();
}
