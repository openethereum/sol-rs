/// conversion of the various ether units (shannon, gwei, ...) to wei

use types::U256;

lazy_static! {
    pub static ref TETHER: U256 = U256::from(10).pow(30.into());
    pub static ref GETHER: U256 = U256::from(10).pow(27.into());
    pub static ref METHER: U256 = U256::from(10).pow(24.into());
    pub static ref KETHER: U256 = U256::from(10).pow(21.into());
    pub static ref ETHER: U256 = U256::from(10).pow(18.into());
    pub static ref FINNEY: U256 = U256::from(10).pow(15.into());
    pub static ref SZABO: U256 = U256::from(10).pow(12.into());
    pub static ref GWEI: U256 = U256::from(10).pow(9.into());
    pub static ref MWEI: U256 = U256::from(10).pow(6.into());
    pub static ref KWEI: U256 = U256::from(10).pow(3.into());
}

/// returns `value` converted from ether to wei
pub fn from_tether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *TETHER
}

/// returns `value` converted from gether to wei
pub fn from_gether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GETHER
}

/// returns `value` converted from mether to wei
pub fn from_mether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *METHER
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_einstein<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_grand<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_kether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

/// returns `value` converted from ether to wei
pub fn from_ether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *ETHER
}

/// returns `value` converted from finney to wei
pub fn from_finney<T: Into<U256>>(value: T) -> U256 {
    value.into() * *FINNEY
}

/// returns `value` converted from szabo to wei
pub fn from_szabo<T: Into<U256>>(value: T) -> U256 {
    value.into() * *SZABO
}

/// returns `value` converted from gwei/shannon to wei
pub fn from_gwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GWEI
}

/// returns `value` converted from gwei/shannon to wei
pub fn from_shannon<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GWEI
}

/// returns `value` converted from mwei/babbage to wei
pub fn from_mwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *MWEI
}

/// returns `value` converted from mwei/babbage to wei
pub fn from_babbage<T: Into<U256>>(value: T) -> U256 {
    value.into() * *MWEI
}

/// returns `value` converted from kwei/ada to wei
pub fn from_kwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KWEI
}

/// returns `value` converted from kwei/ada to wei
pub fn from_ada<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KWEI
}

/// returns `value` converted from wei to wei.
/// unit function
pub fn from_wei<T: Into<U256>>(value: T) -> U256 {
    value.into()
}
