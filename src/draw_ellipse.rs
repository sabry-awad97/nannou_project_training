// nannou = "0.18.0"

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model;

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update logic goes here
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Set the ellipse parameters
    let radius_x = 200.0;
    // let radius_y = 100.0;
    let color = nannou::color::WHITE;

    // Draw the ellipse at the center of the window
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(radius_x)
        .color(color);

    // Render the frame
    draw.to_frame(app, &frame).unwrap();
}
