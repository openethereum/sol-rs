use types::U256;

lazy_static! {
    pub static ref ETHER: U256 = U256::from(10).pow(18.into());

    pub static ref GWEI: U256 = U256::from(10).pow(9.into());
    pub static ref SHANNON: U256 = U256::from(10).pow(9.into());
}

pub fn ether(v: usize) -> U256 {
    U256::from(v) * *ETHER
}

pub fn gwei(v: usize) -> U256 {
    U256::from(v) * *GWEI
}

pub fn shannon(v: usize) -> U256 {
    U256::from(v) * *SHANNON
}

pub fn wei(v: usize) -> U256 {
    v.into()
}

pub fn convert(v: [u8; 32]) -> U256 {
    v.into()
}
