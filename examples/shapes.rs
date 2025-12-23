use wave::{Stage, Color, Style};
use wave::shapes::{line, circle, square, rectangle, triangle, equilateral_triangle};

fn disp_stage(
    stage: &mut Stage,
    label: &str,
    stroke: Option<Color>,
    fill: Option<Color>,
) {
    let (w, h) = stage.dimensions();

    let stroke_rgba = stroke.map(|c| c.rgba());
    let fill_rgba   = fill.map(|c| c.rgba());

    for y in 0..h {
        for x in 0..w {
            let p = stage.pixels()[y * w + x];

            let ch = if p[3] == 0 {
                '·'
            } else if Some(p) == stroke_rgba {
                'S' // stroke
            } else if Some(p) == fill_rgba {
                'F' // fill
            } else {
                '#'
            };

            print!("{ch}");
        }
        println!();
    }

    println!("{label}\n");
    stage.clear(Color::TRANSPARENT);
}

fn draw_line(stage: &mut Stage) {
    let xy1 = (-1.0, -2.0);
    let xy2 = (1.0, 1.0);

    let style = Style::stroke(Color::WHITE);
    line(stage, xy1, xy2, style);

    disp_stage(stage, "Line (stroke)", Some(Color::WHITE), None);
}

fn draw_circ(stage: &mut Stage) {
    let orig = (1.0, 1.0);
    let radius = 4.0;

    let style = Style::new(Some(Color::RED), Some(Color::WHITE));
    circle(stage, orig, radius, style);

    disp_stage(stage, "Circle (stroke + fill)", Some(Color::WHITE), Some(Color::RED));
}

fn draw_square(stage: &mut Stage) {
    let orig = (-1.0, 0.0);
    let side_length = 6.0;

    let style = Style::fill(Color::RED);
    square(stage, orig, side_length, style);

    disp_stage(stage, "Square (fill)", None, Some(Color::RED));
}

fn draw_rect(stage: &mut Stage) {
    let orig = (5.0, 3.0);
    let width = 7.0;
    let height = 8.0;

    let style = Style::new(Some(Color::GREEN), Some(Color::WHITE));
    rectangle(stage, orig, height, width, style);

    disp_stage(stage, "Rectangle (stroke + fill)", Some(Color::WHITE), Some(Color::GREEN));
}

fn draw_triangle(stage: &mut Stage) {
    let xy1 = (0.0, -2.0);
    let xy2 = (0.0, 2.0);
    let xy3 = (8.0, 3.0);

    let style = Style::new(Some(Color::BLUE), Some(Color::WHITE));
    triangle(stage, xy1, xy2, xy3, style);

    disp_stage(stage, "Triangle (stroke + fill)", Some(Color::WHITE), Some(Color::BLUE));

    let origin = (-1.0, 1.0);
    let side_length = 9.0;

    let style = Style::stroke(Color::GREEN);
    equilateral_triangle(stage, origin, side_length, style);

    disp_stage(stage, "Equilateral triangle (stroke)", Some(Color::GREEN), None);
}

fn main() {
    let mut stage = Stage::new(20, 15);

    draw_line(&mut stage);
    draw_circ(&mut stage);
    draw_square(&mut stage);
    draw_rect(&mut stage);
    draw_triangle(&mut stage);
}

