extern crate sysfs_pwm;
use std::env;
use sysfs_pwm::{Pwm, Result};

fn blink_pwm(period_ns: u32, duty_cycle_ns: u32) -> Result<()> {
    let pwm = Pwm::new(0, 0)?;
    pwm.with_exported(|| {
        pwm.enable(true)?;
        pwm.set_period_ns(period_ns)?;
        pwm.set_duty_cycle_ns(duty_cycle_ns)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        pwm.enable(false)?;
        Ok(())
    })?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Period <period_ns>, Cycle  <duty_cycle_ns>");
        return;
    }

    let period_ns = args[1].parse::<u32>().unwrap();
    let duty_cycle_ns = args[2].parse::<u32>().unwrap();

    blink_pwm(period_ns, duty_cycle_ns).unwrap();
}
