use crate::{Stage, Color}; 

/// Draws a line in pixel coords. 
///
/// Arguments: 
/// - stage: &mut [Stage] 
/// - xy1_px: ([isize], [isize])
/// - xy2_px: ([isize], [isize])
/// - color: [Color] 
pub(crate) fn draw_line( 
    stage: &mut Stage, 
    xy1_px: (isize, isize), 
    xy2_px: (isize, isize), 
    color: Color
) { 
    let Some((xy1_px, xy2_px)) = clip_line_to_stage(stage, xy1_px, xy2_px) else { return; };

    let (x1, y1) = xy1_px; 
    let (x2, y2) = xy2_px;

    debug_assert!((x2 - x1) != isize::MIN);
    debug_assert!((y2 - y1) != isize::MIN);

    let mut x = x1; 
    let mut y = y1; 

    let dx = (x2 - x1).abs(); 
    let dy = (y2 - y1).abs(); 

    let sx = (x2 - x1).signum(); 
    let sy = (y2 - y1).signum();
 
    // Bresenham line
    if dx >= dy { 
        let mut err = 2 * dy - dx; 

        for _ in 0..=dx { 
            stage.plot(x, y, color); 

            if err >= 0 { 
                y += sy; 
                err -= 2 * dx; 
            }

            x += sx; 
            err += 2 * dy; 
        }
    } else { 
        let mut err = 2 * dx - dy; 

        for _ in 0..=dy { 
            stage.plot(x, y, color); 

            if err >= 0 { 
                x += sx; 
                err -= 2 * dy; 
            }

            y += sy; 
            err += 2 * dx; 
        }
    }
} 

#[inline(always)]
fn out_code(
    x: isize, 
    y: isize, 
    xmin: isize, 
    ymin: isize, 
    xmax: isize, 
    ymax: isize
) -> u8 {
    let mut c = 0u8;
    if x < xmin { c |= 1; }
    else if x > xmax { c |= 2; }
    if y < ymin { c |= 4; }
    else if y > ymax { c |= 8; }
    c
}

/// Cohen–Sutherland clip.
/// Returns `None` if fully outside; otherwise clipped endpoints.
fn clip_line_to_stage(
    stage: &Stage,
    p0: (isize, isize),
    p1: (isize, isize),
) -> Option<((isize, isize), (isize, isize))> {
    let xmin = 0isize;
    let ymin = 0isize;
    let xmax = stage.width() as isize - 1;
    let ymax = stage.height() as isize - 1;

    let (mut x0, mut y0) = p0;
    let (mut x1, mut y1) = p1;

    let mut c0 = out_code(x0, y0, xmin, ymin, xmax, ymax);
    let mut c1 = out_code(x1, y1, xmin, ymin, xmax, ymax);

    loop {
        if (c0 | c1) == 0 {
            return Some(((x0, y0), (x1, y1)));
        }
        if (c0 & c1) != 0 {
            return None;
        }

        let c_out = if c0 != 0 { c0 } else { c1 };

        // i64 to avoid overflow 
        let (x0i, y0i, x1i, y1i) = (x0 as i64, y0 as i64, x1 as i64, y1 as i64);
        let dx = x1i - x0i;
        let dy = y1i - y0i;
        let (xi, yi): (i64, i64);

        if (c_out & 8) != 0 {
            // y = ymax 
            if dy == 0 { return None; }
            yi = ymax as i64;
            xi = x0i + dx * (yi - y0i) / dy;
        } else if (c_out & 4) != 0 {
            // y = ymin
            if dy == 0 { return None; }
            yi = ymin as i64;
            xi = x0i + dx * (yi - y0i) / dy;
        } else if (c_out & 2) != 0 {
            // x = xmax
            if dx == 0 { return None; }
            xi = xmax as i64;
            yi = y0i + dy * (xi - x0i) / dx;
        } else {
            // x = xmin
            if dx == 0 { return None; }
            xi = xmin as i64;
            yi = y0i + dy * (xi - x0i) / dx;
        }

        let xn = xi as isize;
        let yn = yi as isize;

        if c_out == c0 {
            x0 = xn; y0 = yn;
            c0 = out_code(x0, y0, xmin, ymin, xmax, ymax);
        } else {
            x1 = xn; y1 = yn;
            c1 = out_code(x1, y1, xmin, ymin, xmax, ymax);
        }
    }
}
