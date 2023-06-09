extern crate sysfs_pwm;
use std::env;
use sysfs_pwm::{Pwm, Result};

// PIN: EHRPWM0A (P9_22)
const SERVO_PWM_CHIP: u32 = 0;
const SERVO_PWM_PIN: u32 = 0;

fn move_servo(pwm: &Pwm, period_ns: u32, duty_cycle_ns: u32) -> Result<()> {
    pwm.set_period_ns(period_ns)?;
    pwm.set_duty_cycle_ns(duty_cycle_ns)?;
    pwm.enable(true)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    pwm.enable(false)?;
    pwm.set_duty_cycle_ns(0)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <period_ns> <duty_cycle_ns>", args[0]);
        return;
    }

    let period_ns: u32 = args[1].parse().unwrap_or(20_000);
    let duty_cycle_ns: u32 = args[2].parse().unwrap_or(1_000);

    let pwm = Pwm::new(SERVO_PWM_CHIP, SERVO_PWM_PIN).unwrap(); // Number depends on chip, etc.
    pwm.with_exported(|| {
        move_servo(&pwm, period_ns, duty_cycle_ns)
    })
    .unwrap();

    pwm.unexport().unwrap();
}
