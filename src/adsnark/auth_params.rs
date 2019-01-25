use crate::adsnark::{_IOStream, _SizeDomainBits};

/************************* PUBLIC AUTHENTICATION PARAMETERS ****************************/
// adsnark.hpp 62. 
// T substitutes ppT type.
pub struct PubAuthParams<T>(T);

impl<T> PubAuthParams<T> 
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
