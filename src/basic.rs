use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model;

fn model(_app: &App) -> Model {
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update logic goes here
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    // Drawing logic goes here

    // For example, draw a red rectangle
    draw.rect().color(RED);

    // Finish drawing and present the frame
    draw.to_frame(_app, &frame).unwrap();
}
