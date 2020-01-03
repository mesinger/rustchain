use md5;

pub trait RustChainHash {
    fn hash(&self) -> u128;
}

pub fn md5Hash<T: AsRef<[u8]>>(data : T) -> u128 {
    let hash = md5::compute(data).0;
    unsafe { std::mem::transmute::<[u8; 16], u128>(hash) }.to_le()
}
