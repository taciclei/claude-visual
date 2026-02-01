use super::types::{ConfettiIntensity, ConfettiParticle, ConfettiShape, ConfettiStyle};
use gpui::prelude::*;
use gpui::*;

/// Confetti burst component
#[derive(IntoElement)]
pub struct Confetti {
    id: ElementId,
    style: ConfettiStyle,
    intensity: ConfettiIntensity,
    particles: Vec<ConfettiParticle>,
    colors: Vec<gpui::Hsla>,
    shapes: Vec<ConfettiShape>,
    origin_x: f32,
    origin_y: f32,
    spread: f32,
    duration: u32,
    active: bool,
    width: f32,
    height: f32,
}

impl Confetti {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: ConfettiStyle::default(),
            intensity: ConfettiIntensity::default(),
            particles: Vec::new(),
            colors: vec![
                rgb(0xef4444).into(), // Red
                rgb(0xf97316).into(), // Orange
                rgb(0xeab308).into(), // Yellow
                rgb(0x22c55e).into(), // Green
                rgb(0x3b82f6).into(), // Blue
                rgb(0xa855f7).into(), // Purple
                rgb(0xec4899).into(), // Pink
            ],
            shapes: vec![
                ConfettiShape::Square,
                ConfettiShape::Circle,
                ConfettiShape::Rectangle,
            ],
            origin_x: 0.5,
            origin_y: 0.5,
            spread: 180.0,
            duration: 3000,
            active: false,
            width: 400.0,
            height: 400.0,
        }
    }

    pub fn style(mut self, style: ConfettiStyle) -> Self {
        self.style = style;
        self
    }

    pub fn intensity(mut self, intensity: ConfettiIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn particles(mut self, particles: Vec<ConfettiParticle>) -> Self {
        self.particles = particles;
        self
    }

    pub fn colors(mut self, colors: Vec<gpui::Hsla>) -> Self {
        self.colors = colors;
        self
    }

    pub fn shapes(mut self, shapes: Vec<ConfettiShape>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn origin(mut self, x: f32, y: f32) -> Self {
        self.origin_x = x;
        self.origin_y = y;
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    fn render_particle(&self, particle: &ConfettiParticle) -> impl IntoElement {
        let size = 8.0 * particle.scale;

        let base = div()
            .absolute()
            .left(px(particle.x))
            .top(px(particle.y))
            .opacity(particle.opacity);

        match particle.shape {
            ConfettiShape::Square => base.size(px(size)).bg(particle.color),
            ConfettiShape::Circle => base.size(px(size)).rounded_full().bg(particle.color),
            ConfettiShape::Rectangle => base.w(px(size * 2.0)).h(px(size)).bg(particle.color),
            ConfettiShape::Star | ConfettiShape::Heart | ConfettiShape::Triangle => {
                // Simplified to colored square for these shapes
                base.size(px(size)).bg(particle.color)
            }
        }
    }
}

impl RenderOnce for Confetti {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.active && self.particles.is_empty() {
            return div().id(self.id);
        }

        let id = self.id.clone();
        let width = self.width;
        let height = self.height;
        let particles: Vec<_> = self
            .particles
            .iter()
            .map(|p| self.render_particle(p))
            .collect();

        div()
            .id(id)
            .absolute()
            .inset_0()
            .w(px(width))
            .h(px(height))
            .overflow_hidden()
            .children(particles)
    }
}
