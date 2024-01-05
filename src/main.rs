use nannou::prelude::*;

static SIGMA: f32 = 10.0;
static RHO: f32 = 28.0;
static BETA: f32 = 8.0 / 3.0;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

fn main() {
    // nannou::sketch(view).run()
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    // Store window ID so we can refer to the window later if needed
    _window: WindowId,
    x: f32,
    y: f32,
    z: f32,
    points: Vec<(Point2, Hsv)>,
    hue: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Lorenz Attractor")
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        x: 0.01,
        y: 0.0,
        z: 0.0,
        points: Vec::new(),
        hue: 0.0,
    }
}

// Only want to handle timed updates (60times a sec)
fn update(_app: &App, model: &mut Model, _update: Update) {
    let dt = 0.01;
    let dx: f32 = (SIGMA * (model.y - model.x)) * dt;
    let dy: f32 = (model.x * (RHO - model.z) - model.y) * dt;
    let dz: f32 = (model.x * model.y - BETA * model.z) * dt;
    model.x = model.x + dx;
    model.y = model.y + dy;
    model.z = model.z + dz;

    model.hue += 0.1;
    if model.hue > 360.0 {
        model.hue = 0.0;
    }
    model
        .points
        .push((pt2(model.x, model.y), hsv(deg_to_rad(model.hue), 1.0, 0.3)));
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    // Begin drawing
    let draw = app.draw().scale(7.5);
    draw.polyline()
        .weight(0.2)
        .points_colored(model.points.clone());

    draw.to_frame(app, &frame).unwrap();
}
