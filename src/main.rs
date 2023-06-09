extern crate sysfs_pwm;
use std::env;
use sysfs_pwm::{Pwm, Result};

// PIN: GPIO4 (P9_33)
const LED_PWM_CHIP: u32 = 0;
const LED_PWM_PIN: u32 = 0;

fn blink_led(pwm: &Pwm, period_ns: u32, duty_cycle_ns: u32, num_cycles: u32) -> Result<()> {
    pwm.set_period_ns(period_ns)?;
    pwm.set_duty_cycle_ns(duty_cycle_ns)?;
    pwm.enable(true)?;

    for _ in 0..num_cycles {
        std::thread::sleep(std::time::Duration::from_secs(1));
        pwm.enable(false)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        pwm.enable(true)?;
    }

    pwm.enable(false)?;
    pwm.set_duty_cycle_ns(0)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: ./blink_led <period_ns> <duty_cycle_ns> <num_cycles>");
        return;
    }

    let period_ns = args[1].parse::<u32>().unwrap();
    let duty_cycle_ns = args[2].parse::<u32>().unwrap();
    let num_cycles = args[3].parse::<u32>().unwrap();

    let pwm = Pwm::new(LED_PWM_CHIP, LED_PWM_PIN).unwrap(); // Number depends on chip, etc.
    pwm.with_exported(|| {
        blink_led(&pwm, period_ns, duty_cycle_ns, num_cycles)
    })
    .unwrap();

    pwm.unexport().unwrap();
}
