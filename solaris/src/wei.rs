use types::U256;

lazy_static! {
    pub static ref ETHER: U256 = U256::from(10).pow(18.into());

    pub static ref GWEI: U256 = U256::from(10).pow(9.into());
    pub static ref SHANNON: U256 = U256::from(10).pow(9.into());
}

pub fn from_ether<T: Into<U256>>(v: T) -> U256 {
    v.into() * *ETHER
}

pub fn from_gwei<T: Into<U256>>(v: T) -> U256 {
    v.into() * *GWEI
}

pub fn from_shannon<T: Into<U256>>(v: T) -> U256 {
    v.into() * *SHANNON
}

pub fn from_wei<T: Into<U256>>(v: T) -> U256 {
    v.into()
}

pub fn convert(v: [u8; 32]) -> U256 {
    v.into()
}
