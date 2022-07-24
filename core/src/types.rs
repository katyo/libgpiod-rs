use crate::{utils::*, Error, Result, Time, MAX_BITS};
use std::{fmt, str};

/// Line offset
pub type LineId = u32;

/// Bit offset
pub type BitId = u8;

/// Line offset to bit offset mapping
#[derive(Debug, Clone)]
pub struct LineMap {
    map: Vec<BitId>,
}

impl LineMap {
    const NOT_LINE: BitId = MAX_BITS;

    /// Create line map
    pub fn new(lines: &[LineId]) -> Self {
        let mut map: Vec<BitId> = (0..=lines.iter().max().copied().unwrap_or(0))
            .map(|_| Self::NOT_LINE)
            .collect();
        for i in 0..lines.len() {
            map[lines[i] as usize] = i as _;
        }
        Self { map }
    }

    /// Get bit position by line offset
    pub fn get(&self, line: LineId) -> Result<BitId> {
        let line = line as usize;
        if line < self.map.len() {
            let val = self.map[line];
            if val != Self::NOT_LINE {
                return Ok(val as _);
            }
        }
        Err(invalid_data("Unknown line offset"))
    }
}

/// The information of a specific GPIO line
#[derive(Debug, Clone)]
pub struct LineInfo {
    /// GPIO line direction
    pub direction: Direction,

    /// GPIO line active state
    pub active: Active,

    /// GPIO line edge detection
    pub edge: EdgeDetect,

    /// GPIO line usage status
    ///
    /// `true` means that kernel uses this line for some purposes.
    pub used: bool,

    /// GPIO line input bias
    pub bias: Bias,

    /// GPIO line output drive mode
    pub drive: Drive,

    /// GPIO line name
    pub name: String,

    /// GPIO line consumer name
    pub consumer: String,
}

impl fmt::Display for LineInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "\t unnamed")?;
        } else {
            write!(f, "\t {:?}", self.name)?;
        }
        if self.consumer.is_empty() {
            write!(f, "\t unused")?;
        } else {
            write!(f, "\t {:?}", self.consumer)?;
        }
        write!(f, "\t {}", self.direction)?;
        write!(f, "\t active-{}", self.active)?;
        if !matches!(self.edge, EdgeDetect::Disable) {
            write!(f, "\t {}-edge", self.edge)?;
        }
        if !matches!(self.bias, Bias::Disable) {
            write!(f, "\t {}", self.edge)?;
        }
        if !matches!(self.drive, Drive::PushPull) {
            write!(f, "\t {}", self.drive)?;
        }
        if self.used {
            write!(f, "\t [used]")?;
        }
        Ok(())
    }
}

/// Direction of a GPIO line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ArgEnum))]
#[repr(u8)]
pub enum Direction {
    /// Line acts as input (default)
    #[cfg_attr(feature = "clap", clap(aliases = ["i", "in"]))]
    Input,
    /// Line acts as output
    #[cfg_attr(feature = "clap", clap(aliases = ["o", "out"]))]
    Output,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Input
    }
}

impl AsRef<str> for Direction {
    fn as_ref(&self) -> &str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "i" | "in" | "input" => Self::Input,
            "o" | "out" | "output" => Self::Output,
            _ => return Err(invalid_input("Not recognized direction")),
        })
    }
}

/// Active state condition of a line
///
/// If active state of line is **high** then physical and logical levels is same.
/// Otherwise if it is **low** then physical level will be inverted from logical.
///
/// Also this may be treated as polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ArgEnum))]
#[repr(u8)]
pub enum Active {
    /// Active level is low
    #[cfg_attr(feature = "clap", clap(aliases = ["l", "lo"]))]
    Low,
    /// Active level is high (default)
    #[cfg_attr(feature = "clap", clap(aliases = ["h", "hi"]))]
    High,
}

impl Default for Active {
    fn default() -> Self {
        Self::High
    }
}

impl AsRef<str> for Active {
    fn as_ref(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::High => "high",
        }
    }
}

impl fmt::Display for Active {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for Active {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "l" | "lo" | "low" | "active-low" => Self::Low,
            "h" | "hi" | "high" | "active-high" => Self::High,
            _ => return Err(invalid_input("Not recognized active state")),
        })
    }
}

/// Signal edge or level transition of a GPIO line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Edge {
    /// Rising edge detected
    Rising,
    /// Falling edge detected
    Falling,
}

