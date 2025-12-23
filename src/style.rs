/// [Color] struct containing an RGBA `[u8; 4]` array. 
#[derive(Clone, Copy)]
pub struct Color([u8; 4]); 

impl Color {
    /// `Color([0, 0, 0, 0])`. 
    pub const TRANSPARENT: Color = Color([0, 0, 0, 0]); 
    /// `Color([0, 0, 0, 255])`. 
    pub const BLACK : Color = Color([0, 0, 0, 255]); 
    /// `Color([255, 255, 255, 255])`. 
    pub const WHITE : Color = Color([255, 255, 255, 255]);
    /// `Color([255, 0, 0, 255])`. 
    pub const RED   : Color = Color([255, 0, 0, 255]); 
    /// `Color([0, 255, 0, 255])`. 
    pub const GREEN : Color = Color([0, 255, 0, 255]);
    /// `Color([0, 0, 255, 255])`. 
    pub const BLUE  : Color = Color([0, 0, 255, 255]); 

    /// Constructor. Creates a [Color] based on provided 
    // `color` `[u8; 4]` RGBA array. 
    pub fn new(color: [u8; 4]) -> Self { 
        Self(color) 
    }

    /// Returns rgba `[u8; 4]` in `self`. 
    pub fn rgba(self) -> [u8; 4] { 
        self.0 
    }
}


/// [Style] struct containing visual options for a shape.
#[derive(Clone, Copy)] 
pub struct Style { 
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
}

impl Style {
    /// Constructor. Initializes [Style] struct with given `color` and `fill` option. 
    pub fn new(stroke: Option<Color>, fill: Option<Color>) -> Self { 
        Self { fill, stroke }
    }

    /// Constructor. Initializes [Style] struct for a stroke-only object given `color`. 
    pub const fn stroke(color: Color) -> Self { 
        Self { fill: None, stroke: Some(color) } 
    }

    /// Constructor. Initializes [Style] struct for a filled-only object given `color`. 
    pub const fn fill(color: Color) -> Self { 
        Self { fill: Some(color), stroke: None  }
    }
}




