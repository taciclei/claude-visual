use gpui::*;

/// Confetti particle shape
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfettiShape {
    #[default]
    Square,
    Circle,
    Rectangle,
    Star,
    Heart,
    Triangle,
}

/// Confetti animation style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfettiStyle {
    #[default]
    Burst,
    Rain,
    Cannon,
    Fireworks,
    Spray,
}

/// Confetti intensity level
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfettiIntensity {
    Low,
    #[default]
    Medium,
    High,
    Extreme,
}

impl ConfettiIntensity {
    pub(crate) fn particle_count(&self) -> usize {
        match self {
            Self::Low => 25,
            Self::Medium => 50,
            Self::High => 100,
            Self::Extreme => 200,
        }
    }
}

/// Single confetti particle
#[derive(Debug, Clone)]
pub struct ConfettiParticle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rotation: f32,
    pub(crate) scale: f32,
    pub(crate) color: gpui::Hsla,
    pub(crate) shape: ConfettiShape,
    pub(crate) velocity_x: f32,
    pub(crate) velocity_y: f32,
    pub(crate) opacity: f32,
}

impl Default for ConfettiParticle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
            color: rgb(0xff0000).into(),
            shape: ConfettiShape::default(),
            velocity_x: 0.0,
            velocity_y: 0.0,
            opacity: 1.0,
        }
    }
}

impl ConfettiParticle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn shape(mut self, shape: ConfettiShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn velocity(mut self, vx: f32, vy: f32) -> Self {
        self.velocity_x = vx;
        self.velocity_y = vy;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

/// Balloon size preset
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BalloonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl BalloonSize {
    pub(crate) fn dimensions(&self) -> (f32, f32) {
        match self {
            Self::Sm => (30.0, 36.0),
            Self::Md => (50.0, 60.0),
            Self::Lg => (70.0, 84.0),
        }
    }
}

/// Direction the popper fires
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopperDirection {
    #[default]
    UpRight,
    UpLeft,
    Up,
    Right,
    Left,
}
