# libsnark-rs: AD-SNARK 

Forking libsnark in C++ from SCIPR Labs to rust-lang for OMNIA Protocol.


## DEV-Notes
Josh:

    #1 I've started with r1cs_ppzkadsnark.tcc/.hpp and we can build the dependecies from there so we can avoid 
    overlapping where we fork things we don't need for ad-SNARKS. The code has comments with the file_name 
    and the line_number that I've used for the source: 'adsnark.tcc 171-189' is equivalent to 'I took this 
    from lines 171 to 189 of the r1cs_ppzkadsnark.tcc file'. We'll init everything with generics and put in 
    the trait-parameters as needed.

    We should be able to find a lot of the core dependencies floating around on crates.io. The BN crate used 
    in our zksnark-rs fork from Republic Protocol is provided by Z-Cash Hackworks (associated with SCIPR Labs) 
    and has pairing-crypto functionality (that is, it has all of the G1 and G2 structs scattered in the 
    libsnark library done for us).

    Note that we'll swap out a lot of the bespoke impls such as 'PartialEq' for their macro equivalents to 
    improve readability, but for the moment we'll handwrite them out.

    #2 We have a skeleton of the main structures and can start pulling in the main dependencies
    from the pairing-crypto lib. 