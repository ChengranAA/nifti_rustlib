use std::env;
use niftilib::nifti1_io::{read_first_348_bytes, print_nifti1_header};
use niftilib::nifti1::Nifti1Header;

fn main() {
    println!("==========This is the testing file for niftilib!========");
    println!("Testing for validating nifti header");
    //command line input parsing
    let args: Vec<String> = env::args().collect();


    let path = &args[1];

    let mut test_header: Nifti1Header = Nifti1Header::default();

    //read the header file to test_header struct and print it  
    match read_first_348_bytes(&path,  &mut test_header){
    Ok(()) => {print_nifti1_header(test_header)}
    Err(err) => {eprintln!("ERROR: {}", err)}
    }

}
