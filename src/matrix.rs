use crate::math::{vector3_add, vector3_cross, vector3_dot, vector3_normalize, vector3_sub};
use crate::types::{Camera, Vec3};
use crate::vector::Vec4;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4 { m: [[0.0; 4]; 4] }
    }
}

#[allow(dead_code)]
pub fn get_matrix4_identity() -> Matrix4 {
    Matrix4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn get_matrix4_scale(sx: f32, sy: f32, sz: f32) -> Matrix4 {
    Matrix4 {
        m: {
            [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
    }
}

pub fn get_matrix4_translation(tx: f32, ty: f32, tz: f32) -> Matrix4 {
    Matrix4 {
        m: {
            [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
    }
}

pub fn get_matrix4_rotation_x(angle: f32) -> Matrix4 {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix4 {
        m: {
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
    }
}

pub fn get_matrix4_rotation_y(angle: f32) -> Matrix4 {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix4 {
        m: {
            [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
    }
}

pub fn get_matrix4_rotation_z(angle: f32) -> Matrix4 {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix4 {
        m: {
            [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
    }
}

#[allow(dead_code)]
pub fn matrix4_mul_matrix4(matrix1: Matrix4, matrix2: Matrix4) -> Matrix4 {
    let mut res = Matrix4::default();
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                res.m[i][j] += matrix1.m[i][k] * matrix2.m[k][j];
            }
        }
    }
    res
}

pub fn get_fps_view_matrix(camera: &mut Camera) -> Matrix4 {
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let x_rotation_matrix = get_matrix4_rotation_x(camera.rotation.x);
    let y_rotation_matrix = get_matrix4_rotation_y(camera.rotation.y);
    let mut rotation_matrix = matrix4_mul_matrix4(x_rotation_matrix, get_matrix4_identity());
    rotation_matrix = matrix4_mul_matrix4(y_rotation_matrix, rotation_matrix);
    camera.direction = matrix4_mul_vec4(rotation_matrix, target.into()).into();
    camera.position = vector3_add(
        camera.position,
        matrix4_mul_vec4(y_rotation_matrix, camera.velocity.into()).into(),
    );
    camera.velocity = Vec3::default();
    target = vector3_add(camera.direction, camera.position);

    let view_matrix = get_look_at_view_matrix(
        camera.position,
        target,
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    );
    view_matrix
}

pub fn get_look_at_view_matrix(camera_pos: Vec3, target: Vec3, up_view: Vec3) -> Matrix4 {
    let mut z = vector3_sub(target, camera_pos);
    vector3_normalize(&mut z);
    let mut x = vector3_cross(up_view, z);
    vector3_normalize(&mut x);
    let y = vector3_cross(z, x);

    let view_matrix = Matrix4 {
        m: [
            [x.x, x.y, x.z, -vector3_dot(x, camera_pos)],
            [y.x, y.y, y.z, -vector3_dot(y, camera_pos)],
            [z.x, z.y, z.z, -vector3_dot(z, camera_pos)],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    view_matrix
}

pub fn get_projection_matrix(fov: f32, aspect_ratio: f32) -> Matrix4 {
    let field_of_view_scaling = 1.0 / (fov as f32 / 2.0).tan();
    let z_far = 100.0;
    let z_near = 0.1;
    let z_normalizer_left = z_far / (z_far - z_near);
    let z_normalizer_right = -(z_far / (z_far - z_near) * z_near);
    let res = Matrix4 {
        m: [
            [aspect_ratio * field_of_view_scaling as f32, 0.0, 0.0, 0.0],
            [0.0, field_of_view_scaling as f32, 0.0, 0.0],
            [0.0, 0.0, z_normalizer_left, z_normalizer_right],
            [0.0, 0.0, 1.0, 0.0],
        ],
    };
    res
}

pub fn matrix4_mul_vec4(matrix: Matrix4, vec: Vec4) -> Vec4 {
    let res = Vec4 {
        x: (matrix.m[0][0] * vec.x)
            + (matrix.m[0][1] * vec.y)
            + (matrix.m[0][2] * vec.z)
            + (matrix.m[0][3] * vec.w),
        y: (matrix.m[1][0] * vec.x)
            + (matrix.m[1][1] * vec.y)
            + (matrix.m[1][2] * vec.z)
            + (matrix.m[1][3] * vec.w),
        z: (matrix.m[2][0] * vec.x)
            + (matrix.m[2][1] * vec.y)
            + (matrix.m[2][2] * vec.z)
            + (matrix.m[2][3] * vec.w),
        w: (matrix.m[3][0] * vec.x)
            + (matrix.m[3][1] * vec.y)
            + (matrix.m[3][2] * vec.z)
            + (matrix.m[3][3] * vec.w),
    };
    res
}
