use rand::{Rng, FromEntropy};

/********* Signature Interface for AD-SNARK-rs ***********/
// adsnark_signature.hpp 25-30 
#[derive(PartialEq)]
pub struct skp([u8; 32]);

pub struct vkp([u8; 64]);

pub struct kpT {
    pub sk: skp,
    pub vk: vkp,
}

// adsnark_signature.hpp 32-42
// placeholder arrays
impl kpT {
    pub fn sig_gen() -> Self {
        let mut x = [0; 32];
        rand::prelude::StdRng::from_entropy().fill(&mut x[..]);
        let mut y = [0; 64];
        rand::prelude::StdRng::from_entropy().fill(&mut y[..]);
        Self {
            sk: skp(x),
            vk: vkp(y),
        }
    }
    pub fn sig_sign() {}
    pub fn sig_verify() {}
}

impl PartialEq for vkp {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            match self.0[i] == other.0[i] {
                true => {},
                false => return false,
            }
        }
        true
    }
}


// NOTES and TODOs
// input proper sig_gen 