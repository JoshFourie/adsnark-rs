# libsnark-rs: AD-SNARK 

Forking libsnark in C++ from SCIPR Labs to rust-lang for OMNIA Protocol.


## DEV-Notes
Josh:

    #3 I've implemented an interface for the bn crate in pair_crypto that should enable us to manage generics.
    The auth_generator function which gives us an AuthKey. There is a test implemented but all it does is 
    check that the auth_generator doesn't panic when it's called. Next task is to tackle Proof and Verification.