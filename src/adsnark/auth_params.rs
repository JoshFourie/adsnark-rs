/***********************************************************************/
/*                    PUBLIC AUTHENTICATION PARAMETERS                 */
/***********************************************************************/

use crate::adsnark::pair_crypto::_PairedCrypto;

/************************* TRAIT INTERFACES ****************************/
pub trait _PubAuthParams<T>: PartialEq {
    fn I1(&self) -> T;
    fn constructor(x: T) -> Self;
}

/************************* PUBLIC AUTHENTICATION PARAMETERS ****************************/
// adsnark.hpp 62. 

#[derive(PartialEq, Copy, Clone)]
pub struct PubAuthParams<T> { I1: T }

impl<T> _PubAuthParams<T> for PubAuthParams<T> 
where
    T: _PairedCrypto + PartialEq + Copy + Clone
{
    fn I1(&self) -> T { self.I1 }
    fn constructor(x: T) -> Self { PubAuthParams { I1: x } }
}