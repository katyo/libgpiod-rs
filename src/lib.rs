#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use std::{
    fmt, fs,
    fs::{File, OpenOptions},
    io::Read,
    marker::PhantomData,
    ops::Deref,
    os::unix::{
        fs::{FileTypeExt, MetadataExt},
        io::{AsRawFd, FromRawFd},
    },
    path::{Path, PathBuf},
};

use gpiod_core::{invalid_input, major, minor, AsDevicePath, Internal, Result};

pub use gpiod_core::{
    Active, AsValues, AsValuesMut, Bias, BitId, ChipInfo, Direction, DirectionType, Drive, Edge,
    EdgeDetect, Event, Input, LineId, LineInfo, Masked, Options, Output, Values, ValuesInfo,
    MAX_BITS, MAX_VALUES,
};

/// The interface for accessing to the values of GPIO lines
///
/// Use [Chip::request_lines] with [Options::input] or [Options::output] to configure specific
/// GPIO lines for input or output.
pub struct Lines<Direction> {
    dir: PhantomData<Direction>,
    info: Internal<ValuesInfo>,
    // wrap file to call close on drop
    file: File,
}

impl<Direction> Deref for Lines<Direction> {
    type Target = ValuesInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl<Direction: DirectionType> Lines<Direction> {
    /// Get the value of GPIO lines
    ///
    /// The values can only be read if the lines have previously been requested as inputs
    /// or outputs using the [Chip::request_lines] method with [Options::input] or with
    /// [Options::output].
    pub fn get_values<T: AsValuesMut>(&self, mut values: T) -> Result<T> {
        self.info.get_values(self.file.as_raw_fd(), &mut values)?;
        Ok(values)
    }
}

impl Lines<Input> {
    /// Read GPIO events
    ///
    /// The values can only be read if the lines have previously been requested as inputs
    /// using the [Chip::request_lines] method with [Options::input].
    pub fn read_event(&mut self) -> Result<Event> {
        #[cfg(not(feature = "v2"))]
        {
            todo!();
        }

        #[cfg(feature = "v2")]
        {
            let mut event = gpiod_core::RawEvent::default();

            gpiod_core::check_size(self.file.read(event.as_mut())?, &event)?;

            event.as_event(self.info.index())
        }
    }
}

impl Iterator for Lines<Input> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_event())
    }
}

impl Lines<Output> {
    /// Set the value of GPIO lines
    ///
    /// The value can only be set if the lines have previously been requested as outputs
    /// using the [Chip::request_lines] with [Options::output].
    pub fn set_values<T: AsValues>(&self, values: T) -> Result<()> {
        self.info.set_values(self.file.as_raw_fd(), values)
    }
}

/// A Linux chardev GPIO chip interface
///
/// It can be used to get information about the chip and lines and
/// to request GPIO lines that can be used as inputs or outputs.
pub struct Chip {
    info: Internal<ChipInfo>,
    // wrap file to call close on drop
    file: File,
}

impl Deref for Chip {
    type Target = ChipInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl fmt::Display for Chip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(f)
    }
}

impl Chip {
    /// Create a new GPIO chip interface using path
    pub fn new(device: impl AsDevicePath) -> Result<Chip> {
        let path = device.as_device_path();

        Chip::check_device(&path)?;

        let file = OpenOptions::new().read(true).write(true).open(path)?;

        Ok(Chip {
            info: Internal::<ChipInfo>::from_fd(file.as_raw_fd())?,
            file,
        })
    }

    /// List all found chips
    pub fn list_devices() -> Result<Vec<PathBuf>> {
        Ok(fs::read_dir("/dev")?
            .filter_map(Result::ok)
            .map(|ent| ent.path())
            .filter(|path| Self::check_device(path).is_ok())
            .collect())
    }

    fn check_device(path: &Path) -> Result<()> {
        let metadata = fs::symlink_metadata(path)?;

        /* Is it a character device? */
        if !metadata.file_type().is_char_device() {
            return Err(invalid_input("File is not character device"));
        }

        let rdev = metadata.rdev();

        /* Is the device associated with the GPIO subsystem? */
        if fs::canonicalize(format!(
            "/sys/dev/char/{}:{}/subsystem",
            major(rdev),
            minor(rdev)
        ))? != Path::new("/sys/bus/gpio")
        {
            return Err(invalid_input("Character device is not a GPIO"));
        }

        Ok(())
    }

    /// Request the info of a specific GPIO line.
    pub fn line_info(&self, line: LineId) -> Result<LineInfo> {
        self.info.line_info(self.file.as_raw_fd(), line)
    }

    /// Request the GPIO chip to configure the lines passed as argument as inputs or outputs
    ///
    /// Calling this operation is a precondition to being able to set the state of the GPIO lines.
    /// All the lines passed in one request must share the configured options such as active state,
    /// edge detect, GPIO bias, output drive and consumer string.
    pub fn request_lines<Direction: DirectionType>(
        &self,
        options: Options<Direction, impl AsRef<[LineId]>, impl AsRef<str>>,
    ) -> Result<Lines<Direction>> {
        let (info, fd) = self.info.request_lines(self.file.as_raw_fd(), options)?;

        let file = unsafe { File::from_raw_fd(fd) };

        Ok(Lines {
            dir: PhantomData,
            info,
            file,
        })
    }
}
