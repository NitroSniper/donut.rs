use std::f64::consts::TAU;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}


fn main() {
    // let (x, y, z) = (R_2, 0, 0) + (r*cos(theta), r*sin(theta), 0)
    // R_2 is the x offset and r is the radius. 
    const X_OFFSET: f64 = 2.0;
    const RADIUS: f64 = 1.0;
    

    // TASK: Generate points in a circle.
    
    // the step constant going around the circumference. 
    const CIRCLE_STEPS: f64 = 0.02;
    const TORUS_STEPS: f64 = 0.02;
    
    // cargo clippy suggests TAU in std instead of 6.28
    // generate the angles
    let A: f64 = 0.0;
    let B: f64 = 0.0;
    
    
    for phi in 0..(TAU/TORUS_STEPS) as usize {
        for theta in 0..(TAU/CIRCLE_STEPS) as usize {
        
            // hold the angle in the circle 
            let theta = theta as f64 * CIRCLE_STEPS;
            // hold the angle in torus
            let phi = phi as f64 * TORUS_STEPS;

            //generate point in circle with trig
            let circle_points = Point { 
                x: theta.cos()*RADIUS + X_OFFSET,
                y: theta.sin()*RADIUS,
                z: 0.0 // z will always be 0 so any matrix calculation ignore this field
            };
            
            // create torus Point struct with the rotation matrix
            let torus_points = Point {
                x: circle_points.x * phi.cos(),
                y: circle_points.y,
                z: circle_points.x * phi.sin()
            };

            // now we want to transform it by rotating in on the x and z axis
            
            let transformed_point = Point {
                x: torus_points.x * B.cos() - torus_points.y*A.cos()*B.sin() + torus_points.z*A.sin()*B.sin(),
                y: torus_points.x * B.sin() + torus_points.y*A.cos()*B.cos() - torus_points.z*A.sin()*B.cos(),
                z: torus_points.y*A.sin() + torus_points.z*A.cos(),
            };
        }
    }
}
