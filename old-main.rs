mod gmlib;
mod camera;
use crate::gmlib::matrix::*;
use crate::camera::camera::*;
use std::f32::consts::PI;

/*
struct Camera {
    position: Vec3,
    angle: Vec2,
    scale_factor: f32,
    depth_factor: f32,
}

impl Camera {
    fn direction(&self) -> Vec3 {
        Vec3 {
            x_1: self.angle.x_1.cos() * self.angle.x_2.sin(),
            x_2: self.angle.x_1.sin(),
            x_3: self.angle.x_1.cos() * self.angle.x_2.cos(),
        }
    }
}
*/

fn main() {
    let camera = crate::camera::camera::Camera {
        position: Vec3 { x_1: 0.0, x_2: 0.0, x_3: 0.0 },
        angle: [0.0 / 4.0, 3.0 * PI / 4.0], 
        scale_factor: 1.0,
        depth_factor: 1.0,
    };

    let cam_vec = Vec3 {
        x_1: 1.0,
        x_2: 0.0,
        x_3: 1.0,
    };

    let cam_vec_transformed = camera.transform() * cam_vec;

    println!("{}\n", cam_vec_transformed);

    println!("{}\n", camera.direction());

    println!("{}", camera.transform() * camera.direction());

    /*let mat3x3_test = Mat3x3 {
        x_11: 1.0, x_12: 2.0, x_13: 3.0,
        x_21: 4.0, x_22: 5.0, x_23: 6.0,
        x_31: 7.0, x_32: 8.0, x_33: 9.0,
    };*/

    let vec2_test1 = Vec2::from([2.0, 3.0]);

    let vec3_test1 = Vec3 {
        x_1: 1.0,
        x_2: 2.0,
        x_3: 3.0,
    };

    let vec3_test2 = Vec3 {
        x_1: 3.0,
        x_2: 2.0,
        x_3: 1.0,
    };

    let mat2x2_test1 = Mat2x2::from([
        3.0, 4.0,
        1.0, 2.0,
    ]);

    let mat3x3_test1 = Mat3x3::from([
        1.0, 3.0, 2.0,
        5.0, 1.0, 2.0,
        4.0, 2.0, 1.0,
    ]);

    let mat3x3_test2 = Mat3x3::from([
        -1.0/6.0,  1.0/18.0,  2.0/9.0,
         1.0/6.0, -7.0/18.0,  4.0/9.0,
         1.0/3.0,  5.0/9.0,  -7.0/9.0,
    ]);

    let mat4x4_test1 = Mat4x4::from([
        3.0, 1.0, 2.0, 5.0,
        4.0, 4.0, 3.0, 6.0,
        1.0, 3.0, 1.0, 7.0,
        -2.0, 2.0, 1.0, 2.0,
    ]);

    println!();

    /*
    println!("Vec3 test vector 1:\n{}\n", vec3_test1);
    println!("Vec3 test vector 2:\n{}\n", vec3_test2);
    println!("Vec3 scalar product:\n{}\n", vec3_test1 * vec3_test2);
    println!("Vec3 cross product:\n{}\n", vec3_test1 % vec3_test2);
    println!("Vec3 normalization:\n{}\n", vec3_test1.norm());

    println!("Mat2x2 test matrix 1:\n{}\n", mat2x2_test1);
    println!("Mat2x2 inverse multiplication test:\n{}\n", mat2x2_test1 * mat2x2_test1.inverse());

    println!("Mat3x3 test matrix 1:\n{}\n", mat3x3_test1);
    println!("Mat3x3 test matrix 2:\n{}\n", mat3x3_test2);
    println!("Mat3x3 addition:\n{}\n", mat3x3_test1 + mat3x3_test2);
    println!("Mat3x3 subtraction:\n{}\n", mat3x3_test1 - mat3x3_test2);
    println!("Mat3x3-Mat3x3 multiplication:\n{}\n", mat3x3_test1 * mat3x3_test2);
    println!("Mat3x3-scalar multiplication:\n{}\n", mat3x3_test1 * 2.0);
    println!("Mat3x3-scalar division:\n{}\n", mat3x3_test1 / 2.0);
    println!("Mat3x3 negation:\n{}\n", -mat3x3_test1);
    println!("Mat3x3 exponentiation:\n{}\n", mat3x3_test1.pow(2));
    println!("Mat3x3 determinant:\n{}\n", mat3x3_test1.determinant());
    println!("Mat3x3 inverse:\n{}\n", mat3x3_test1.inverse());
    println!("Mat3x3-Vec3 multiplication:\n{}\n", UNIT_MAT3X3 * vec3_test1);
    println!("Mat3x3 inverse multiplication:\n{}\n", mat3x3_test1 * mat3x3_test1.inverse());

    println!("Mat4x4 test matrix 1:\n{}\n", mat4x4_test1);
    println!("Mat4x4-Mat4x4 multiplication:\n{}\n", mat4x4_test1 * mat4x4_test1);
    println!("Mat4x4 determinant:\n{}\n", mat4x4_test1.determinant());
    println!("Mat4x4 inverse:\n{}\n", mat4x4_test1.inverse());
    println!("Mat4x4 inverse multiplication:\n{}\n", mat4x4_test1 * mat4x4_test1.inverse());

    let u = Vec3::from([1.0, 1.0, 1.0]);
    let v = Vec3::from([-3.0, 2.0, 3.0]);
    let n = Vec3::from([-1.0, 0.0, 1.0]);
    let result = v % u - n * (2.0 * ((v % u) * n) / (n * n));

    println!("Result:\n{}\n", result);
    println!("{}", Mat3x3::from([
            6.0, 0.0, 0.0,
            2.0, 1.0, 0.0,
            1.0, 0.0, 1.0,
    ]).inverse());
    */
}
