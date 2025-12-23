/// `Stage` struct containing a row major framebuffer 
/// of length `width * height` containing RGBA `[u8; 4]` 
/// array for each pixel.
pub struct Stage { 
    width: usize, 
    height: usize, 
    framebuf: Vec<[u8; 4]> 
}


/// Constructor and Getters.
impl Stage { 
    /// Constructor. Initializes a `width` x `height` [Stage] 
    /// that is black and transparent.
    ///
    /// Arguments: 
    /// - width: [usize]: stage width. 
    /// - height: [usize]: stage height.
    ///
    /// Returns: 
    /// [Stage] of size `(width, height)`. 
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Stage must be strictly positive in size"); 
        let length = width
            .checked_mul(height)
            .expect("Stage dimensions overflow");

        Self { 
            width, 
            height, 
            framebuf: vec![[0, 0, 0, 0]; length], 
        }
    }

    /// Getter. Returns the width of the [Stage].
    pub fn width(&self) -> usize { 
        self.width 
    }           

    /// Getter. Returns the height of the [Stage].
    pub fn height(&self) -> usize { 
        self.height
    }

    /// Getter. Returns the dimensions `(width, height)` 
    /// of the [Stage].
    pub fn dimensions(&self) -> (usize, usize) { 
        (self.width, self.height)
    }

    /// Getter. Returns a reference to the [Stage] framebuffer.
    pub fn pixels(&self) -> &[[u8; 4]] { 
       &self.framebuf 
    }

    /// Getter. Returns a mutable reference to the [Stage]
    /// framebuffer. 
    pub fn pixels_mut (&mut self) -> &mut [[u8; 4]] { 
        &mut self.framebuf
    }

    /// Getter. Gets the color value of a pixel at `(x, y)`. 
    /// Returns `None` if pixel out-of-bounds. Otherwise `Some([u8; 4])`. 
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<[u8; 4]> {

        // quick return
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = self.index(x, y);
        Some(self.framebuf[index])
    }
 
    /// Getter. Returns the number of pixels in the [Stage].
    pub fn len(&self) -> usize { 
        self.framebuf.len()
    }

    /// Returns `true` if Stage is empty. 
    pub fn is_empty(&self) -> bool { 
        self.framebuf.is_empty()
    }
}


/// Setters. 
impl Stage { 
    /// Returns linear framebuffer index for the pixel 
    /// at `(x, y)` where `(0, 0)` is the top-left.
    fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        y * self.width + x
    }


    /// Setter. Sets the [Stage] background to provided `color`. 
    pub fn clear(&mut self, color: [u8; 4]) { 
        self.framebuf.fill(color); 
    } 


    /// Setter. Sets the color value of a pixel at `(x, y)`. 
    /// If pixel is out-of-bounds, silently does nothing.
    #[inline(always)]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 4]) {

        // silent quick return 
        if x >= self.width || y >= self.height {
            return;
        }

        let index = self.index(x, y);
        self.framebuf[index] = color;
    }


    /// Setter. Sets the color value of a signed pixel at `(x, y)`.
    /// If the pixel is out-of-bounds, silently does nothing.
    ///
    /// Hot path in drawing shapes.
    #[inline(always)]
    pub fn plot(&mut self, x: isize, y: isize, color: [u8; 4]) {
        if (x as usize) < self.width && (y as usize) < self.height {
            let idx = (y as usize) * self.width + (x as usize);
            self.framebuf[idx] = color;
        }
    }
}

/// Helpers 
impl Stage { 
    /// Returns the framebuffer as contiguous `&[u8]` slice of RGBA bytes 
    /// in row major order suitable for rendering.
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: 
        // framebuf is Vec<[u8; 4]>. arrays of u8 have no padding. 
        // so the data is tightly packed RGBA bytes.
        unsafe { 
            std::slice::from_raw_parts( 
                self.framebuf.as_ptr() as *const u8, 
                self.len() * std::mem::size_of::<[u8; 4]>()
            )
        }
    }

    /// Converts world/cartesian coordinates (origin at center) into 
    /// pixel coordinates (origin top-left).
    ///
    /// Returns 
    /// - `Some(isize, isize)`: if pixel coordinate is finite and representable
    /// - `None`: otherwise
    pub(crate) fn world_to_pixel(&self, (x, y): (f32, f32)) -> Option<(isize, isize)> {
        if !x.is_finite() || !y.is_finite() { 
            return None; 
        } 

        let center_x = (self.width as f32 - 1.0) * 0.5; 
        let center_y = (self.height as f32 - 1.0) * 0.5; 

        let px = (x + center_x).round(); 
        let py = (center_y - y).round();

        if px < isize::MIN as f32 || px > isize::MAX as f32 { return None; }
        if py < isize::MIN as f32 || py > isize::MAX as f32 { return None; }

        Some((px as isize, py as isize))
    }

    /// Fills given contiguous pixels at row `y` from `x0` to `x1` with given `color`. 
    /// `y`, `x0`, `x1` in pixel coords. 
    pub(crate) fn fill_span(&mut self, y: isize, x0: isize, x1: isize, color: [u8; 4]) {
        if (y as usize) >= self.height { return; } 

        let mut a = x0; 
        let mut b = x1; 
        if a > b { std::mem::swap(&mut a, &mut b); }

        if b < 0 || a >= self.width as isize { return; } 
        a = a.max(0); 
        b = b.min(self.width as isize - 1); 

        let y = y as usize; 
        let row = y * self.width; 
        self.framebuf[row + a as usize .. row + b as usize + 1].fill(color); 
    }


}
