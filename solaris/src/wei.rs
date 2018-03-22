/// conversion of the various ether units (shannon, gwei, ...) to wei

use ethereum_types::U256;

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

/// returns `value` converted from tether to wei
pub fn from_tether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *TETHER
}

#[test]
fn test_from_tether() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000000000000").unwrap(),
        from_tether(2));
}

/// returns `value` converted from gether to wei
pub fn from_gether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GETHER
}

#[test]
fn test_from_gether() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000000000").unwrap(),
        from_gether(2));
}

/// returns `value` converted from mether to wei
pub fn from_mether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *METHER
}

#[test]
fn test_from_mether() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000000").unwrap(),
        from_mether(2));
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_kether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

#[test]
fn test_from_kether() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000").unwrap(),
        from_kether(2));
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_grand<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

#[test]
fn test_from_grand() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000").unwrap(),
        from_grand(2));
}

/// returns `value` converted from kether/grand/einstein to wei
pub fn from_einstein<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KETHER
}

#[test]
fn test_from_einstein() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000000").unwrap(),
        from_einstein(2));
}

/// returns `value` converted from ether to wei
pub fn from_ether<T: Into<U256>>(value: T) -> U256 {
    value.into() * *ETHER
}

#[test]
fn test_from_ether() {
    assert_eq!(
        U256::from_dec_str("2000000000000000000").unwrap(),
        from_ether(2));
}

/// returns `value` converted from finney to wei
pub fn from_finney<T: Into<U256>>(value: T) -> U256 {
    value.into() * *FINNEY
}

#[test]
fn test_from_finney() {
    assert_eq!(
        U256::from_dec_str("2000000000000000").unwrap(),
        from_finney(2));
}

/// returns `value` converted from szabo to wei
pub fn from_szabo<T: Into<U256>>(value: T) -> U256 {
    value.into() * *SZABO
}

#[test]
fn test_from_szabo() {
    assert_eq!(
        U256::from_dec_str("2000000000000").unwrap(),
        from_szabo(2));
}

/// returns `value` converted from gwei/shannon to wei
pub fn from_gwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GWEI
}

#[test]
fn test_from_gwei() {
    assert_eq!(
        U256::from_dec_str("2000000000").unwrap(),
        from_gwei(2));
}

/// returns `value` converted from gwei/shannon to wei
pub fn from_shannon<T: Into<U256>>(value: T) -> U256 {
    value.into() * *GWEI
}

#[test]
fn test_from_shannon() {
    assert_eq!(
        U256::from_dec_str("2000000000").unwrap(),
        from_shannon(2));
}

/// returns `value` converted from mwei/babbage to wei
pub fn from_mwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *MWEI
}

#[test]
fn test_from_mwei() {
    assert_eq!(
        U256::from_dec_str("2000000").unwrap(),
        from_mwei(2));
}

/// returns `value` converted from mwei/babbage to wei
pub fn from_babbage<T: Into<U256>>(value: T) -> U256 {
    value.into() * *MWEI
}

#[test]
fn test_from_babbage() {
    assert_eq!(
        U256::from_dec_str("2000000").unwrap(),
        from_babbage(2));
}

/// returns `value` converted from kwei/ada to wei
pub fn from_kwei<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KWEI
}

#[test]
fn test_from_kwei() {
    assert_eq!(
        U256::from_dec_str("2000").unwrap(),
        from_kwei(2));
}

/// returns `value` converted from kwei/ada to wei
pub fn from_ada<T: Into<U256>>(value: T) -> U256 {
    value.into() * *KWEI
}

#[test]
fn test_from_ada() {
    assert_eq!(
        U256::from_dec_str("2000").unwrap(),
        from_ada(2));
}

/// returns `value` converted from wei to wei.
/// unit function
pub fn from_wei<T: Into<U256>>(value: T) -> U256 {
    value.into()
}

#[test]
fn test_from_wei() {
    assert_eq!(
        U256::from(2),
        from_wei(2));
}
