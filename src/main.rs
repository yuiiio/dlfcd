//use nix::sys::ioctl;
use nix::fcntl::{open, OFlag};
use nix::unistd::close;
use nix::ioctl_readwrite;

// from i8kutils
// #define I8K_SET_FAN		_IOWR('i', 0x87, size_t)
const I8K_FAN_MAGIC: u8 = b'i';
const I8K_FAN_SET: u8 = 0x87;
ioctl_readwrite!(i8k_fan_set, I8K_FAN_MAGIC, I8K_FAN_SET, u64);

fn main() {
    let i8k_fd = open(
        "/proc/i8k",
        OFlag::O_RDWR,
        nix::sys::stat::Mode::empty(),
        ).unwrap();

    unsafe {
        let mut data: u64 = (1 as u64) << 32 | 0 as u64;
        // upper u32: {fan_speed [0, 1, 2]}
        // down u32: {0: right_fan, 1: left_fan}
        let _ = i8k_fan_set(i8k_fd, &mut data);
    }

    let _ = close(i8k_fd);
    println!("Hello, world!");
}
