use nix::fcntl::{open, OFlag};
use nix::unistd::close;
use nix::ioctl_readwrite;

use std::thread;
use std::time::Duration;
use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::{ self, Read, Write };

// from i8kutils
// #define I8K_SET_FAN		_IOWR('i', 0x87, size_t)
const I8K_FAN_MAGIC: u8 = b'i';
const I8K_FAN_SET: u8 = 0x87;
ioctl_readwrite!(i8k_fan_set, I8K_FAN_MAGIC, I8K_FAN_SET, u64);

fn pass_char(c: char) -> bool {
    match c {
        '0'..='9' | '-' => true,
        _               => false,
    }
}

fn main() {
    let i8k_fd = open(
        "/proc/i8k",
        OFlag::O_RDWR,
        nix::sys::stat::Mode::empty(),
        ).unwrap();

    let cpu_temp_path = format!("/sys/class/hwmon/hwmon6/temp1_input"); //?temp2 hardcode path
    let mut path_v = PathBuf::new();
    path_v.push(Path::new(&cpu_temp_path));

    let mut raw_bytes: [u8; 6] = [0; 6];

    let mut fan_control_data: u64 = (1 as u64) << 32 | 0 as u64;
    // upper u32: {fan_speed [0, 1, 2]}
    // down u32: {0: right_fan, 1: left_fan}
    unsafe {
        let _ = i8k_fan_set(i8k_fd, &mut fan_control_data);
    }
    let mut pre_fan_status: bool = true;
    println!("start dlfcd");
    loop {
        let mut file = File::open(&path_v).unwrap();
        file.read(&mut raw_bytes).unwrap();
        let val_str = std::str::from_utf8(&raw_bytes).unwrap()
            .chars().filter(|c| pass_char(*c))
            .collect::<String>();
        let raw_value = val_str.parse::<u32>().unwrap();
        //println!("temp: {}", raw_value);

        if raw_value > 60000 {
            if pre_fan_status == false {
                //println!("fan speed to mid");
                fan_control_data = (1 as u64) << 32 | 0 as u64; // right mid
                unsafe {
                    let _ = i8k_fan_set(i8k_fd, &mut fan_control_data);
                }
                pre_fan_status = true;
            }
        } else {
            if pre_fan_status == true {
                //println!("fan stop");
                fan_control_data = (0 as u64) << 32 | 0 as u64; // right off
                unsafe {
                    let _ = i8k_fan_set(i8k_fd, &mut fan_control_data);
                }
                pre_fan_status = false;
            }
        }

        thread::sleep(Duration::from_secs(5));
    }

    /*
    let _ = close(i8k_fd);
    println!("exit dlfcd");
    */
}
