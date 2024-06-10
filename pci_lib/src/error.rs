use core::fmt;

pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub struct PciError {
    code: Code,
    file: &'static str,
    line: u32,
}

impl PciError {
    pub fn new(code: Code, file: &'static str, line: u32) -> Self {
        Self { code, file , line }
    }
}

impl fmt::Display for PciError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, file = {}, line = {}", self.code, self.file, self.line)
    }
}

#[derive(Debug)]
pub enum Code {
    // General Errors
    Unknown,
     InvalidVendorId,
    InvalidDeviceId,
    DeviceNotFound,
    InvalidClassCode,
    ResourceAllocationFailed,
    
    // Configuration errors
    BarReadFailed,
    BarWriteFailed,
    MsiConfigurationFailed,
    InterruptSetupFailed,

    // Device-specific errors
    DeviceInitializationFailed,
    DeviceResetFailed,
    DeviceNotReady,
    UnsupportedDevice,

    // Memory and IO errors
    MmioAccessFailed,
    IoAccessFailed,

    // Port and controller errors
    PortConfigurationFailed,
    PortNotConnected,
    ControllerInitializationFailed,
    ControllerRunFailed,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[macro_export]
macro_rules! make_error {
    ($x:expr) => {{
        PciError::new(($x), file!(), line!())
    }};
}