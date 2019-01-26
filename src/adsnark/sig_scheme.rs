/***********************************************************************/
/*                          SIGNATURE SCHEME                           */
/***********************************************************************/

use rand::{Rng, FromEntropy};

/************************* TRAIT INTERFACES ****************************/

pub trait _skp: 
    PartialEq + Copy 
{
    type Key;
    fn get(&self) -> Self::Key;
    fn constructor(k: Self::Key) -> Self;
}

pub trait _vkp:  
    PartialEq + Copy 
{
    type Key;
    fn constructor(k: Self::Key) -> Self;
    fn get(&self) -> Self::Key;
}

pub trait _kpT<Sk, Vk> {
    fn constructor(sk: Sk, vk: Vk) -> Self;
    fn sk(&self) -> Sk;
    fn vk(&self) -> Vk;
    fn sig_gen() -> Self;
}

/************************* STRUCTS ****************************/
// adsnark_signature.hpp 25-30 

#[derive(PartialEq, Copy, Clone)] pub struct skp( [u8; 32] );

#[derive(Copy, Clone)] pub struct vkp( [u8; 64] );

#[derive(Copy, Clone)] pub struct kpT<Sk, Vk> { sk: Sk, vk: Vk }

/************************* IMPLEMENTATIONS ****************************/

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

impl _skp for skp {
    type Key = [u8; 32];
    fn constructor(k: Self::Key) -> Self { skp(k)}
    fn get(&self) -> Self::Key { self.0 }
}

impl _vkp for vkp {
    type Key = [u8; 64];
    fn constructor(k: Self::Key) -> Self { vkp(k)}
    fn get(&self) -> Self::Key { self.0 }
}

// adsnark_signature.hpp 32-42
impl<Sk, Vk> _kpT<Sk, Vk> for kpT<Sk, Vk> 
where
    Sk: _skp<Key=[u8; 32]>, 
    Vk: _vkp<Key=[u8; 64]>,
{

    fn constructor(sk: Sk, vk: Vk) -> Self { Self { sk, vk } }

    fn sig_gen() -> Self {
        let mut x = [0; 32];
        rand::prelude::StdRng::from_entropy().fill(&mut x[..]);
        let mut y = [0; 64];
        rand::prelude::StdRng::from_entropy().fill(&mut y[..]);
        Self {
            sk: Sk::constructor(x),
            vk: Vk::constructor(y),
        }
    }
    fn sk(&self) -> Sk {
        self.sk
    }
    fn vk(&self) -> Vk {
        self.vk
    }
}


// NOTES and TODOs
// input proper sig_gen.