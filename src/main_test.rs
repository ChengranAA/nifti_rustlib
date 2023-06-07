// This file should contain testing scripts for nifti2, niftilib
// Testings should include I/O etc. 


use nifti_rslib::niftilib; 

fn main() {
    niftilib::nifti1_io::process_nifti_header();
    println!("Hello, world!");
}
