use crate::adsnark::{_IOStream};
use rand::{Rng, FromEntropy};
use bn::Group;

/************************* SECRET AUTHENTICATION KEY ****************************/
// 58-83 adsnark.tcc & adsnark.hpp

// adsnark.hpp 91
// T U V substitute ppT, snark_pp and ppT types.
pub struct SecAuthKey<T, U, V> {
    i: T,
    skp: U,
    seed: V,
}

impl<T, U, V> SecAuthKey<T, U, V>
{
    pub fn sec_auth_key(self) {
        // overloaded fn()
    }
}

// adsnark.tcc 66-84
impl<T, U, V> _IOStream for SecAuthKey<T, U, V> 
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

// adsnark.tcc 60
// PartialEq returns bool but libsnark returns tuple (b, b, b)
impl<T, U, V> PartialEq for SecAuthKey<T, U, V> 
where
    T: PartialEq, U: PartialEq, V: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.i == other.i,
            self.skp == other.skp,
            self.seed == other.seed,
        ) {
            (true, true, true) => true,
            (_, _, _) => false,
        }
    }
}

/************************* PUBLIC AUTHENTICATION KEY ****************************/
// adsnark.hpp 125-156 & adsnark.tcc 86-109

pub struct PubAuthKey<T, U> {
    minus_i2: T,
    vkp: U,
}

// adsnark.tcc 93-109
impl<T, U> PubAuthKey<T, U> {
    pub fn auth_key(self) {
        // overloaded fn()
    }
}

// adsnark.tcc 93-109
impl<T, U> _IOStream for PubAuthKey<T, U>
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

// adsnark.tcc 93-109
impl<T, U> PartialEq for PubAuthKey<T, U>
where
    T: PartialEq, U: PartialEq,
{
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
pub struct AuthKeys<T, U, V> {
    pap: T,
    pak: U,
    sak: V,
}

// adsnark.hpp 167-174
impl<T, U, V> AuthKeys<T, U, V>
{
    pub fn auth_keys(&self) {}
    // overloaded fn(). 

    // adsnark.tcc 376-388
    // sigkp is a generic KeyPair type with a placeholder implementation
    // prfseed simulates the generation of the aest_ctr_prf.tcc prfKeyT class.
    // swapped variable positions for i, I1 and minus_i2.
    pub fn auth_generator() // -> Self {
        let sigkp = crate::adsnark::sig_scheme::kpT::<u8, u8>::sig_gen();
        let prfseed = rand::prelude::StdRng::from_entropy().fill(&mut [0; 32]);
        let i: bn::Fr = bn::Fr::random(&mut rand::prelude::StdRng::from_entropy());
        let I1: bn::G1 = bn::G1::one() * i; 
        let minus_i2: bn::G2 = bn::G2::zero() - (bn::G2::one() * i); 
        // Self {auth_prms(I1), auth_key(minus_i2, sigkp.vk), auth_key(i, sigkp.sk, prfseed)}
    }
}
