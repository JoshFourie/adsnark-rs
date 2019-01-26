/***********************************************************************/
/*                  PUBLIC AND SECRET AUTHENTICATION KEYS              */
/***********************************************************************/

use crate::adsnark::{ 
    sig_scheme::{ _kpT, _skp, _vkp }, 
    auth_params::_PubAuthParams,
    pair_crypto::_PairedCrypto   
    };
use rand::{ FromEntropy, Rng };
use std::ops::{ Mul, Sub };

/************************* TRAIT INTERFACES ****************************/
pub trait _Seed: 
    PartialEq + Copy 
{
    fn constructor() -> Self;
}

pub trait _SecAuthKey<FR, Sk, S>: 
    PartialEq + Copy
{
    fn i(&self) -> FR;
    fn skp(&self) -> Sk;
    fn seed(&self) -> S;
    fn constructor(x: FR, y: Sk, z: S) -> Self;
}

pub trait _PubAuthKey<G2, Vk>: 
    PartialEq + Copy 
{
    fn minus_i2(&self) -> G2;
    fn vkp(&self) -> Vk;
    fn constructor(x: G2, y: Vk) -> Self;
}

pub trait _AuthKeys<FR, G1, G2, PAP, PAK, SAK, Vk, Sk, S>:
    PartialEq + Copy
{
    fn pap(&self) -> PAP;
    fn pak(&self) -> PAK;
    fn sak(&self) -> SAK;
    fn auth_generator<RNG, Kp>() -> Self where RNG: Rng + FromEntropy, Kp: _kpT<Sk, Vk> + Copy + PartialEq; // adsnark.tcc 376-388

}

/************************* STRUCTS ****************************/
// 58-83 adsnark.tcc & adsnark.hpp

// adsnar.tcc 60
#[derive(PartialEq, Copy, Clone)] pub struct Seed ([u8; 32]);

// adsnark.hpp 91
#[derive(PartialEq, Copy, Clone)] pub struct SecAuthKey<FR, Sk, S> { i: FR, skp: Sk, seed: S }

// adsnark.hpp 125-156 & adsnark.tcc 86-109
#[derive(PartialEq, Copy, Clone)] pub struct PubAuthKey<G2, Vk> { minus_i2: G2, vkp: Vk, }

// adsnark.tcc 87-92
#[derive(PartialEq, Copy, Clone)] pub struct AuthKeys<PAP, PAK, SAK> { pap: PAP, pak: PAK, sak: SAK }

/************************* IMPLEMENTATIONS ****************************/
// adsnark.hpp 158 - 178
// adsnark.hpp 167-174

// aes_ctr_prf.tcc 20-25
impl _Seed for Seed {
    fn constructor() -> Self {
        let mut x = [0; 32];
        rand::prelude::StdRng::from_entropy().fill(&mut x);
        Self(x)
    }
}

impl<FR, Sk, S> _SecAuthKey<FR, Sk, S> for SecAuthKey<FR, Sk, S> 
where 
    FR: _PairedCrypto + Copy + PartialEq, 
    Sk: _skp + Copy + PartialEq, 
    S: _Seed + Copy + PartialEq,
{
    fn i(&self) -> FR { self.i }
   
    fn skp(&self) -> Sk { self.skp } 
   
    fn seed(&self) -> S { self.seed }
   
    fn constructor(x: FR, y: Sk, z: S) -> Self { Self { i: x, skp: y, seed: z} }
}

impl<G2, Vk> _PubAuthKey<G2, Vk> for PubAuthKey<G2, Vk> 
where
    G2: _PairedCrypto + Copy + PartialEq, 
    Vk: _vkp + Copy + PartialEq, 
{
    fn minus_i2(&self) -> G2 { self.minus_i2 }
    
    fn vkp(&self) -> Vk { self.vkp }
    
    fn constructor(x: G2, y: Vk) -> Self { Self { minus_i2: x, vkp: y } }
}

impl<FR, G1, G2, PAP, PAK, SAK, Vk, Sk, S> 
    _AuthKeys<FR, G1, G2, PAP, PAK, SAK, Vk, Sk, S>
for 
    AuthKeys<PAP, PAK, SAK> 
where   
    PAP: _PubAuthParams<G1> 
        + Copy
        + PartialEq, 
    PAK: _PubAuthKey<G2, Vk>
        + Copy
        + PartialEq, 
    SAK: _SecAuthKey<FR, Sk, S>
        + Copy
        + PartialEq,
    FR: _PairedCrypto
        + Copy
        + PartialEq, 
    G1: _PairedCrypto
        + Mul<FR, Output=G1>
        + Copy
        + PartialEq, 
    G2: _PairedCrypto
        + Mul<FR, Output=G2>
        + Sub<G2, Output=G2>
        + Copy
        + PartialEq, 
    Sk: _skp
        + Copy
        + PartialEq, 
    Vk: _vkp
        + Copy
        + PartialEq,
    S: _Seed
        + Copy
        + PartialEq,

{
    fn pap(&self) -> PAP { self.pap }

    fn pak(&self) -> PAK { self.pak }
    
    fn sak(&self) -> SAK { self.sak }
    
    fn auth_generator<RNG, Kp>() -> Self 
    where
        RNG: Rng + FromEntropy,
        Kp: _kpT<Sk, Vk>
            + Copy
            + PartialEq,
    {
        let sigkp: Kp = Kp::sig_gen();
        let prfseed = S::constructor(); // aes_ctr_prf.tcc 20-25
        let i: FR = FR::random(&mut RNG::from_entropy());
        let I1 = G1::one() * i; 
        let minus_i2 = G2::zero() - (G2::one() * i); 
        Self { 
            pap: PAP::constructor(I1),
            pak: PAK::constructor(minus_i2, sigkp.vk()),
            sak: SAK::constructor(i, sigkp.sk(), prfseed)
        }
    }
}

#[cfg(test)]
mod tests { 
    
    use {
        crate::adsnark::{ 
            pair_crypto::{ FrLocal, G1Local, G2Local }, 
            auth_params::PubAuthParams, sig_scheme::{ skp, kpT, vkp}, 
            auth_key::{_AuthKeys, PubAuthKey, SecAuthKey, AuthKeys, Seed} 
        }, 
        rand::prelude::StdRng
    }; 
       
    #[test]
    fn auth_generator() { 
        let auth: AuthKeys<PubAuthParams<G1Local>, PubAuthKey<G2Local, _>, SecAuthKey<FrLocal, _, Seed>> 
            = AuthKeys::auth_generator::<StdRng, kpT<skp, vkp>>();
    }
}