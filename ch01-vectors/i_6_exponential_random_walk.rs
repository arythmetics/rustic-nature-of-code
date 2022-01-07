use nannou::prelude::*;
use nannou::rand::*;
use rand_distr::StandardNormal;

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
    let mut step: f32 = 0.0;

    while step == 0.0
    {
        let num = random_range(0,11);
        let p_num = u32::pow(num, 2);

        if p_num > random_range(0,11)
        {
            step = num as f32;
        }
    }

    if choice < 0.25
    {
        model.x = model.x + step
    }
    else if choice < 0.50
    {
        model.x = model.x - step
    }
    else if choice < 0.75
    {
        model.y = model.y + step
    }
    else if choice < 1.0
    {
        model.y = model.y - step
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