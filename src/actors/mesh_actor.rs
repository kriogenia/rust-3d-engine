use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::color::{ColorBuilder, palette};
use crate::actors::actor::Drawable;
use crate::actors::Actor;
use crate::engine::EngineCamera;
use crate::geometry::Mesh;
use crate::geometry::Projectable;
use crate::math::builders::{RotationAxis, RotationMatrixBuilder};
use crate::geometry::vector::ops::{Dot, Module, Normalizable};

/// Implementation of an actor with a mesh
pub struct MeshActor {
    mesh: Mesh,
}

impl MeshActor {
    /// Creates a new actor with the given mesh
    ///
    /// # Arguments
    /// * `mesh` - Mesh of the actor
    ///
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh }
    }
}

impl Drawable for MeshActor {
    fn draw(&self, canvas: &mut dyn Canvas, camera: &EngineCamera) {
        let offset = camera.offset();
        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        let light = camera.light().normal();
        for triangle in &self.mesh.triangles {
            let plain = triangle.plain_component().apply_offset(offset);
            let normal = triangle.normal();
            if normal.dot(&(&plain - camera.position())) < 0.0 {
                let brightness = light.dot(&normal) * (u8::MAX as f32);
                let color = ColorBuilder::new().with_alpha(brightness as u8).build();

                let projection = triangle.get_projection(camera.projection_matrix(),
                                                         offset, width, height);

                canvas.fill_triangle((projection.0.x as u32, projection.0.y as u32),
                                     (projection.1.x as u32, projection.1.y as u32),
                                     (projection.2.x as u32, projection.2.y as u32),
                                     color);
                /*
                canvas.draw_triangle((projection.0.x as u32, projection.0.y as u32),
                                     (projection.1.x as u32, projection.1.y as u32),
                                     (projection.2.x as u32, projection.2.y as u32),
                                     palette::BLACK);*/
            }
        }
    }
}

impl Actor for MeshActor {
    fn update(&mut self, delta: u128) {
        let matrix_x = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::X)
            .with_speed(0.005)
            .with_theta(delta as f32 * 0.1)
            .build();
        let matrix_z = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::Z)
            .with_speed(-0.0025)
            .with_theta(delta as f32 * 0.1)
            .build();

        for triangle in &mut self.mesh.triangles {
            triangle.rotate(&matrix_x);
            triangle.rotate(&matrix_z)
        }
    }
}
