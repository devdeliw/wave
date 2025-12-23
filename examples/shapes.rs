use wave::{Stage, Color};
use wave::shapes::{line, circle, square, rectangle, triangle, equilateral_triangle};

fn disp_stage(stage: &mut Stage, label: &str) {
    let (w, h) = stage.dimensions();
    for y in 0..h {
        for x in 0..w {
            let p = stage.pixels()[y*w + x];
            print!("{}", if p[3] != 0 { "#" } else { "•" });
        }
        println!();
    }
    println!("{label}\n");
    stage.clear(Color::TRANSPARENT.rgba()); 
}

fn draw_line(stage: &mut Stage) {

    let color = Color::WHITE; 
    let xy1 = (-1.0, -2.0); 
    let xy2 = (1.0, 1.0); 

    line(stage, xy1, xy2, color); 
}

fn draw_circ(stage: &mut Stage) {

    let color = Color::WHITE; 
    let orig = (1.0, 1.0); 
    let radius = 2.0; 

    circle(stage, orig, radius, color); 
}

fn draw_square(stage: &mut Stage) { 
    
    let color = Color::RED; 
    let orig = (-1.0, 0.0); 
    let side_length = 4.0; 

    square(stage, orig, side_length, color); 
}

fn draw_rect(stage: &mut Stage) { 
    
    let color = Color::WHITE; 
    let orig = (5.0, 3.0); 
    let width = 5.0; 
    let height = 8.0; 

    rectangle(stage, orig, height, width, color); 
}

fn draw_triangle(stage: &mut Stage) { 
    let color = Color::GREEN; 
    let xy1 = (0.0, -2.0); 
    let xy2 = (0.0, 2.0); 
    let xy3 = (8.0, 3.0); 

    triangle(stage, xy1, xy2, xy3, color);
    disp_stage(stage, "Arbitrary Triangle"); 

    let origin = (-1.0, 1.0); 
    let side_length = 7.0; 
    equilateral_triangle(stage, origin, side_length, color);
    disp_stage(stage, "Equilateral Triangle"); 

    
}

fn main() {
    let mut stage = Stage::new(20, 15);

    draw_line(&mut stage);
    disp_stage(&mut stage, "Line");

    draw_circ(&mut stage); 
    disp_stage(&mut stage, "Circle");

    draw_square(&mut stage); 
    disp_stage(&mut stage, "Square"); 

    draw_rect(&mut stage); 
    disp_stage(&mut stage, "Rectangle"); 

    draw_triangle(&mut stage); 
}


