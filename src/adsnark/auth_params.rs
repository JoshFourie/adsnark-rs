/***********************************************************************/
/*                    PUBLIC AUTHENTICATION PARAMETERS                 */
/***********************************************************************/

/************************* TRAIT INTERFACES ****************************/
pub trait _PubAuthParams<T, G>: PartialEq {
    fn I1() -> T;
    fn constructor(x: G) -> Self;
}

/************************* PUBLIC AUTHENTICATION PARAMETERS ****************************/
// adsnark.hpp 62. 

use bn::*;

#[derive(PartialEq)]
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