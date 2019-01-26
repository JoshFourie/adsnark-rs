use crate::adsnark::{_IOStream};
use bn::*;

/************************* PUBLIC AUTHENTICATION PARAMETERS ****************************/
// adsnark.hpp 62. 
// T substitutes ppT type.
pub struct PubAuthParams {
    I1: G1,
}

impl PubAuthParams {
    pub fn constructor(x: G1) -> Self {
        PubAuthParams {
            I1: x,
        }
    }
}

impl _IOStream for PubAuthParams {
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
impl PartialEq for PubAuthParams {
    fn eq(&self, other: &PubAuthParams) -> bool {
        self.I1 == other.I1
    }
}
