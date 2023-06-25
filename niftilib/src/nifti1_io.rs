use std::str;
use std::fs::{File};
use std::io::{Read, Error};
use crate::nifti1::{Nifti1Header, NiftiOffsets};

// This is function is used to read the whole header part of the NIFTI file into 
pub fn read_first_348_bytes(filename: &str, ret: &mut Nifti1Header) -> Result<(), Error> {
    const BYTES_TO_READ: usize = std::mem::size_of::<Nifti1Header>();
    let mut file = File::open(filename)?;
    let mut byte_buffer = [0u8; BYTES_TO_READ];

    file.read_exact(&mut byte_buffer)?;

    *ret = deserialize_header(&byte_buffer)?;

    Ok(())
}

pub fn deserialize_header(header_bytes: &[u8]) -> Result<Nifti1Header, Error> {
    if header_bytes.len() != 348 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Invalid header size. Expected 348 bytes.",
        ));
    }

    let sizeof_hdr = i32::from_ne_bytes(
        header_bytes[NiftiOffsets::SizeOfHdr as usize..NiftiOffsets::DataType as usize]
            .try_into()
            .unwrap(),
    );

    let data_type = {
        let mut data_type = [0; 10];
        data_type.copy_from_slice(
            &header_bytes[NiftiOffsets::DataType as usize..NiftiOffsets::DbName as usize],
        );
        data_type
    };

    let db_name = {
        let mut db_name = [0; 18];
        db_name.copy_from_slice(
            &header_bytes[NiftiOffsets::DbName as usize..NiftiOffsets::Extents as usize],
        );
        db_name
    };

    let extents = i32::from_ne_bytes(
        header_bytes[NiftiOffsets::Extents as usize..NiftiOffsets::SessionError as usize]
            .try_into()
            .unwrap(),
    );

    let session_error = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::SessionError as usize..NiftiOffsets::Regular as usize]
            .try_into()
            .unwrap(),
    );

    let regular = header_bytes[NiftiOffsets::Regular as usize];

    let dim_info = header_bytes[NiftiOffsets::DimInfo as usize];

    let dim: [i16; 8] = {
        let mut dim: [i16; 8] = [0; 8];
        let dim_bytes = &header_bytes[NiftiOffsets::Dim as usize..NiftiOffsets::Dim as usize + 16];
        for i in 0..8 {
            dim[i] = i16::from_ne_bytes(dim_bytes[i * 2..(i * 2) + 2].try_into().unwrap());
        }
        dim
    };

    let intent_p1 = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::IntentP1 as usize..NiftiOffsets::IntentP2 as usize]
            .try_into()
            .unwrap(),
    );

    let intent_p2 = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::IntentP2 as usize..NiftiOffsets::IntentP3 as usize]
            .try_into()
            .unwrap(),
    );

    let intent_p3 = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::IntentP3 as usize..NiftiOffsets::IntentCode as usize]
            .try_into()
            .unwrap(),
    );

    let intent_code = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::IntentCode as usize..NiftiOffsets::DataTypeCode as usize]
            .try_into()
            .unwrap(),
    );

    let datatype = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::DataTypeCode as usize..NiftiOffsets::Bitpix as usize]
            .try_into()
            .unwrap(),
    );

    let bitpix = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::Bitpix as usize..NiftiOffsets::SliceStart as usize]
            .try_into()
            .unwrap(),
    );

    let slice_start = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::SliceStart as usize..NiftiOffsets::Pixdim as usize]
            .try_into()
            .unwrap(),
    );

    let pixdim = {
        let mut pixdim = [0.0; 8];
        for i in 0..8 {
            let offset = NiftiOffsets::Pixdim as usize + i * 4;
            pixdim[i] = f32::from_ne_bytes(
                header_bytes[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            );
        }
        pixdim
    };

    let vox_offset = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::VoxOffset as usize..NiftiOffsets::SclSlope as usize]
            .try_into()
            .unwrap(),
    );

    let scl_slope = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::SclSlope as usize..NiftiOffsets::SclInter as usize]
            .try_into()
            .unwrap(),
    );

    let scl_inter = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::SclInter as usize..NiftiOffsets::SliceEnd as usize]
            .try_into()
            .unwrap(),
    );

    let slice_end = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::SliceEnd as usize..NiftiOffsets::SliceCode as usize]
            .try_into()
            .unwrap(),
    );

    let slice_code = header_bytes[NiftiOffsets::SliceCode as usize];

    let xyzt_units = header_bytes[NiftiOffsets::XyztUnits as usize];

    let cal_max = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::CalMax as usize..NiftiOffsets::CalMin as usize]
            .try_into()
            .unwrap(),
    );

    let cal_min = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::CalMin as usize..NiftiOffsets::SliceDuration as usize]
            .try_into()
            .unwrap(),
    );

    let slice_duration = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::SliceDuration as usize..NiftiOffsets::TOffset as usize]
            .try_into()
            .unwrap(),
    );

    let toffset = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::TOffset as usize..NiftiOffsets::GlMax as usize]
            .try_into()
            .unwrap(),
    );

    let glmax = i32::from_ne_bytes(
        header_bytes[NiftiOffsets::GlMax as usize..NiftiOffsets::GlMin as usize]
            .try_into()
            .unwrap(),
    );

    let glmin = i32::from_ne_bytes(
        header_bytes[NiftiOffsets::GlMin as usize..NiftiOffsets::Descrip as usize]
            .try_into()
            .unwrap(),
    );

    let descrip = {
        let mut descrip = [0; 80];
        descrip.copy_from_slice(
            &header_bytes[NiftiOffsets::Descrip as usize..NiftiOffsets::AuxFile as usize],
        );
        descrip
    };

    let aux_file = {
        let mut aux_file = [0; 24];
        aux_file.copy_from_slice(
            &header_bytes[NiftiOffsets::AuxFile as usize..NiftiOffsets::QFormCode as usize],
        );
        aux_file
    };

    let qform_code = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::QFormCode as usize..NiftiOffsets::SFormCode as usize]
            .try_into()
            .unwrap(),
    );

    let sform_code = i16::from_ne_bytes(
        header_bytes[NiftiOffsets::SFormCode as usize..NiftiOffsets::QuaternB as usize]
            .try_into()
            .unwrap(),
    );

    let quatern_b = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QuaternB as usize..NiftiOffsets::QuaternC as usize]
            .try_into()
            .unwrap(),
    );

    let quatern_c = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QuaternC as usize..NiftiOffsets::QuaternD as usize]
            .try_into()
            .unwrap(),
    );

    let quatern_d = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QuaternD as usize..NiftiOffsets::QOffsetX as usize]
            .try_into()
            .unwrap(),
    );

    let qoffset_x = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QOffsetX as usize..NiftiOffsets::QOffsetY as usize]
            .try_into()
            .unwrap(),
    );

    let qoffset_y = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QOffsetY as usize..NiftiOffsets::QOffsetZ as usize]
            .try_into()
            .unwrap(),
    );

    let qoffset_z = f32::from_ne_bytes(
        header_bytes[NiftiOffsets::QOffsetZ as usize..NiftiOffsets::SRowX as usize]
            .try_into()
            .unwrap(),
    );

    let srow_x = {
        let mut srow_x = [0.0; 4];
        for i in 0..4 {
            let offset = NiftiOffsets::SRowX as usize + i * 4;
            srow_x[i] = f32::from_ne_bytes(
                header_bytes[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            );
        }
        srow_x
    };

    let srow_y = {
        let mut srow_y = [0.0; 4];
        for i in 0..4 {
            let offset = NiftiOffsets::SRowY as usize + i * 4;
            srow_y[i] = f32::from_ne_bytes(
                header_bytes[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            );
        }
        srow_y
    };

    let srow_z = {
        let mut srow_z = [0.0; 4];
        for i in 0..4 {
            let offset = NiftiOffsets::SRowZ as usize + i * 4;
            srow_z[i] = f32::from_ne_bytes(
                header_bytes[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            );
        }
        srow_z
    };

    let intent_name = {
        let mut intent_name = [0; 16];
        intent_name.copy_from_slice(
            &header_bytes[NiftiOffsets::IntentName as usize..NiftiOffsets::Magic as usize],
        );
        intent_name
    };

    let magic = {
        let mut magic = [0; 4];
        magic.copy_from_slice(
            &header_bytes[NiftiOffsets::Magic as usize..NiftiOffsets::Magic as usize + 4],
        );
        magic
    };

    let header = Nifti1Header {
        sizeof_hdr,
        data_type,
        db_name,
        extents,
        session_error,
        regular,
        dim_info,
        dim,
        intent_p1,
        intent_p2,
        intent_p3,
        intent_code,
        datatype,
        bitpix,
        slice_start,
        pixdim,
        vox_offset,
        scl_slope,
        scl_inter,
        slice_end,
        slice_code,
        xyzt_units,
        cal_max,
        cal_min,
        slice_duration,
        toffset,
        glmax,
        glmin,
        descrip,
        aux_file,
        qform_code,
        sform_code,
        quatern_b,
        quatern_c,
        quatern_d,
        qoffset_x,
        qoffset_y,
        qoffset_z,
        srow_x,
        srow_y,
        srow_z,
        intent_name,
        magic,
    };

    Ok(header)
}


pub fn validate_nifti_header(file_path: String){
    println!("{}", file_path); 
}

pub fn print_nifti1_header(header: Nifti1Header) {
    println!("sizeof_hdr: {}", header.sizeof_hdr);
    println!("dim_info: {}", header.dim_info);
    print!("dim: ");
    for i in 0..header.dim.len() {
        print!("{} ", header.dim[i]);
    }
    println!();
    println!("intent_p1: {}", header.intent_p1);
    println!("intent_p2: {}", header.intent_p2);
    println!("intent_p3: {}", header.intent_p3);
    println!("intent_code: {}", header.intent_code);
    println!("datatype: {}", header.datatype);
    println!("bitpix: {}", header.bitpix);
    println!("slice_start: {}", header.slice_start);
    print!("pixdim: ");
    for i in 0..header.pixdim.len() {
        print!("{} ", header.pixdim[i]);
    }
    println!();
    println!("vox_offset: {}", header.vox_offset);
    println!("scl_slope: {}", header.scl_slope);
    println!("scl_inter: {}", header.scl_inter);
    println!("slice_end: {}", header.slice_end);
    println!("slice_code: {}", header.slice_code);
    println!("xyzt_units: {}", header.xyzt_units);
    println!("cal_max: {}", header.cal_max);
    println!("cal_min: {}", header.cal_min);
    println!("slice_duration: {}", header.slice_duration);
    println!("toffset: {}", header.toffset);
    println!("descrip: {:?}",  String::from_utf8_lossy(&header.descrip).replace('\0', "").trim());
    println!("aux_file: {:?}",  String::from_utf8_lossy(&header.aux_file).replace('\0', "").trim());
    println!("qform_code: {}", header.qform_code);
    println!("sform_code: {}", header.sform_code);
    println!("quatern_b: {}", header.quatern_b);
    println!("quatern_c: {}", header.quatern_c);
    println!("quatern_d: {}", header.quatern_d);
    println!("qoffset_x: {}", header.qoffset_x);
    println!("qoffset_y: {}", header.qoffset_y);
    println!("qoffset_z: {}", header.qoffset_z);
    print!("srow_x: ");
    for i in 0..header.srow_x.len() {
        print!("{} ", header.srow_x[i]);
    }
    println!();
    print!("srow_y: ");
    for i in 0..header.srow_y.len() {
        print!("{} ", header.srow_y[i]);
    }
    println!();
    print!("srow_z: ");
    for i in 0..header.srow_z.len() {
        print!("{} ", header.srow_z[i]);
    }
    println!();
    println!("intent_name: {}",String::from_utf8_lossy(&header.intent_name).replace('\0', "").trim());
    println!("magic: {}", String::from_utf8_lossy(&header.magic).replace('\0', "").trim());
}
