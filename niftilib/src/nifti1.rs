#[derive(Debug)]
#[repr(C)]
pub struct Nifti1Header {
    sizeof_hdr: i32,
    data_type: [u8; 10],
    db_name: [u8; 18],
    extents: i32,
    session_error: i16,
    regular: u8,
    dim_info: u8,
    dim: [i16; 8],
    intent_p1: f32,
    intent_p2: f32,
    intent_p3: f32,
    intent_code: i16,
    datatype: i16,
    bitpix: i16,
    slice_start: i16,
    pixdim: [f32; 8],
    vox_offset: f32,
    scl_slope: f32,
    scl_inter: f32,
    slice_end: i16,
    slice_code: u8,
    xyzt_units: u8,
    cal_max: f32,
    cal_min: f32,
    slice_duration: f32,
    toffset: f32,
    glmax: i32,
    glmin: i32,
    descrip: [u8; 80],
    aux_file: [u8; 24],
    qform_code: i16,
    sform_code: i16,
    quatern_b: f32,
    quatern_c: f32,
    quatern_d: f32,
    qoffset_x: f32,
    qoffset_y: f32,
    qoffset_z: f32,
    srow_x: [f32; 4],
    srow_y: [f32; 4],
    srow_z: [f32; 4],
    intent_name: [u8; 16],
    magic: [u8; 4],
}

// default Nifti1Header use for debug
impl Default for Nifti1Header {
    fn default() -> Self {
        Nifti1Header {
            sizeof_hdr: 348,
            data_type: [0; 10],
            db_name: [0; 18],
            extents: 0,
            session_error: 0,
            regular: 0,
            dim_info: 0,
            dim: [0; 8],
            intent_p1: 0.0,
            intent_p2: 0.0,
            intent_p3: 0.0,
            intent_code: 0,
            datatype: 0,
            bitpix: 0,
            slice_start: 0,
            pixdim: [0.0; 8],
            vox_offset: 0.0,
            scl_slope: 0.0,
            scl_inter: 0.0,
            slice_end: 0,
            slice_code: 0,
            xyzt_units: 0,
            cal_max: 0.0,
            cal_min: 0.0,
            slice_duration: 0.0,
            toffset: 0.0,
            glmax: 0,
            glmin: 0,
            descrip: [0; 80],
            aux_file: [0; 24],
            qform_code: 0,
            sform_code: 0,
            quatern_b: 0.0,
            quatern_c: 0.0,
            quatern_d: 0.0,
            qoffset_x: 0.0,
            qoffset_y: 0.0,
            qoffset_z: 0.0,
            srow_x: [0.0; 4],
            srow_y: [0.0; 4],
            srow_z: [0.0; 4],
            intent_name: [0; 16],
            magic: *b"ni1\0",
        }
    }
}
