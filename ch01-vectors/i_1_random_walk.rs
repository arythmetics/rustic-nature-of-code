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
    let choice = random_range(0.0,1.0);

    if choice < 0.30
    {
        model.x = model.x + 1.0;
    }
    else if choice < 0.50
    {
        model.x = model.x - 1.0;
    }
    else if choice < 0.70
    {
        model.y = model.y + 1.0
    }
    else if choice < 1.0
    {
        model.y = model.y - 1.0
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    if app.elapsed_frames()==1
    {
        draw.background().color(PLUM);
    }
    
    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(5.0, 5.0)
        .color(STEELBLUE)
        .stroke(BLACK);
    
    draw.to_frame(app, &frame).unwrap();
}