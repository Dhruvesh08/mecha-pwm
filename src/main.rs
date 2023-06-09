extern crate sysfs_pwm;
use std::env;
use sysfs_pwm::{Pwm, Result};

// PIN: EHRPWM0A (P9_22)
const SERVO_PWM_CHIP: u32 = 0;
const SERVO_PWM_PIN: u32 = 0;

fn move_servo(pwm: &Pwm, min_position: f32, max_position: f32) -> Result<()> {
    let period_ns: u32 = pwm.get_period_ns()?;
    let min_duty_cycle_ns = (min_position * period_ns as f32) as u32;
    let max_duty_cycle_ns = (max_position * period_ns as f32) as u32;

    pwm.set_duty_cycle_ns(min_duty_cycle_ns)?;
    pwm.enable(true)?;

    // Move to the maximum position
    pwm.set_duty_cycle_ns(max_duty_cycle_ns)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Move to the minimum position
    pwm.set_duty_cycle_ns(min_duty_cycle_ns)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    pwm.enable(false)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <min_position> <max_position>", args[0]);
        return;
    }

    let min_position: f32 = args[1].parse().unwrap_or(0.0);
    let max_position: f32 = args[2].parse().unwrap_or(1.0);

    let pwm = Pwm::new(SERVO_PWM_CHIP, SERVO_PWM_PIN).unwrap(); // Number depends on chip, etc.
    pwm.with_exported(|| {
        move_servo(&pwm, min_position, max_position)
    })
    .unwrap();
}