impl AsRef<str> for Edge {
    fn as_ref(&self) -> &str {
        match self {
            Self::Rising => "rising",
            Self::Falling => "falling",
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "r" | "rise" | "rising" => Self::Rising,
            "f" | "fall" | "falling" => Self::Falling,
            _ => return Err(invalid_input("Not recognized edge")),
        })
    }
}

/// Signal edge detection event
#[derive(Debug, Clone, Copy)]
pub struct Event {
    /// GPIO line where edge detected
    pub line: BitId,
    /// Detected edge or level transition
    pub edge: Edge,
    /// Time when edge actually detected
    pub time: Time,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        '#'.fmt(f)?;
        self.line.fmt(f)?;
        ' '.fmt(f)?;
        self.edge.fmt(f)?;
        ' '.fmt(f)?;
        self.time.as_nanos().fmt(f)
    }
}

/// Edge detection setting for GPIO line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ArgEnum))]
#[repr(u8)]
pub enum EdgeDetect {
    /// Detection disabled (default)
    #[cfg_attr(feature = "clap", clap(aliases = ["d", "dis"]))]
    Disable,
    /// Detect rising edge only
    #[cfg_attr(feature = "clap", clap(aliases = ["r", "rise"]))]
    Rising,
    /// Detect falling edge only
    #[cfg_attr(feature = "clap", clap(aliases = ["f", "fall"]))]
    Falling,
    /// Detect both rising and falling edges
    #[cfg_attr(feature = "clap", clap(aliases = ["b"]))]
    Both,
}

impl Default for EdgeDetect {
    fn default() -> Self {
        Self::Disable
    }
}

impl AsRef<str> for EdgeDetect {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disable => "disable",
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::Both => "both",
        }
    }
}

impl fmt::Display for EdgeDetect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for EdgeDetect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "d" | "dis" | "disable" => Self::Disable,
            "r" | "rise" | "rising" => Self::Rising,
            "f" | "fall" | "falling" => Self::Falling,
            "b" | "both" | "rise-fall" | "rising-falling" => Self::Both,
            _ => return Err(invalid_input("Not recognized edge-detect")),
        })
    }
}

/// Input bias of a GPIO line
///
/// Sometimes GPIO lines shall be pulled to up (power rail) or down (ground)
/// through resistor to avoid floating level on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ArgEnum))]
#[repr(u8)]
pub enum Bias {
    /// Disabled bias (default)
    #[cfg_attr(feature = "clap", clap(aliases = ["d", "dis"]))]
    Disable,
    /// Pull line up
    #[cfg_attr(feature = "clap", clap(aliases = ["pu"]))]
    PullUp,
    /// Pull line down
    #[cfg_attr(feature = "clap", clap(aliases = ["pd"]))]
    PullDown,
}

impl Default for Bias {
    fn default() -> Self {
        Self::Disable
    }
}

impl AsRef<str> for Bias {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disable => "disable",
            Self::PullUp => "pull-up",
            Self::PullDown => "pull-down",
        }
    }
}

impl fmt::Display for Bias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for Bias {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "d" | "dis" | "disable" => Self::Disable,
            "pu" | "pull-up" => Self::PullUp,
            "pd" | "pull-down" => Self::PullUp,
            _ => return Err(invalid_input("Not recognized input bias")),
        })
    }
}

/// Output drive mode of a GPIO line
///
/// Usually GPIO lines configured as push-pull but sometimes it required to drive via open drain or source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ArgEnum))]
#[repr(u8)]
pub enum Drive {
    /// Drive push-pull (default)
    #[cfg_attr(feature = "clap", clap(aliases = ["pp"]))]
    PushPull,
    /// Drive with open-drain
    #[cfg_attr(feature = "clap", clap(aliases = ["od"]))]
    OpenDrain,
    /// Drive with open-source
    #[cfg_attr(feature = "clap", clap(aliases = ["os"]))]
    OpenSource,
}

impl Default for Drive {
    fn default() -> Self {
        Self::PushPull
    }
}

impl AsRef<str> for Drive {
    fn as_ref(&self) -> &str {
        match self {
            Self::PushPull => "push-pull",
            Self::OpenDrain => "open-drain",
            Self::OpenSource => "open-source",
        }
    }
}

impl fmt::Display for Drive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl str::FromStr for Drive {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "pp" | "push-pull" => Self::PushPull,
            "od" | "open-drain" => Self::OpenDrain,
            "os" | "open-source" => Self::OpenSource,
            _ => return Err(invalid_input("Not recognized output drive")),
        })
    }
}
