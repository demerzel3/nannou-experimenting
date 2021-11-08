use libm::*;
use nannou::prelude::pow;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400, 400)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn draw_triangle(draw: &Draw, mouse: Point2, position: Point2) {
    let rotation = atan2((mouse.x - position.x) as f64, (mouse.y - position.y) as f64) as f32;
    let distance = sqrtf(pow(mouse.x - position.x, 2) + pow(mouse.y - position.y, 2)) / 5.0;

    //let radius = 50.0;
    let radius = fmin(distance as f64, 25.0) as f32;
    let points = (0..=360).step_by(120).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), if i == 0 { RED } else { STEELBLUE })
    });
    draw.polygon()
        .points_colored(points)
        .rotate(-rotation)
        .xy(position);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    draw_triangle(&draw, app.mouse.position(), pt2(0.0, 0.0));
    (0..=360).step_by(360 / 6).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 45.0;
        let y = radian.cos() * 45.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });
    (0..=360).step_by(360 / 12).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 90.0;
        let y = radian.cos() * 90.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });
    (0..=360).step_by(360 / 18).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 135.0;
        let y = radian.cos() * 135.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });
    (0..=360).step_by(360 / 24).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 180.0;
        let y = radian.cos() * 180.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });
    (0..=360).step_by(360 / 30).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 225.0;
        let y = radian.cos() * 225.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });
    (0..=360).step_by(360 / 36).for_each(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * 270.0;
        let y = radian.cos() * 270.0;
        draw_triangle(&draw, app.mouse.position(), pt2(x, y));
    });

    draw.to_frame(app, &frame).unwrap();
}
