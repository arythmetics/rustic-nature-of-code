use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model 
{
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let x = 0.0;
    let y = 0.0;
    
    let _window = app.new_window().view(view).build().unwrap();
    Model 
    { 
        x,
        y, 
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) 
{
    let choice = random_range(1,7);

    if choice == 1 || choice == 6
    {
        model.x = model.x + 1.0;
    }
    else if choice == 2
    {
        model.x = model.x - 1.0;
    }
    else if choice == 3
    {
        model.y = model.y + 1.0
    }
    else if choice == 4 || choice == 5
    {
        model.y = model.y - 1.0
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(20.0, 20.0)
        .color(STEELBLUE)
        .stroke(BLACK);
    
    draw.to_frame(app, &frame).unwrap();
}