use std::env;
use std::fs::File;
use r1cs_file::{FieldElement,R1csFile};
use num::bigint::BigUint;

fn print_vec(prime: &BigUint, data: &[(FieldElement<32>,u32)]) {
    print!("(");
    let mut first = true;
    for (f,v) in data {
        if !first { print!(" + "); }
        first = false;
        let bytes : &[u8] = f.as_bytes();
        let i = BigUint::from_bytes_le(bytes);
        let diff = prime - i.clone();
        if diff < BigUint::from(10u32) {
            print!("-{diff}");
        } else {
            print!("{i}");
        }
        if *v != 0 {
            // Wire 0 is always forced to 1
            print!(".w{v}");
        }
    }
    print!(")");
}

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Reading file {}.",args[1]);
    let file = File::open(&args[1]).unwrap();
    let r1cs = R1csFile::<32>::read(file).unwrap();
    let bytes : &[u8] = &r1cs.header.prime.as_bytes();
    let prime = BigUint::from_bytes_le(bytes);    
    println!("Prime: {}",prime);
    println!("#Constraints: {}",r1cs.constraints.0.len());
    //
    for c in &r1cs.constraints.0 {
        print_vec(&prime, &c.0);
        print!(" * ");
        print_vec(&prime, &c.1);
        print!(" - ");        
        print_vec(&prime, &c.2);
        println!(" = 0");
    }
}
