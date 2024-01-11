use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    phase: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    Model { phase: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update the phase to make the sine wave move
    model.phase += 0.02;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    // Draw the sine wave
    draw_sine_wave(&draw, model.phase);

    // Render the frame
    draw.to_frame(app, &frame).unwrap();
}

fn draw_sine_wave(draw: &Draw, phase: f32) {
    // Set up sine wave parameters
    let amplitude = 100.0;
    let frequency = 0.02;
    let num_points = 800;

    // Calculate and draw points for the sine wave
    let points = (0..=num_points)
        .map(|i| {
            let x = map_range(i, 0, num_points, 0.0, 800.0);
            let y = amplitude * (frequency * x + phase).sin();
            pt2(x, y)
        })
        .collect::<Vec<_>>();

    draw.polyline().color(BLACK).weight(2.0).points(points);
}
