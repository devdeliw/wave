use wave::{Stage, Style, Color, Opacity}; 
use wave::shapes::{circle, line, rectangle, triangle}; 

fn draw_cursor( 
    stage: &mut Stage, 
    center: (f32, f32), 
    length: f32, 
    style: Style, 
) {
    let (x, y) = center; 

    line(stage, (x - length/2.0, y), (x + length/2.0, y), style);
    line(stage, (x, y - length/2.0), (x, y + length/2.0), style); 
}

fn draw_circle( 
    stage: &mut Stage, 
    center: (f32, f32), 
    radius: f32, 
    style: Style, 
) { 
    circle(stage, center, radius, style); 
}

fn draw_rectangle( 
    stage: &mut Stage, 
    center: (f32, f32), 
    width: f32, 
    height: f32, 
    style: Style, 
) { 
    rectangle(stage, center, width, height, style); 
}

fn draw_triangle(
    stage: &mut Stage, 
    xy1: (f32, f32), 
    xy2: (f32, f32), 
    xy3: (f32, f32), 
    style: Style
) { 
    triangle(stage, xy1, xy2, xy3, style); 
}

fn main() { 
    let mut stage = Stage::new(1080, 1080);
    stage.clear(Color::BLACK);

    let cursor_center = (0.0, 0.0); 
    let circle_center = (0.0, 300.0);  
    let length = 100.0; 
    let radius = 50.0; 
    
    let mut style = Style::new(
        Some(Color::WHITE), 
        Some(Color::new([245, 40, 145, 255])), 
    );
    style.set_stroke_width(5.0); 

    style.set_stroke_opacity(Opacity::OPAQUE);
    style.set_fill_opacity(Opacity::OPAQUE);

    draw_cursor(&mut stage, cursor_center, length, style); 
    draw_circle(&mut stage, circle_center, radius, style); 

    let rectangle_center = (-300.0, 0.0); 
    let width = 200.0; 
    let height = 100.0; 
    draw_rectangle(&mut stage, rectangle_center, width, height, style); 

    let xy1 = (500.0, -200.0); 
    let xy2 = (-200.0, -100.0); 
    let xy3 = (300.0, -50.0);  
    
    draw_triangle(&mut stage, xy1, xy2, xy3, style); 

    stage.save_png("/Users/vinland/Desktop/cursor.png")
        .expect("Render Failed"); 
}
