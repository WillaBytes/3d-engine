pub mod camera {
    use std::f32::consts::PI;

    // Consts
    const TWO_PI: f32 = 2.0 * PI;
    const HALF_PI: f32 = 0.5 * PI;

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Camera {
        pub position: [f32; 3],
        pub angle_h: f32,
        pub angle_v: f32,
        pub mouse_sensitivity: f32,
        pub depth_factor: f32,
    }

    impl Camera {
        pub fn new(
            position: [f32; 3],
            angle_h: f32,
            angle_v: f32,
            mouse_sensitivity: f32,
            depth_factor: f32,
        ) -> Self {
            Self {
                position,
                angle_h,
                angle_v,
                mouse_sensitivity,
                depth_factor,
            }
        }

        pub fn matrix(&self) -> [[f32; 3]; 3] {
            let sin_h = self.angle_h.sin();
            let cos_h = self.angle_h.cos();
            let sin_v = self.angle_v.sin();
            let cos_v = self.angle_v.cos();

            [
                [cos_h,     sin_h * sin_v,  -sin_h * cos_v],
                [0.0,       cos_v,          sin_v         ],
                [sin_h,     -cos_h * sin_v, cos_h * cos_v ],
            ]
        }

        pub fn adjust_angle_h(&mut self, increment: f32) {
            self.angle_h += increment * self.mouse_sensitivity;

            if self.angle_h >= TWO_PI { self.angle_h -= TWO_PI; }
            if self.angle_h <= -TWO_PI { self.angle_h += TWO_PI; }
        }

        pub fn adjust_angle_v(&mut self, increment: f32, limit: f32) {
            // For the sake of not breaking your neck ;)
            if (self.angle_v < limit && self.angle_v > -limit)
                || (increment < 0.0 && self.angle_v < -limit)
                || (increment > 0.0 && self.angle_v > limit)
            {
                self.angle_v += increment * self.mouse_sensitivity;
                if self.angle_v > limit { self.angle_v = limit - 0.000001; }
                if self.angle_v < -limit { self.angle_v = -limit + 0.000001; }
            }
        }

        pub fn direction(&self) -> [f32; 3] {
            [-self.angle_h.sin(), self.angle_v.sin(), self.angle_h.cos()]
        }

        pub fn direction_h(&self) -> [f32; 3] {
            [-self.angle_h.sin(), 0.0, self.angle_h.cos()]
        }

        pub fn move_to(&mut self, destination: [f32; 3]) {
            self.position = destination;
        }

        pub fn move_relative(&mut self, distance: [f32; 3]) {
            self.position[0] += distance[0];
            self.position[1] += distance[1];
            self.position[2] += distance[2];
        }

        pub fn move_forward(&mut self, increment: f32) {
            let camera_direction = self.direction();

            self.position[0] += camera_direction[0] * increment;
            self.position[1] += camera_direction[1] * increment;
            self.position[2] += camera_direction[2] * increment;
        }

        pub fn move_backward(&mut self, increment: f32) {
            let camera_direction = self.direction();

            self.position[0] -= camera_direction[0] * increment;
            self.position[1] -= camera_direction[1] * increment;
            self.position[2] -= camera_direction[2] * increment;
        }

        pub fn move_left(&mut self, increment: f32) {
            let camera_direction = self.direction_h();

            self.position[0] -= camera_direction[2] * increment;
            self.position[2] += camera_direction[0] * increment;
        }

        pub fn move_right(&mut self, increment: f32) {
            let camera_direction = self.direction_h();

            self.position[0] += camera_direction[2] * increment;
            self.position[2] -= camera_direction[0] * increment;
        }

        pub fn move_up(&mut self, increment: f32) {
            self.position[1] += increment;
        }

        pub fn move_down(&mut self, increment: f32) {
            self.position[1] -= increment;
        }

        pub fn move_forward_h(&mut self, increment: f32) {
            let camera_direction = self.direction_h();

            self.position[2] += camera_direction[2] * increment;
            self.position[0] += camera_direction[0] * increment;
        }

        pub fn move_backward_h(&mut self, increment: f32) {
            let camera_direction = self.direction_h();

            self.position[0] -= camera_direction[0] * increment;
            self.position[2] -= camera_direction[2] * increment;
        }

    }
}
