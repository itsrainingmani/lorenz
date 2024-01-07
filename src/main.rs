use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::collections::VecDeque;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // nannou::sketch(view).run()
    nannou::app(model).update(update).view(view).run();
}

struct Angles {
    roll: f32,
    pitch: f32,
    yaw: f32,
}

impl Angles {
    fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self { roll, pitch, yaw }
    }
}

struct Settings {
    scale: f32,
    speed: usize,
}

struct Model {
    x: f32,
    y: f32,
    z: f32,
    // points: Vec<(Point2, Hsv)>,
    points: VecDeque<(Point3, f32)>,
    hue: f32,
    egui: Egui,
    settings: Settings,
    angles: Angles,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Lorenz Attractor")
        .view(view)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let egui = Egui::from_window(&app.window(window_id).unwrap());

    Model {
        x: 0.01,
        y: 0.0,
        z: 0.0,
        points: VecDeque::with_capacity(5000),
        hue: 0.0,
        egui,
        settings: Settings {
            scale: 7.5,
            speed: 1,
        },
        angles: Angles::new(0.0, 0.0, 0.0),
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Plus | Key::Equals => model.settings.scale += 0.1,
        Key::Minus => model.settings.scale -= 0.1,
        Key::Up => model.angles.pitch += 0.01,
        Key::Down => model.angles.pitch -= 0.01,
        Key::Left => model.angles.yaw += 0.01,
        Key::Right => model.angles.yaw -= 0.01,
        Key::R => {
            // Restart the whole thing
            // *model = Model::new(model._window);
        }
        _ => {}
    }
}

fn calc_lorenz(x: &mut f32, y: &mut f32, z: &mut f32, dt: f32) {
    let sigma: f32 = 10.0;
    let rho: f32 = 28.0;
    let beta: f32 = 8.0 / 3.0;
    *x += (sigma * (*y - *x)) * dt;
    *y += (*x * (rho - *z) - *y) * dt;
    *z += (*x * *y - beta * *z) * dt;
}

// Only want to handle timed updates (60times a sec)
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(_update.since_start);

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("SPEED:");
        ui.add(egui::Slider::new(&mut model.settings.speed, 0..=5));
    });

    model.hue += 0.05;
    if model.hue > 360.0 {
        model.hue = 0.0;
    }

    for _ in 0..model.settings.speed {
        calc_lorenz(&mut model.x, &mut model.y, &mut model.z, 0.01);
        model
            .points
            // .push((pt2(model.x, model.y), hsv(deg_to_rad(model.hue), 1.0, 0.3)));
            .push_back((pt3(model.x, model.y, model.z), model.hue));
        if model.points.len() > 5000 {
            model.points.pop_front();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    // Begin drawing
    let draw = app
        .draw()
        .scale(model.settings.scale)
        .pitch(model.angles.pitch)
        .yaw(model.angles.yaw);
    // This only draws lines in 2D
    // draw.polyline()
    //     .weight(0.2)
    //     .points_colored(model.points.clone());

    for (pt, hue) in &model.points {
        draw.ellipse()
            .radius(0.2)
            .x_y_z(pt.x, pt.y, pt.z)
            .hsv(deg_to_rad(*hue), 0.6, 0.6);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
