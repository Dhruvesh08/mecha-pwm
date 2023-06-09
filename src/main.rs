extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// PIN: GPIO4 (P9_33)
const LED_PWM_CHIP: u32 = 3;
const LED_PWM_PIN: u32 = 1;

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
    let pwm = Pwm::new(LED_PWM_CHIP, LED_PWM_PIN).unwrap(); // Number depends on chip, etc.
    pwm.with_exported(|| {
        blink_led(&pwm, 1_000_000, 500_000, 5) // Blink 5 times with 1-second period and 50% duty cycle
    })
    .unwrap();

    pwm.unexport().unwrap();
}
