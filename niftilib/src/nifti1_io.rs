use crate::nifti1::Nifti1Header;

pub fn process_nifti_header() {
    let header = Nifti1Header::default();

    // Perform some operations on the header
    println!("Header: {:?}", header);
}
