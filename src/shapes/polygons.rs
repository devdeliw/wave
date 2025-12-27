use crate::{Stage, Style, Path};

const SQRT3: f32 = 1.7320508;

/// Draws a line in world coords from `xy1` to `xy2`. 
///
/// Arguments: 
/// - stage: &mut [`Stage`] - stage to draw onto. 
/// - xy1: ([f32], [f32]) - coord for first point. 
/// - xy2: ([f32], [f32]) - coord for second point. 
/// - style: [`Style`] - struct containing style args. 
pub fn line( 
    stage: &mut Stage, 
    xy1: (f32, f32), 
    xy2: (f32, f32), 
    style: Style, 
) { 
    let nodes = Vec::from([xy1, xy2]);
    let line_path = Path::new(nodes, false); 

    line_path.render(stage, style); 
}

/// Draws a triangle using three world coords. 
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - xy1: ([f32], [f32]) - coord for first vertex. 
/// - xy2: ([f32], [f32]) - coord for second vertex. 
/// - xy3: ([f32], [f32]) - coord for third vertex. 
/// - style: [Style] - struct containing style args. 
pub fn triangle( 
    stage: &mut Stage, 
    xy1: (f32, f32), 
    xy2: (f32, f32), 
    xy3: (f32, f32), 
    style: Style, 
) { 
    let nodes = Vec::from([xy1, xy2, xy3]); 
    let triangle_path = Path::new(nodes, true); 

    triangle_path.render(stage, style); 
}

/// Draws a rectangle centered on `origin` of given `width` and `height` in world coords.
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - coords for origin. 
/// - width: [f32] - width of rectangle. 
/// - height: [f32] - height of rectangle. 
/// - style: [Style] - struct containing style args. 
pub fn rectangle( 
    stage: &mut Stage, 
    origin: (f32, f32), 
    width: f32, 
    height: f32, 
    style: Style, 
) { 
    if !height.is_finite() || height <= 0.0 || !width.is_finite() || width <= 0.0 { 
        return; 
    } 

    // pixel coords
    let (stage_width, stage_height) = stage.dimensions();

    // clamp to stage 
    let stage_width = stage_width as f32; 
    let stage_height = stage_height as f32; 
    let min_x = -stage_width / 2.0; 
    let max_x = stage_width / 2.0;  
    let min_y = -stage_height / 2.0; 
    let max_y = stage_height / 2.0;  
    
    let (x, y) = origin; 
    let hhalf = height / 2.0; 
    let whalf = width / 2.0; 

    let l = (x - whalf).max(min_x); 
    let r = (x + whalf).min(max_x); 
    let t = (y + hhalf).min(max_y); 
    let b = (y - hhalf).max(min_y); 

    let tl = (l, t); 
    let bl = (l, b); 
    let tr = (r, t); 
    let br = (r, b); 

    let nodes = Vec::from([tl, tr, br, bl]); 
    let rectangle_path = Path::new(nodes, true);
    rectangle_path.render(stage, style); 
} 


/// Draws an equilateral triangle centered on `origin` of given `side_length`. For arbitrary
/// triangles use [triangle] 
///
/// Arguments:
/// - stage: &mut [Stage] - stage to draw onto.
/// - origin: ([f32], [f32]) - center coord.
/// - side_length: [f32] - side length.
/// - style: [Style] - struct containing style args.
pub fn equilateral_triangle( 
    stage: &mut Stage, 
    origin: (f32, f32), 
    side_length: f32, 
    style: Style, 
) { 
    if !side_length.is_finite() || side_length <= 0.0 { 
        return; 
    }

    let (xc, yc) = origin; 

    // dy from origin to top and bottom 
    let apex_dy = (SQRT3 / 3.0) * side_length; 
    let base_dy = (SQRT3 / 6.0) * side_length; 

    let ybase = yc - base_dy; 
    let yapex = yc + apex_dy; 

    let xy1 = (xc, yapex); 
    let xy2 = (xc - side_length * 0.5, ybase); 
    let xy3 = (xc + side_length * 0.5, ybase); 

    let nodes = Vec::from([xy1, xy2, xy3]); 
    let equilateral_triangle_path = Path::new(nodes, true); 
    equilateral_triangle_path.render(stage, style); 
}


/// Draws a square centered on `origin` of given `side_length`. 
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - center coord. 
/// - side_length: [f32] - side length. 
/// - style: [Style] - struct containing style args. 
pub fn square( 
    stage: &mut Stage, 
    origin: (f32, f32), 
    side_length: f32, 
    style: Style
) { 
    if !side_length.is_finite() || side_length <= 0.0 { 
        return; 
    } 

    let (stage_width, stage_height) = stage.dimensions(); 
    let stage_width = stage_width as f32; 
    let stage_height = stage_height as f32; 

    let xmin = -stage_width / 2.0; 
    let xmax = stage_width / 2.0; 
    let ymin = -stage_height / 2.0; 
    let ymax = stage_height / 2.0; 

    let (xc, yc) = origin; 
    let side_half = side_length / 2.0; 
    let l = (xc - side_half).max(xmin); 
    let r = (xc + side_half).min(xmax); 
    let t = (yc + side_half).min(ymax); 
    let b = (yc - side_half).max(ymin); 

    let tl = (l, t); 
    let tr = (r, t); 
    let bl = (l, b); 
    let br = (r, b); 

    let nodes = Vec::from([tl, tr, br, bl]); 
    let square_path = Path::new(nodes, true); 
    square_path.render(stage, style); 
}

