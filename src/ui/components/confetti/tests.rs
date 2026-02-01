use super::*;

#[test]
fn test_confetti_creation() {
    let confetti = Confetti::new("c-1")
        .style(ConfettiStyle::Burst)
        .intensity(ConfettiIntensity::High)
        .active(true);
    assert_eq!(confetti.style, ConfettiStyle::Burst);
    assert!(confetti.active);
}

#[test]
fn test_confetti_particle() {
    let particle = ConfettiParticle::new(100.0, 50.0)
        .color(rgb(0xff0000).into())
        .shape(ConfettiShape::Circle)
        .velocity(5.0, -10.0);
    assert_eq!(particle.x, 100.0);
    assert_eq!(particle.y, 50.0);
    assert_eq!(particle.shape, ConfettiShape::Circle);
}

#[test]
fn test_emoji_burst() {
    let burst = EmojiBurst::new("eb-1")
        .emojis(vec!["🎉".to_string(), "🎊".to_string()])
        .count(10)
        .active(true);
    assert!(burst.active);
    assert_eq!(burst.emojis.len(), 2);
}

#[test]
fn test_sparkle_effect() {
    let sparkle = SparkleEffect::new("se-1")
        .sparkle_count(20)
        .color(rgb(0xffd700).into())
        .active(true);
    assert_eq!(sparkle.sparkle_count, 20);
}

#[test]
fn test_firework() {
    let firework = Firework::new("fw-1")
        .position(200.0, 100.0)
        .radius(150.0)
        .particle_count(40);
    assert_eq!(firework.x, 200.0);
    assert_eq!(firework.y, 100.0);
    assert_eq!(firework.radius, 150.0);
}

#[test]
fn test_balloon() {
    let balloon = Balloon::new("b-1")
        .color(rgb(0x3b82f6).into())
        .size(BalloonSize::Lg)
        .position(50.0, 100.0);
    assert_eq!(balloon.size, BalloonSize::Lg);
    assert_eq!(balloon.x, 50.0);
}

#[test]
fn test_party_popper() {
    let popper = PartyPopper::new("pp-1")
        .direction(PopperDirection::UpLeft)
        .active(true);
    assert_eq!(popper.direction, PopperDirection::UpLeft);
    assert!(popper.active);
}

#[test]
fn test_intensity_particle_count() {
    assert!(ConfettiIntensity::Low.particle_count() < ConfettiIntensity::Medium.particle_count());
    assert!(ConfettiIntensity::Medium.particle_count() < ConfettiIntensity::High.particle_count());
    assert!(ConfettiIntensity::High.particle_count() < ConfettiIntensity::Extreme.particle_count());
}
