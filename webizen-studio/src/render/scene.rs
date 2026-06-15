//! Backend-agnostic scene primitives: vector math, camera, projected points.
//!
//! These compile on every target (no `web-sys`), so the same camera/projection
//! logic is shared by the CPU Canvas2D backend and any future GPU backend.

/// Minimal 3D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn scale(self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len <= f64::EPSILON {
            Self::default()
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }
}

/// A world point after projection into screen space, with retained depth so
/// callers can fade/scale by distance.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenPoint {
    pub x: f64,
    pub y: f64,
    pub depth: f64,
}

/// A look-at perspective camera. `focal_scale` multiplies `min(viewport)` to set
/// the effective focal length, matching the original physics-surface tuning.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub world_up: Vec3,
    pub focal_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vec3::new(0.0, 8.0, 18.0),
            target: Vec3::new(0.0, 0.5, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            focal_scale: 0.88,
        }
    }
}

impl Camera {
    /// Project a world point to screen space for the given `(width, height)`
    /// viewport. Returns `None` for points at or behind the near plane.
    pub fn project(&self, world: Vec3, viewport: (f64, f64)) -> Option<ScreenPoint> {
        let forward = self.target.sub(self.eye).normalize();
        let right = forward.cross(self.world_up).normalize();
        let up = right.cross(forward).normalize();
        let relative = world.sub(self.eye);

        let view_x = relative.dot(right);
        let view_y = relative.dot(up);
        let view_z = relative.dot(forward);

        if view_z <= 0.2 {
            return None;
        }

        let focal = viewport.0.min(viewport.1) * self.focal_scale;
        Some(ScreenPoint {
            x: viewport.0 * 0.5 + (view_x / view_z) * focal,
            y: viewport.1 * 0.52 - (view_y / view_z) * focal,
            depth: view_z,
        })
    }
}
