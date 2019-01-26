pub mod auth_data;
pub mod auth_key;
pub mod prf_scheme;
pub mod auth_params;
pub mod sig_scheme;
pub mod verify_scheme;

/************************* Placeholder Traits ****************************/
// these are added iteratively as required.

pub trait _PairedCrypto {
    // simulates paired-crypto operations.
    fn one() -> Self;
    fn zero() -> Self;
    fn random<R: rand::Rng>(r: R) -> Self;
}

// istream and ostream '<<' equivalent
pub trait _IOStream {
    fn ostream(&self);
    fn istream(&self);
}

// size methods for the G1/G2 groupings
// not confident how the size_in_bits impl will work as a const.
pub trait _SizeDomainBits {
    fn size(&self) -> usize;
    fn domain_size(&self) -> usize;
    fn size_in_bits(&self) -> usize;
}


/************************* Key Pair ****************************/
// adsnark.hpp 426-449
pub struct KeyPair<S, T, U, V> {
    pk: prf_scheme::ProvingKey<S, T, U, V>,
    vk: verify_scheme::VerificationKey<T, U, V>,
}

// adsnark.hpp 437-445
impl<S, T, U, V> KeyPair<S, T, U, V> 
{
    pub fn key_pair(&self) {}
}
