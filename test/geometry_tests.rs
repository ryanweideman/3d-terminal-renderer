use crate::geometry::*;

#[cfg(test)]
mod tests {
    use crate::geometry::{
        clip_triangle_against_plane, transform_triangle_to_camera_coords, Color, FrustumPlane,
        Triangle3, Triangle4,
    };
    use nalgebra::{Matrix4, Point3, Point4, Vector3};

    #[test]
    fn test_transform_triangle_to_camera_coords() {
        let world_triangle = Triangle3 {
            vertices: [
                Point3::new(0.0, 10.0, 10.0),
                Point3::new(0.0, -10.0, -10.0),
                Point3::new(1.0, 3.0, 0.0),
            ],
            color: Color::new(255, 255, 255),
        };

        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(0.0, 0.0, -1.0);

        let global_up = Vector3::new(0.0, 1.0, 0.0);
        let right = direction.cross(&global_up).normalize();
        let up = right.cross(&direction);

        let view_matrix = Matrix4::look_at_rh(&origin, &(origin + direction), &up);

        let result = transform_triangle_to_camera_coords(&world_triangle, &view_matrix);

        let expected_triangle = Triangle3 {
            vertices: [
                Point3::new(0.0, 10.0, 10.0),
                Point3::new(0.0, -10.0, -10.0),
                Point3::new(1.0, 3.0, 0.0),
            ],
            color: Color::new(255, 255, 255),
        };

        assert_eq!(result, expected_triangle);
    }

    #[test]
    fn test_near_plane_one_clip() {
        let triangle = Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(0.0, 0.0, -2.0, 1.0),
                Point4::new(1.0, 0.0, -2.0, 1.0),
            ],
            color: Color::new(255, 255, 255),
        };

        let near_clipped_triangles =
            clip_triangle_against_plane(FrustumPlane::Near, &vec![triangle]);

        let expected_triangles = vec![Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(0.0, 0.0, -1.0, 1.0),
                Point4::new(0.5, 0.0, -1.0, 1.0),
            ],
            color: Color::new(255, 255, 255),
        }];

        assert_eq!(near_clipped_triangles, expected_triangles);
    }

    #[test]
    fn test_right_plane_one_clip() {
        let triangle = Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(2.0, 0.0, 0.0, 1.0),
                Point4::new(2.0, 0.0, -1.0, 1.0),
            ],
            color: Color::new(255, 255, 255),
        };

        let near_clipped_triangles =
            clip_triangle_against_plane(FrustumPlane::Right, &vec![triangle]);

        let expected_triangles = vec![Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(1.0, 0.0, 0.0, 1.0),
                Point4::new(1.0, 0.0, -0.5, 1.0),
            ],
            color: Color::new(255, 255, 255),
        }];

        assert_eq!(near_clipped_triangles, expected_triangles);
    }

    #[test]
    fn test_top_plane_one_clip() {
        let triangle = Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(0.0, 2.0, 0.0, 1.0),
                Point4::new(0.0, 2.0, -1.0, 1.0),
            ],
            color: Color::new(255, 255, 255),
        };

        let near_clipped_triangles =
            clip_triangle_against_plane(FrustumPlane::Top, &vec![triangle]);

        let expected_triangles = vec![Triangle4 {
            vertices: [
                Point4::new(0.0, 0.0, 0.0, 1.0),
                Point4::new(0.0, 1.0, 0.0, 1.0),
                Point4::new(0.0, 1.0, -0.5, 1.0),
            ],
            color: Color::new(255, 255, 255),
        }];

        assert_eq!(near_clipped_triangles, expected_triangles);
    }
}
