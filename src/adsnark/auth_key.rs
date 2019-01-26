/***********************************************************************/
/*                  PUBLIC AND SECRET AUTHENTICATION KEYS              */
/***********************************************************************/

use crate::adsnark::{
    sig_scheme::{_kpT, _skp, _vkp},
    auth_params::_PubAuthParams,
    _PairedCrypto,
    };

use crate::adsnark::{sig_scheme, auth_params};

/************************* TRAIT INTERFACES ****************************/
pub trait _Seed: 
    PartialEq + Copy 
{
    fn constructor() -> Self;
}

pub trait _SecAuthKey<FR, Sk, S>: 
    PartialEq + Copy
where 
    FR: _PairedCrypto, Sk: _skp, S: _Seed,
{
    fn i(&self) -> FR;
    fn skp(&self) -> Sk;
    fn seed(&self) -> S;
    fn constructor(x: FR, y: Sk, z: S) -> Self;
}

pub trait _PubAuthKey<G2, Vk>: 
    PartialEq + Copy 
where
    G2: _PairedCrypto, Vk: _vkp,
{
    fn minus_i2(&self) -> G2;
    fn vkp(&self) -> Vk;
    fn constructor(x: G2, y: Vk) -> Self;
}

pub trait _AuthKeys<FR, G1, G2, PAP, PAK, SAK, Vk, Sk, S>:
    PartialEq + Copy
where
    PAP: _PubAuthParams<G1>, 
    PAK: _PubAuthKey<G2, Vk>, 
    SAK: _SecAuthKey<FR, Sk, S>,
    FR: _PairedCrypto, 
    G1: _PairedCrypto, 
    G2: _PairedCrypto,
    Vk: _vkp, 
    Sk: _skp, 
    S: _Seed,
{
    fn pap(&self) -> PAP;
    fn pak(&self) -> PAK;
    fn sak(&self) -> SAK;
    fn auth_generator() -> Self; // adsnark.tcc 376-388

}

/************************* STRUCTS ****************************/

// 58-83 adsnark.tcc & adsnark.hpp
#[derive(PartialEq, Copy, Clone)] // adsnark.tcc 60
pub struct Seed ([u8; 32]);

// adsnark.hpp 91
#[derive(PartialEq, Copy, Clone)]
pub struct SecAuthKey<FR, Sk, S> 
{
    i: FR,
    skp: Sk,
    seed: S,
}

// adsnark.hpp 125-156 & adsnark.tcc 86-109
#[derive(PartialEq, Copy, Clone)] // adsnark.tcc 87-92
pub struct PubAuthKey<G2, Vk> 
{
    minus_i2: G2,
    vkp: Vk,
}

#[derive(PartialEq, Copy, Clone)]
pub struct AuthKeys<PAP, PAK, SAK> 
{
    pap: PAP,
    pak: PAK,
    sak: SAK,
}

/************************* IMPLEMENTATIONS ****************************/
// adsnark.hpp 158 - 178
// adsnark.hpp 167-174

// aes_ctr_prf.tcc 20-25
impl _Seed for Seed {
    fn constructor() -> Self {
        use rand::{Rng, FromEntropy};
        let mut x = [0; 32];
        rand::prelude::StdRng::from_entropy().fill(&mut x);
        Self(x)
    }
}

impl<FR, Sk, S> _SecAuthKey<FR, Sk, S> for SecAuthKey<FR, Sk, S> 
where 
    FR: _PairedCrypto 
        + Copy
        + PartialEq, 
    Sk: _skp 
        + Copy
        + PartialEq, 
    S: _Seed 
        + Copy
        + PartialEq,
{
    fn i(&self) -> FR {
        self.i
    }
    fn skp(&self) -> Sk {
        self.skp
    } 
    fn seed(&self) -> S {
        self.seed
    }
    fn constructor(x: FR, y: Sk, z: S) -> Self {
        Self { i: x, skp: y, seed: z} 
    }
}

impl<G2, Vk> _PubAuthKey<G2, Vk> for PubAuthKey<G2, Vk> 
where
    G2: _PairedCrypto
        + Copy
        + PartialEq, 
    Vk: _vkp
        + Copy
        + PartialEq, 
{
    fn minus_i2(&self) -> G2 {
        self.minus_i2
    }
    fn vkp(&self) -> Vk {
        self.vkp
    }
    fn constructor(x: G2, y: Vk) -> Self {
        Self {
            minus_i2: x,
            vkp: y,
        }
    }
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
        + Copy
        + PartialEq, 
    G2: _PairedCrypto
        + Copy
        + PartialEq,
    Vk: _vkp
        + Copy
        + PartialEq, 
    Sk: _skp
        + Copy
        + PartialEq, 
    S: _Seed
        + Copy
        + PartialEq,

{
    fn pap(&self) -> PAP {
        self.pap
    }
    fn pak(&self) -> PAK {
        self.pak
    }
    fn sak(&self) -> SAK {
        self.sak
    }
    fn auth_generator() -> Self {
        let sigkp: kpT = kpT::sig_gen();
        let prfseed = Seed::constructor(); // aes_ctr_prf.tcc 20-25
        let i: Fr = Fr::random(&mut rand::prelude::StdRng::from_entropy());
        let I1: G1 = G1::one() * i; 
        let minus_i2: G2 = G2::zero() - (G2::one() * i); 
        Self {
            pap: PubAuthParams::constructor(I1),
            pak: PubAuthKey::constructor(minus_i2, sigkp.vk),
            sak: SecAuthKey::constructor(i, sigkp.sk, prfseed)
        }
    }
}
