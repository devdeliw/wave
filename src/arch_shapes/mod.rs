pub(crate) mod lines; 
pub use lines::line; 

pub(crate) mod circles; 
pub use circles::circle; 

pub(crate) mod triangles; 
pub use triangles::triangle; 
pub use triangles::equilateral_triangle;

mod rectangles; 
pub use rectangles::rectangle; 
pub use rectangles::square; 
