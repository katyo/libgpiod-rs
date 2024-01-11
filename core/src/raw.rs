#[cfg(not(feature = "v2"))]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;

const GPIO_MAX_NAME_SIZE: usize = 32;
const GPIO_MAGIC: u8 = 0xB4;

// All the structs used for ioctl must be represented in C otherwise weird memory mappings happen.
//
// The implementations provided inside this module are also a copy of gpio.h which is normally
// used to access the gpio in linux. If that header file changes, this module will likely be broken
// due to mismatched data types or GPIO command codes.
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct GpioChipInfo {
    pub name: [u8; GPIO_MAX_NAME_SIZE],
    pub label: [u8; GPIO_MAX_NAME_SIZE],
    pub lines: u32,
}

nix::ioctl_read!(gpio_get_chip_info, GPIO_MAGIC, 0x01, GpioChipInfo);
nix::ioctl_read!(gpio_line_info_unwatch, GPIO_MAGIC, 0x0c, u32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sizes() {
        use core::mem::size_of;

        assert_eq!(size_of::<GpioChipInfo>(), 68);
    }
}
