/************************* Placeholder Traits ****************************/

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

/************************* PUBLIC AUTHENTICATION PARAMETERS ****************************/

// adsnark.hpp 62. 
// T substitutes ppT type.
pub struct PubAuthParams<T>(T);

impl<T> PubAuthParams<T> 
where 
    T: Copy // for field_out().
{
    // adsnark.hpp 78.
    pub fn pub_auth_params(self) {
        // overloaded fn()
    }
}

impl<T> _IOStream for PubAuthParams<T> {
    // adsnark.hpp 64.
    fn ostream(&self) {
        // unknown function
        // rough translate ostream << self.0 
    }

    // adsnark.hpp 67.
    fn istream(&self) {
        // unknown function
        // rough translate istream >> self.0
    }
} 

// adsnark.tcc 37.
impl<T> PartialEq for PubAuthParams<T> 
where 
    T: PartialEq,
{
    fn eq(&self, other: &PubAuthParams<T>) -> bool {
        self.0 == other.0
    }
}

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
}

/************************* Authenticated Data ****************************/
// adsnark.hpp 178-215 & adsnark.tcc 111-139
pub struct AuthData<T, U, V> {
    mu: T,
    lambda: U,
    sigma: V
}

// adsnark.hpp 199-208
impl<T, U, V> AuthData<T, U, V> {
    pub fn auth_data(&self) {}
    // overloaded fn()
}

// adsnark.tcc 119-137
impl<T, U, V> _IOStream for AuthData<T, U, V>
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

//adsnark.tcc 111-118
impl<T, U, V> PartialEq for AuthData<T, U, V> 
where   
    T: PartialEq, U: PartialEq, V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.mu == other.mu, 
            self.lambda == other.lambda,
            self.sigma == other.sigma,
        ) {
            (true, true, true) => true,
            (_, _, _) => false,
        }
    }
}

/************************* Proving Key ****************************/
// adsnark.hpp 215-302 & adsnark.tcc 140-177
pub struct ProvingKey<S, T, U, V> {
    A_query: (T, U),
    B_query: (V, U),
    C_query: (T, U),
    H_query: U,
    K_query: U,
    rA_i_Z_g1: U,
    constraint_system: S,
}

// adsnark.tcc 151-177
impl<S, T, U, V> _IOStream for ProvingKey<S, T, U, V>
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

// adsnark.tcc 140-148
impl<S, T, U, V> PartialEq for ProvingKey<S, T, U, V>
where
    S: PartialEq, T: PartialEq, U: PartialEq, V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.A_query == other.A_query,
            self.B_query == other.B_query,
            self.C_query == other.C_query,
            self.H_query == other.H_query,
            self.K_query == other.K_query,
            self.rA_i_Z_g1 == other.rA_i_Z_g1,
            self.constraint_system == other.constraint_system
        ) {
            (true, true, true, true, true, true, true) => true,
            (_, _, _, _, _, _, _) => false,
        }
    }
}

// adsnark.hpp 242-296
impl<S, T, U, V> ProvingKey<S, T, U, V> 
where
    T: _SizeDomainBits, 
    U: _SizeDomainBits, 
    V: _SizeDomainBits,
    (T, U): _SizeDomainBits, 
    (V, U): _SizeDomainBits
{
    pub fn poving_key(&self) {}
    // overloaded fn()

    pub fn g1_size(&self) -> usize {
        2 * (self.A_query.domain_size() + self.C_query.domain_size()) 
        + self.B_query.domain_size()
        + self.H_query.size() 
        + self.K_query.size()
        + 1       
    }
    pub fn g2_size(&self) -> usize {
        self.B_query.domain_size()
    }
    pub fn g1_sparse_size(&self) -> usize {
        2 * (self.A_query.size() + self.C_query.size()) 
        + self.B_query.size()
        + self.H_query.size() 
        + self.K_query.size()
        + 1       
    } 
    pub fn g2_sparse_size(&self) -> usize {
        self.B_query.size()
    }

    // missing associated fn().
    pub fn size_in_bits(&self) -> usize {
        self.A_query.size_in_bits() 
        + self.B_query.size_in_bits()
        + self.C_query.size_in_bits()
        // + libff::size_in_bits(H_query)
        // + libff::size_in_bits(K_query)
        // + libff::G1<snark_pp<ppT>>::size_in_bits()
    }
    // pub fn print_size() {}
}

/************************* Verification Key ****************************/
// adsnark.hpp 305-381 & adsnark.tcc 179-232
pub struct VerificationKey<T, U, V> {
    alphaA_g2: U,
    alphaB_g1: T,
    alphaC_g2: U,
    gamma_g2: U,
    gamma_beta_g1: T,
    gamma_beta_g2: U,
    rC_Z_g2: U,
    A0: T,
    Ain: V,
}

// adsnark.tcc 179-191
impl<T, U, V> PartialEq for VerificationKey<T, U, V>
where
    T: PartialEq, U: PartialEq, V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.alphaA_g2 == other.alphaA_g2,
            self.alphaB_g1 == other.alphaB_g1,
            self.alphaC_g2 == other.alphaC_g2,
            self.gamma_g2 == other.gamma_beta_g2,
            self.gamma_beta_g1 == other.gamma_beta_g1,
            self.gamma_beta_g2 == other.gamma_beta_g2,
            self.rC_Z_g2 == other.rC_Z_g2,
            self.A0 == other.A0,
            self.Ain == other.Ain,
        ) {
            (true, true, true, true, true, true, true, true, true) => true,
            (_, _, _, _, _, _, _, _, _) => false,
        }
    }
}

