use core::fmt;

#[derive(Debug)]
pub struct Error {
    code: Code,
    file: &'static str,
    line: u32,
}

impl Error {
    pub fn new(code: Code, file: &'static str, line: u32) -> Self {
        Self { code, file , line }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, file = {}, line = {}", self.code, self.file, self.line)
    }
}

#[derive(Debug)]
pub enum Code {
    Full,
    Empty,
    NoEnoughMemory,
    IndexOutOfRange,
    HostControllerNotHalted,
    InvalidSlotID,
    PortNotConnected,
    InvalidEndpointNumber,
    TransferRingNotSet,
    AlreadyAllocated,
    NotImplemented,
    InvalidDescriptor,
    BufferTooSmall,
    UnknownDevice,
    NoCorrespondingSetupStage,
    TransferFailed,
    InvalidPhase,
    UnknownXHCISpeedID,
    NoWaiter,
    LastOfCode,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[macro_export]
macro_rules! make_error {
    ($x:expr) => {{
        Error::new(($x), file!(), line!())
    }};
}