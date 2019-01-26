use crate::adsnark::*;
/************************* SECRET AUTHENTICATION KEY ****************************/
// 58-83 adsnark.tcc & adsnark.hpp

#[derive(PartialEq)] // adsnark.tcc 60
pub struct Seed ([u8; 32]);

impl Seed {
    pub fn constructor() -> Self {
        use rand::{Rng, FromEntropy};
        let mut x = [0; 32];
        rand::prelude::StdRng::from_entropy().fill(&mut x);
        Self(x)
    }
}

// adsnark.hpp 91
pub struct SecAuthKey {
    i: bn::Fr,
    skp: sig_scheme::skp,
    seed: Seed,
}

impl SecAuthKey {
    pub fn sec_auth_key(self) {}
    pub fn constructor(x: bn::Fr, y: sig_scheme::skp, z: Seed) -> Self { Self { i: x, skp: y, seed: z} }
}



/************************* PUBLIC AUTHENTICATION KEY ****************************/
// adsnark.hpp 125-156 & adsnark.tcc 86-109
pub struct PubAuthKey {
    minus_i2: bn::G2,
    vkp: sig_scheme::vkp,
}

impl PubAuthKey {
    pub fn auth_key(self) {
        // overloaded fn()
    }
    pub fn constructor(x: bn::G2, y: sig_scheme::vkp) -> Self {
        Self {
            minus_i2: x,
            vkp: y,
        }
    }
}

// adsnark.tcc 93-109
impl _IOStream for PubAuthKey
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

// adsnark.tcc 87-92
impl PartialEq for PubAuthKey {
    fn eq(&self, other: &Self) -> bool {
        match (
            self.minus_i2 == other.minus_i2,
            self.vkp == other.vkp
        ) {
            (true, true) => true,
            (_, _) => false
        }
    }
}


/************************* Authentication Key Material ****************************/
// adsnark.hpp 158 - 178
pub struct AuthKeys {
    pap: auth_params::PubAuthParams,
    pak: PubAuthKey,
    sak: SecAuthKey,
}

// adsnark.hpp 167-174
impl AuthKeys {
    fn auth_keys(&self) {}

    // adsnark.tcc 376-388
    // sigkp is a generic KeyPair type with a placeholder implementation
    // prfseed simulates the generation of the aest_ctr_prf.tcc prfKeyT class.
    // swapped variable positions for i, I1 and minus_i2.
    fn auth_generator() -> Self {
        use {rand::{Rng, FromEntropy}, bn::{Fr, G1, G2, Group}, auth_params::PubAuthParams, sig_scheme::kpT};

        let sigkp: kpT = kpT::sig_gen();
        // aes_ctr_prf.tcc 20-25
        let prfseed = Seed::constructor();
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
