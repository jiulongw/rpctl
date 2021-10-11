// This module uses deprecated GPIO interface via sysfs, for simplicity.
// A better approach is to use nix crate with ioctl to interact with /dev/gpiochip*
// See https://embeddedbits.org/new-linux-kernel-gpio-user-space-interface/ for details.
use std::{fs, io};

const OS_ERR_RESOURCE_BUSY: i32 = 16;

pub struct OutputPin {
    pin_number: u32,
}

impl OutputPin {
    pub fn new(pin_number: u32) -> io::Result<OutputPin> {
        let err = fs::write("/sys/class/gpio/export", pin_number.to_string()).err();
        match err {
            Some(e) => {
                if e.raw_os_error() != Some(OS_ERR_RESOURCE_BUSY) {
                    return Err(e);
                }
            }
            _ => {}
        }

        let direction = format!("/sys/class/gpio/gpio{}/direction", pin_number);
        fs::write(direction, "out")?;

        Ok(OutputPin { pin_number })
    }

    pub fn set_value(&self, value: Value) -> io::Result<()> {
        let value_file = format!("/sys/class/gpio/gpio{}/value", self.pin_number);
        fs::write(value_file, value.value())?;
        Ok(())
    }
}

pub enum Value {
    Low = 0,
    High = 1,
}

impl Value {
    fn value(self) -> String {
        (self as i32).to_string()
    }
}
