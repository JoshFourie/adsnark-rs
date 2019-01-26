/***********************************************************************/
/*                          PAIRED-CRYPTO                              */
/***********************************************************************/

use bn::{ Group, Fr, G1, G2 };
use rand::Rng;
use std::ops::{ Mul, Sub};

/************************* PUBLIC TRAITS ****************************/

pub trait _PairedCrypto {
    fn one() -> Self;
    fn zero() -> Self;
    fn random<R: Rng>(r: &mut R) -> Self;
}

/************************* PAIRED-CRYPTO STRUCTS ****************************/


#[derive(PartialEq, Copy, Clone)] pub struct G1Local(G1);


#[derive(PartialEq, Copy, Clone)] pub struct G2Local(G2);


#[derive(PartialEq, Copy, Clone)] pub struct FrLocal(Fr);

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

impl Mul<FrLocal> for G1Local {
    type Output = Self;
    fn mul(self, rhs: FrLocal) -> Self {
        Self( self.0 * rhs.0 )
    }
}

impl Mul<FrLocal> for G2Local {
    type Output = Self;
    fn mul(self, rhs: FrLocal) -> Self {
        Self( self.0 * rhs.0 )
    }
}

impl Sub<Self> for G2Local {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self( self.0 - rhs.0 )
    }
}