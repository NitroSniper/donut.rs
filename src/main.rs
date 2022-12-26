use std::f64::consts::TAU;
fn main() {
    // let (x, y, z) = (R_2, 0, 0) + (r*cos(theta), r*sin(theta), 0)
    // R_2 is the x offset and r is the radius. 
    const X_OFFSET: f64 = 2.0;
    const RADIUS: f64 = 1.0;
    

    // TASK: Generate points in a circle.
    
    // the step constant going around the circumference. 
    const CIRCLE_STEPS: f64 = 0.02;

    // hold the angle in the circle
    let mut theta: f64;
    

    // x and y coords of the circle.
    let mut circle_x: f64;
    let mut circle_y: f64;
    
    // cargo clippy suggests TAU in std instead of 6.28
    // generate the angles
    for step in 0..(TAU/CIRCLE_STEPS) as usize {
        theta = step as f64 *CIRCLE_STEPS;
        circle_x = theta.cos()*RADIUS + X_OFFSET;
        circle_y = theta.sin()*RADIUS;
        
        println!("{}, {}", circle_x, circle_y);
    }
}
