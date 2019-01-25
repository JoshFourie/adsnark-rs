use crate::adsnark::{_IOStream, _SizeDomainBits};

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