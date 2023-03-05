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

    // the step constant going around the circumference.
    const CIRCLE_STEPS: f64 = 0.02;
    const TORUS_STEPS: f64 = 0.02;

    // generate the angles
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    const SCREEN_WIDTH: f64 = 100.0;
    const SCREEN_HEIGHT: f64 = 64.0;
    // K Constants
    // Calculate K1 with the screen size which is the terminal
    let k_2: f64 = 5.0;
    let k_1: f64 = SCREEN_HEIGHT * k_2 * 3.0 / (8.0 * (RADIUS + X_OFFSET));

    loop {
        a += 0.007;
        b += 0.003;
        let mut screen_output = [[' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
        let mut z_buffer = [[0.0; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // rotate the torus around y axis
        // cargo clippy suggests TaU in std instead of 6.28
        for phi in 0..(TAU / TORUS_STEPS) as usize {
            // rotate the angle around the circle
            for theta in 0..(TAU / CIRCLE_STEPS) as usize {
                // hold the angle in the circle
                let theta = theta as f64 * CIRCLE_STEPS;
                // hold the angle in torus
                let phi = phi as f64 * TORUS_STEPS;

                // store all sin and cos of the angles.
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                let sin_a = a.sin();
                let cos_a = a.cos();
                let sin_b = b.sin();
                let cos_b = b.cos();

                let circle_point = Point2D {
                    x: cos_theta * RADIUS + X_OFFSET,
                    y: sin_theta * RADIUS,
                };

                let transformed_point = Point3D {
                    x: circle_point.x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                        - circle_point.y * cos_a * sin_b,
                    y: circle_point.x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                        + circle_point.y * cos_a * cos_b,
                    z: circle_point.x * cos_a * sin_phi + circle_point.y * sin_a,
                };

                // now we need to calculate the screen position of the 3D transformed_point

                // store inverse of z+k_2 for performance
                let inverse_z = 1.0 / (transformed_point.z + k_2);

                // since we are writing this on a terminal which doesn't consider the origin as the
                // middle. we have to offset the drawing to match our calculation.

                let screen_point = Point2D {
                    x: SCREEN_WIDTH / 2.0 + k_1 * inverse_z * transformed_point.x,
                    y: SCREEN_HEIGHT / 2.0 - k_1 * inverse_z * transformed_point.y, // - because y goes
                                                                                    // downwards.
                };

                // now time to create lighting which follows the same matrix calculation cause a circle
                // at the orgin mirrors the surface normal.
                // we will also do a dot product on it to calcluate the number.
                let lighting = {
                    cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                        + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi)
                };

                // screen_point needs to be in a usize format
                let x_pixel = screen_point.x as usize;
                let y_pixel = screen_point.y as usize;

                // if it is less than 0 then it is dark
                if lighting > 0.0 && z_buffer[y_pixel][x_pixel] < inverse_z {
                    z_buffer[y_pixel][x_pixel] = inverse_z;
                    let luminance_index = (lighting * 8.0) as usize;
                    screen_output[y_pixel][x_pixel] = ".,-~:;=!*#$@"
                        .chars()
                        .nth(luminance_index)
                        .expect("luminance_index is not between 0-11")
                }
            }
        }
        // now draw it onto the terminal
        print!("\x1b[H"); // clears the terminal
        for row in screen_output {
            for pixel in row {
                print!("{}", pixel);
            }
            println!();
        }
    }
}
