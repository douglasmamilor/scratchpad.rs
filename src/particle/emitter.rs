use rand::random_range;

use crate::{Color, Vec2, particle::Particle};

pub enum EmitterShape {
    Point,
    Line { start: Vec2, end: Vec2 },
    Circle { radius: f32 },
    Rect { width: f32, height: f32 },
}

pub struct EmitterConfig {
    pub spawn_rate: f32,

    pub lifetime_min: f32,
    pub lifetime_max: f32,

    pub speed_min: f32,
    pub speed_max: f32,
    pub angle_min: f32,
    pub angle_max: f32,

    pub gravity: Vec2,

    pub color_start: Color,
    pub color_end: Color,
    pub size_start_min: f32,
    pub size_start_max: f32,
    pub size_end_min: f32,
    pub size_end_max: f32,
}

pub struct Emitter {
    pub position: Vec2,
    pub shape: EmitterShape,
    pub config: EmitterConfig,

    pub particles: Vec<Particle>,
    pub spawn_accumulator: f32,
    pub is_active: bool,
}

impl Emitter {
    pub fn new(position: Vec2, shape: EmitterShape, config: EmitterConfig) -> Self {
        Self {
            position,
            shape,
            config,
            particles: Vec::new(),
            spawn_accumulator: 0.0,
            is_active: true,
        }
    }

    pub fn fire(position: Vec2) -> Self {
        // Up is negative Y in screen space.
        let up = -std::f32::consts::FRAC_PI_2;
        let spread = std::f32::consts::FRAC_PI_6; // +/- 30 degrees

        let config = EmitterConfig {
            spawn_rate: 90.0,
            lifetime_min: 0.5,
            lifetime_max: 1.0,
            speed_min: 50.0,
            speed_max: 150.0,
            angle_min: up - spread,
            angle_max: up + spread,
            gravity: Vec2::new(0.0, -40.0),
            color_start: Color::RGBA(255, 220, 40, 255),
            color_end: Color::RGBA(255, 40, 0, 255),
            size_start_min: 8.0,
            size_start_max: 12.0,
            size_end_min: 2.0,
            size_end_max: 4.0,
        };

        Self::new(position, EmitterShape::Circle { radius: 6.0 }, config)
    }

    pub fn smoke(position: Vec2) -> Self {
        let up = -std::f32::consts::FRAC_PI_2;
        let spread = 0.9; // wider than fire

        let config = EmitterConfig {
            spawn_rate: 25.0,
            lifetime_min: 2.0,
            lifetime_max: 4.0,
            speed_min: 20.0,
            speed_max: 50.0,
            angle_min: up - spread,
            angle_max: up + spread,
            gravity: Vec2::new(0.0, -10.0),
            color_start: Color::RGBA(160, 160, 160, 140),
            color_end: Color::RGBA(80, 80, 80, 0),
            size_start_min: 10.0,
            size_start_max: 14.0,
            size_end_min: 40.0,
            size_end_max: 55.0,
        };

        Self::new(position, EmitterShape::Circle { radius: 8.0 }, config)
    }

    pub fn sparkle_trail(position: Vec2) -> Self {
        let config = EmitterConfig {
            spawn_rate: 50.0,
            lifetime_min: 0.3,
            lifetime_max: 0.6,
            speed_min: 0.0,
            speed_max: 20.0,
            angle_min: 0.0,
            angle_max: std::f32::consts::TAU,
            gravity: Vec2::new(0.0, 0.0),
            color_start: Color::RGBA(255, 255, 255, 255),
            color_end: Color::RGBA(255, 255, 255, 0),
            size_start_min: 6.0,
            size_start_max: 8.0,
            size_end_min: 0.0,
            size_end_max: 0.0,
        };

        Self::new(position, EmitterShape::Point, config)
    }

    pub fn rain(line_start: Vec2, line_end: Vec2) -> Self {
        let down = std::f32::consts::FRAC_PI_2;
        let spread = 0.25;

        let config = EmitterConfig {
            spawn_rate: 140.0,
            lifetime_min: 2.0,
            lifetime_max: 3.0,
            speed_min: 300.0,
            speed_max: 500.0,
            angle_min: down - spread,
            angle_max: down + spread,
            gravity: Vec2::new(0.0, 200.0),
            color_start: Color::RGBA(200, 220, 255, 160),
            color_end: Color::RGBA(200, 220, 255, 0),
            size_start_min: 2.0,
            size_start_max: 3.0,
            size_end_min: 2.0,
            size_end_max: 3.0,
        };

        Self::new(
            (line_start + line_end) * 0.5,
            EmitterShape::Line {
                start: line_start,
                end: line_end,
            },
            config,
        )
    }

    pub fn sparks_burst(position: Vec2) -> Self {
        let config = EmitterConfig {
            spawn_rate: 0.0,
            lifetime_min: 0.3,
            lifetime_max: 0.8,
            speed_min: 200.0,
            speed_max: 500.0,
            angle_min: 0.0,
            angle_max: std::f32::consts::TAU,
            gravity: Vec2::new(0.0, 300.0),
            color_start: Color::RGBA(255, 255, 255, 255),
            color_end: Color::RGBA(255, 140, 40, 0),
            size_start_min: 2.0,
            size_start_max: 4.0,
            size_end_min: 2.0,
            size_end_max: 4.0,
        };

        let mut emitter = Self::new(position, EmitterShape::Point, config);
        emitter.is_active = false;
        emitter
    }

