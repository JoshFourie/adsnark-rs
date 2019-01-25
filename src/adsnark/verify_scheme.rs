use crate::adsnark::{_IOStream, _SizeDomainBits};

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