// adsnark.tcc 193-232
impl<T, U, V> _IOStream for VerificationKey<T, U, V>
{
    fn istream(&self) {}
    fn ostream(&self) {}
}

// adsnark.hpp 354-381
impl<T, U, V> VerificationKey<T, U, V> 
where 
    V: _SizeDomainBits
{
    pub fn verification_key(&self) {}
    // overloaded fn()

    pub fn g1_size(&self) -> usize {
        3 + self.Ain.size()
    }
    pub fn g2_size(&self) -> usize {
        5
    }

    // NOTE: possible zksnark bug noted in libsnark code.
    // missing associated fn().
    pub fn size_in_bits(&self) -> usize {
        self.g1_size() // * libff::G1<snark_pp<ppT>>::size_in_bits() 
        + self.g2_size() // * libff::G2<snark_pp<ppT>>::size_in_bits() 
    }

    // pub fn print_size() {}
}

/************************* Processed Verification Key ****************************/
// adsnark.hpp 384-423 & adsnark.tcc 234-304
pub struct ProcessedVerificationKey<T, U, V> {
    pp_g2_one_precomp: U,
    vk_alphaA_g2_precomp: U,
    vk_alphaB_g1_precomp: T,
    vk_alphaC_g2_precomp: U,
    vk_rC_Z_G2_precomp: U,
    vk_gamma_g2_precomp: U,
    vk_gamma_beta_g1_precomp: T,
    vk_gamma_beta_g2_precomp: U,
    vk_rC_i_g2_precomp: U,
    A0: T,
    Ain: V,
    proof_g_vki_precomp: Vec<T>,
}

// adsnark.tcc 234-255
// not sure why we throw the condition through the for loop.
impl <T, U, V> PartialEq for ProcessedVerificationKey<T, U, V>
where   
    T: PartialEq, U: PartialEq, V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let mut result = match (
            self.pp_g2_one_precomp == other.pp_g2_one_precomp,
            self.vk_alphaA_g2_precomp == other.vk_alphaA_g2_precomp,
            self.vk_alphaB_g1_precomp == other.vk_alphaB_g1_precomp,
            self.vk_alphaC_g2_precomp == other.vk_alphaC_g2_precomp,
            self.vk_rC_Z_G2_precomp == other.vk_rC_Z_G2_precomp,
            self.vk_gamma_g2_precomp == other.vk_gamma_g2_precomp,
            self.vk_gamma_beta_g1_precomp == other.vk_gamma_beta_g1_precomp,
            self.vk_gamma_beta_g2_precomp == other.vk_gamma_beta_g2_precomp,
            self.vk_rC_i_g2_precomp == other.vk_rC_i_g2_precomp,
            self.A0 == other.A0,
            self.Ain == other.Ain,
            self.proof_g_vki_precomp == other.proof_g_vki_precomp
        ) {
            (true, true, true, true, true, true, true, true, true, true, true, true) => true,
            (_, _, _, _, _, _, _, _, _, _, _, _, ) => false,
        };
        for i in 0..self.proof_g_vki_precomp.len() {
            result &= self.proof_g_vki_precomp[i] == other.proof_g_vki_precomp[i];
        };
        result
    }
}

// adsnark.tcc 257-304
impl<T, U, V> _IOStream for ProcessedVerificationKey<T, U, V>
{
    fn ostream(&self) {}
    fn istream(&self) {}
}

/************************* Key Pair ****************************/
// adsnark.hpp 426-449
pub struct KeyPair<S, T, U, V> {
    pk: ProvingKey<S, T, U, V>,
    vk: VerificationKey<T, U, V>,
}

// adsnark.hpp 437-445
impl<S, T, U, V> KeyPair<S, T, U, V> 
{
    pub fn key_pair(&self) {}
}

/************************* Proof ****************************/
// adsnark.hpp 449-549 & adsnark.tcc 307-352 
pub struct Proof<R, S, T> {
    g_A: (R, T),
    g_B: (S, T),
    g_C: (R, T),
    g_H: T,
    g_K: T,
    g_Aau: (R, T),
    muA: T,
}

// adsnark.tcc 307-317
impl<R, S, T> PartialEq for Proof<R, S, T>
where
    R: PartialEq, 
    S: PartialEq, 
    T: PartialEq,
    (R, T): PartialEq,
    (S, T): PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.g_A == other.g_A,
            self.g_B == other.g_B,
            self.g_C == other.g_C,
            self.g_H == other.g_H,
            self.g_K == other.g_K,
            self.g_Aau == other.g_Aau,
            self.muA == other.muA, 
        ) {
            (true, true, true, true, true, true, true) => true,
            (_, _, _, _, _, _, _, ) => false,
        }
    }
}

// adsnark.tcc 319-352
impl<R, S, T> _IOStream for Proof<R, S, T> 
{
    fn ostream(&self) {}
    fn istream(&self) {}
}

// adsnark.hpp 513-549
impl<R, S, T> Proof<R, S, T>
{
    pub fn g1_size() -> usize {
        10
    }
    pub fn g2_size() -> usize {
        1
    }
    pub fn size_in_bits() -> usize {
        Self::g1_size() // * libff::G1<snark_pp<ppT>>::size_in_bits() 
        + Self::g2_size() // * libff::G2<snark_pp<ppT>>::size_in_bits()
    }
    // pub fn is_well_formed(&self) -> bool {}
}

/************************* Main Algorithms ****************************/