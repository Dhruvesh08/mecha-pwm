extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// Specify the PWM chip and pin for the servo motor
const SERVO_PWM_CHIP: u32 = 0;   // Update with the actual chip number
const SERVO_PWM_PIN: u32 = 0;    // Update with the actual pin number

fn servo_increase_to_max(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = 1.0 / (duration_ms as f32 / update_period_ms as f32);
    let mut duty_cycle = 0.0;
    let period_ns: u32 = pwm.get_period_ns()?;
    while duty_cycle < 1.0 {
        pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32)?;
        duty_cycle += step;
    }
    pwm.set_duty_cycle_ns(period_ns)
}

fn servo_decrease_to_minimum(pwm: &Pwm, duration_ms: u32, update_period_ms: u32) -> Result<()> {
    let step: f32 = 1.0 / (duration_ms as f32 / update_period_ms as f32);
    let mut duty_cycle = 1.0;
    let period_ns: u32 = pwm.get_period_ns()?;
    while duty_cycle > 0.0 {
        pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32)?;
        duty_cycle -= step;
    }
    pwm.set_duty_cycle_ns(0)
}

fn main() {
    let pwm = Pwm::new(SERVO_PWM_CHIP, SERVO_PWM_PIN).unwrap();
    pwm.with_exported(|| {
        pwm.enable(true).unwrap();
        pwm.set_period_ns(20_000_000).unwrap();   // 20ms period (50 Hz)
        loop {
            servo_increase_to_max(&pwm, 1000, 20).unwrap();
            servo_decrease_to_minimum(&pwm, 1000, 20).unwrap();
        }
    })
    .unwrap();
}
