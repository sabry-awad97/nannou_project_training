use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    capture_done: bool,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    Model {
        capture_done: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Check if capture has been done, and capture the frame if not.
    if !model.capture_done {
        println!("Capturing frame...");
        // Capture the frame and save it as a PNG file.
        app.main_window()
            .capture_frame(app.exe_name().unwrap() + "_frame.png");
        model.capture_done = true;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing.
    let draw = app.draw();

    // Draw the x-axis.
    draw.line()
        .start(pt2(-400.0, 0.0))
        .end(pt2(400.0, 0.0))
        .color(STEELBLUE);

    // Draw the y-axis.
    draw.line()
        .start(pt2(0.0, -300.0))
        .end(pt2(0.0, 300.0))
        .color(STEELBLUE);

    // Finish drawing and render the frame.
    draw.to_frame(app, &frame).unwrap();
}
