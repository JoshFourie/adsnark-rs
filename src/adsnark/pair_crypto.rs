/***********************************************************************/
/*                          PAIRED-CRYPTO                              */
/***********************************************************************/

use bn::{ Group, Fr, G1, G2 };
use rand::Rng;

/************************* PUBLIC TRAITS ****************************/

pub trait _PairedCrypto {
    fn one() -> Self;
    fn zero() -> Self;
    fn random<R: Rng>(r: &mut R) -> Self;
}

/************************* PAIRED-CRYPTO STRUCTS ****************************/

pub struct G1Local(G1);

pub struct G2Local(G2);

pub struct FrLocal(Fr);

/************************* PAIRED-CRYPTO IMPLEMENTATIONS ****************************/

impl _PairedCrypto for G1Local {

    fn one() ->  Self {
        Self( G1::one() )
    }
    fn zero() -> Self {
        Self( G1::zero() )
    }
    fn random<R: Rng>(r: &mut R) -> Self {
        Self( G1::random(r) )
    }
}

impl _PairedCrypto for G2Local {

    fn one() ->  Self {
        Self( G2::one() )
    }
    fn zero() -> Self {
        Self( G2::zero() )
    }
    fn random<R: Rng>(r: &mut R) -> Self {
        Self( G2::random(r) )
    }
}

impl _PairedCrypto for FrLocal {

    fn one() ->  Self {
        Self( Fr::one() )
    }
    fn zero() -> Self {
        Self( Fr::zero() )
    }
    fn random<R: Rng>(r: &mut R) -> Self {
        Self( Fr::random(r) )
    }
}
