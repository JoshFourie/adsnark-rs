/***********************************************************************/
/*                          SIGNATURE SCHEME                           */
/***********************************************************************/

use rand::{Rng, FromEntropy};

/************************* TRAIT INTERFACES ****************************/

pub trait _skp: PartialEq {
    fn get() -> [u8; 32];
}

pub trait _vkp: PartialEq {
    fn get() -> [u8; 64];
}

pub trait _kpT<S, V> 
where 
    S: _skp, V: _vkp,
{
    fn sk() -> S;
    fn vk() -> V;
    fn sig_gen() -> Self;
}

/************************* STRUCTS ****************************/
// adsnark_signature.hpp 25-30 

#[derive(PartialEq)]
pub struct skp([u8; 32]);

pub struct vkp([u8; 64]);

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

pub struct kpT {
    pub sk: skp,
    pub vk: vkp,
}

// adsnark_signature.hpp 32-42
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


// NOTES and TODOs
// input proper sig_gen.