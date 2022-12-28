use std::f64::consts::TAU;

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}


#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

fn main() {
    // let (x, y, z) = (R_2, 0, 0) + (r*cos(theta), r*sin(theta), 0)
    // R_2 is the x offset and r is the radius. 
    const X_OFFSET: f64 = 2.0;
    const RADIUS: f64 = 1.0;
    

    // TaSK: Generate points in a circle.
    
    // the step constant going around the circumference. 
    const CIRCLE_STEPS: f64 = 0.02;
    const TORUS_STEPS: f64 = 0.02;
    
    // generate the angles
    let a: f64 = 0.0;
    let B: f64 = 0.0;

    // K Constants
    let k_1: f64 = 1.0;
    let k_2: f64 = 1.0;

    // rotate the torus around y axis
    // cargo clippy suggests TaU in std instead of 6.28
    for phi in 0..(TAU/TORUS_STEPS) as usize {
        // rotate the angle around the circle
        for theta in 0..(TAU/CIRCLE_STEPS) as usize {
        
            // hold the angle in the circle 
            let theta = theta as f64 * CIRCLE_STEPS;
            // hold the angle in torus
            let phi = phi as f64 * TORUS_STEPS;

            //generate point in circle with trig
            let circle_points = Point2D { 
                x: theta.cos()*RADIUS + X_OFFSET,
                y: theta.sin()*RADIUS,
            };
            
            // create torus Point struct with the rotation matrix
            let torus_points = Point3D {
                x: circle_points.x * phi.cos(),
                y: circle_points.y,
                z: circle_points.x * phi.sin()
            };

            // now we want to transform it by rotating in on the x and z axis
            let transformed_point = Point3D {
                x: torus_points.x * B.cos() - torus_points.y*a.cos()*B.sin() + torus_points.z*a.sin()*B.sin(),
                y: torus_points.x * B.sin() + torus_points.y*a.cos()*B.cos() - torus_points.z*a.sin()*B.cos(),
                z: torus_points.y*a.sin() + torus_points.z*a.cos(),
            };
            
            // now we need to translate it to the user to a 2D screen
            
            let screen_point = Point2D { 
                x: k_1*transformed_point.x/(k_2 + transformed_point.z),
                y: k_1*transformed_point.y/(k_2 + transformed_point.z),
            };
            

            // old implementation above. since I have an understanding it is time to optimize the
            // calculation. 
            
            // store all sin and cos of the angles. 
            
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            let sin_a = a.sin();
            let cos_a = a.cos();
            let sin_B = B.sin();
            let cos_B = B.cos();



            let circle_point = Point2D {
                x: cos_theta*RADIUS + X_OFFSET, 
                y: sin_theta*RADIUS
            };

            let transformed_point_new = Point3D {
                x: circle_point.x*(cos_B*cos_phi + sin_a*sin_B*sin_phi) - circle_point.y*cos_a*sin_B,
                y: circle_point.x*(sin_B*cos_phi - sin_a*cos_B*sin_phi) + circle_point.y*cos_a*cos_B,
                z: circle_point.x*cos_a*sin_phi+circle_point.y*sin_a,
            };
            assert_eq!(transformed_point.x, transformed_point_new.x);
            assert_eq!(transformed_point.y, transformed_point_new.y);
            assert_eq!(transformed_point.z, transformed_point_new.z);

             

            // I'm gonna combine all the matrix calculation.
        }
    }
}
