/********* Signature Interface for AD-SNARK-rs ***********/
// adsnark_signature.hpp 25-30 
pub struct kpT<T, U> {
    sk: T,
    vk: U,
}

// adsnark_signature.hpp 32-42
impl<T, U> kpT<T, U> {
    pub fn sig_gen() {}
    pub fn sig_sign() {}
    pub fn sig_verify() {}
}