use std::{ops::Range, time::Duration};

pub use bevy::prelude::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scale_up);
    }
}

#[derive(Component)]
pub struct AnimateScale {
    timer: Timer,
    ease: Ease,
    range: Range<f32>,
}

impl AnimateScale {
    pub fn new(duration: Duration, ease: Ease, range: Range<f32>) -> Self {
        Self {
            timer: Timer::new(duration, false),
            ease,
            range,
        }
    }
}

pub fn scale_up(time: Res<Time>, mut query: Query<(&mut Transform, &mut AnimateScale)>) {
    for (mut transform, mut animate_scale) in query.iter_mut() {
        animate_scale.timer.tick(time.delta());
        let amount = animate_scale.ease.ease(animate_scale.timer.percent());
        let scale = animate_scale.range.start
            + ((animate_scale.range.end - animate_scale.range.start) * amount);
        transform.scale = Vec3::splat(scale);
    }
}

pub enum Ease {
    InOutCirc,
    OutBack,
}

impl Ease {
    pub fn ease(&self, x: f32) -> f32 {
        match self {
            Ease::InOutCirc => {
                if x < 0.5 {
                    (1. - (1. - (2. * x).powf(2.)).sqrt()) / 2.
                } else {
                    ((1. - (-2. * x + 2.).powf(2.)).sqrt() + 1.) / 2.
                }
            }
            Ease::OutBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;

                1. + C3 * (x - 1.).powf(3.) + C1 * (x - 1.).powf(2.)
            }
        }
    }
}
