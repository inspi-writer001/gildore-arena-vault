#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GildoreVaultError {
    Unauthorized = 0,
}

impl GildoreVaultError {
    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            0 => Some(Self::Unauthorized),
            _ => None,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::Unauthorized => "Unauthorized",
        }
    }
}