    pub fn emit_burst(&mut self, count: usize) {
        for _ in 0..count {
            self.spawn_particle();
        }
    }

    pub fn spawn_particle(&mut self) {
        let pos = match self.shape {
            EmitterShape::Point => self.position,
            EmitterShape::Line { start, end } => start.lerp(end, random_range(0.0..1.0)),
            EmitterShape::Circle { radius } => {
                let angle = random_range(0.0..=std::f32::consts::TAU);
                let radius = radius * random_range(0.0f32..=1.0f32).sqrt(); // Uniform distribution in
                // circle
                Vec2::new(angle.cos(), angle.sin()) * radius + self.position
            }
            EmitterShape::Rect { width, height } => {
                Vec2::new(
                    random_range(-width / 2.0..=width / 2.0),
                    random_range(-height / 2.0..=height / 2.0),
                ) + self.position
            }
        };

        let speed = random_range(self.config.speed_min..=self.config.speed_max);
        let angle = random_range(self.config.angle_min..=self.config.angle_max);
        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        let particle = Particle {
            position: pos,
            velocity: vel,
            acceleration: self.config.gravity,
            age: 0.0,
            lifetime: random_range(self.config.lifetime_min..=self.config.lifetime_max),
            color_start: self.config.color_start,
            color_end: self.config.color_end,
            size_start: random_range(self.config.size_start_min..=self.config.size_start_max),
            size_end: random_range(self.config.size_end_min..=self.config.size_end_max),
            rotation: 0.0,
            angular_velocity: 0.0,
        };

        self.particles.push(particle);
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_active {
            self.spawn_accumulator += self.config.spawn_rate * delta_time;

            while self.spawn_accumulator >= 1.0 {
                self.spawn_particle();
                self.spawn_accumulator -= 1.0;
            }
        }

        for particle in &mut self.particles {
            particle.velocity += particle.acceleration * delta_time;
            particle.position += particle.velocity * delta_time;

            particle.age += delta_time;
        }

        self.particles.retain(|p| p.is_alive());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_config() -> EmitterConfig {
        EmitterConfig {
            spawn_rate: 0.0,
            lifetime_min: 1.0,
            lifetime_max: 1.0,
            speed_min: 0.0,
            speed_max: 0.0,
            angle_min: 0.0,
            angle_max: 0.0,
            gravity: Vec2::new(0.0, 0.0),
            color_start: Color::WHITE,
            color_end: Color::WHITE,
            size_start_min: 1.0,
            size_start_max: 1.0,
            size_end_min: 1.0,
            size_end_max: 1.0,
        }
    }

    #[test]
    fn emit_burst_spawns_exact_count() {
        let mut emitter = Emitter::new(Vec2::new(0.0, 0.0), EmitterShape::Point, base_config());
        emitter.is_active = false;
        emitter.emit_burst(25);
        assert_eq!(emitter.particles.len(), 25);
    }

    #[test]
    fn spawn_in_rect_stays_within_bounds() {
        let mut emitter = Emitter::new(
            Vec2::new(10.0, 20.0),
            EmitterShape::Rect {
                width: 8.0,
                height: 6.0,
            },
            base_config(),
        );
        emitter.is_active = false;

        for _ in 0..200 {
            emitter.spawn_particle();
        }

        let half_w = 4.0;
        let half_h = 3.0;
        for p in &emitter.particles {
            assert!(p.position.x >= 10.0 - half_w - 1e-5);
            assert!(p.position.x <= 10.0 + half_w + 1e-5);
            assert!(p.position.y >= 20.0 - half_h - 1e-5);
            assert!(p.position.y <= 20.0 + half_h + 1e-5);
        }
    }

    #[test]
    fn spawn_in_circle_stays_within_radius() {
        let radius = 12.0;
        let mut emitter = Emitter::new(
            Vec2::new(-5.0, 3.0),
            EmitterShape::Circle { radius },
            base_config(),
        );
        emitter.is_active = false;

        for _ in 0..400 {
            emitter.spawn_particle();
        }

        for p in &emitter.particles {
            let d = (p.position - emitter.position).len();
            assert!(d <= radius + 1e-4);
        }
    }

    #[test]
    fn continuous_update_spawns_expected_count() {
        let mut cfg = base_config();
        cfg.spawn_rate = 10.0;
        let mut emitter = Emitter::new(Vec2::new(0.0, 0.0), EmitterShape::Point, cfg);
        emitter.particles.clear();

        emitter.update(0.35);
        assert_eq!(emitter.particles.len(), 3);
        assert!((emitter.spawn_accumulator - 0.5).abs() < 1e-6);

        emitter.update(0.10);
        assert_eq!(emitter.particles.len(), 4);
        assert!((emitter.spawn_accumulator - 0.5).abs() < 1e-6);
    }
}
