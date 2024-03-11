use nix::sys::ioctl;
use nix::ioctl_write_buf;

// from i8kutils
// #define I8K_SET_FAN		_IOWR('i', 0x87, size_t)
const I8K_FAN_MAGIC: u8 = b'i';
const I8K_FAN_SET: u8 = 0x87;

fn main() {

    let data: [u32; 2] = [0, 1]; // right, fan_speed[0, 1, 2]
    ioctl_write_buf!(i8k_fd, I8K_FAN_MAGIC, I8K_FAN_SET, &[u32; 2]);
    println!("Hello, world!");
}
