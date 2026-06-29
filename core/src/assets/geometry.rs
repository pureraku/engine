const PI: f32 = std::f32::consts::PI;

pub struct GeometryData {
    pub vertices: Vec<f32>,
    pub stride: i32,
}

fn push_vertex(v: &mut Vec<f32>, px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32, u: f32, w: f32) {
    v.extend_from_slice(&[px, py, pz, nx, ny, nz, u, w]);
}

pub fn cube() -> GeometryData {
    GeometryData {
        stride: 8,
        vertices: vec![
            // front
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            // back
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
            // left
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 1.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 1.0,
            // right
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
            // top
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
            // bottom
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
        ],
    }
}

pub fn uv_sphere(stacks: i32, slices: i32) -> GeometryData {
    let stacks = stacks.max(2);
    let slices = slices.max(3);

    let mut v = Vec::new();
    for i in 0..stacks {
        let v0 = i as f32 / stacks as f32;
        let v1 = (i + 1) as f32 / stacks as f32;
        let phi0 = v0 * PI;
        let phi1 = v1 * PI;

        for j in 0..slices {
            let u0 = j as f32 / slices as f32;
            let u1 = (j + 1) as f32 / slices as f32;

            let theta0 = u0 * 2.0 * PI;
            let theta1 = u1 * 2.0 * PI;

            let x00 = phi0.sin() * theta0.cos();
            let y00 = phi0.cos();
            let z00 = phi0.sin() * theta0.sin();

            let x10 = phi1.sin() * theta0.cos();
            let y10 = phi1.cos();
            let z10 = phi1.sin() * theta0.sin();

            let x01 = phi0.sin() * theta1.cos();
            let y01 = phi0.cos();
            let z01 = phi0.sin() * theta1.sin();

            let x11 = phi1.sin() * theta1.cos();
            let y11 = phi1.cos();
            let z11 = phi1.sin() * theta1.sin();

            push_vertex(&mut v, x00, y00, z00, x00, y00, z00, u0, v0);
            push_vertex(&mut v, x10, y10, z10, x10, y10, z10, u0, v1);
            push_vertex(&mut v, x11, y11, z11, x11, y11, z11, u1, v1);

            push_vertex(&mut v, x00, y00, z00, x00, y00, z00, u0, v0);
            push_vertex(&mut v, x11, y11, z11, x11, y11, z11, u1, v1);
            push_vertex(&mut v, x01, y01, z01, x01, y01, z01, u1, v0);
        }
    }

    GeometryData { vertices: v, stride: 8 }
}
