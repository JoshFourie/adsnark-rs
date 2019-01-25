use crate::adsnark::{_IOStream, _SizeDomainBits};

/************************* Authenticated Data ****************************/
// adsnark.hpp 178-215 & adsnark.tcc 111-139
pub struct AuthData<T, U, V> {
    mu: T,
    lambda: U,
    sigma: V
}

// adsnark.hpp 199-208
impl<T, U, V> AuthData<T, U, V> {
    pub fn auth_data(&self) {}   // overloaded fn()
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
