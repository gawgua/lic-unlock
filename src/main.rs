use libloading::{Library, Symbol};
use md5;
use std::{env::args, ffi, os::windows::ffi::OsStrExt, process::Command, path::Path};

fn main() {
    let window_id = cal_window_id();
    let path = args().nth(1).unwrap_or_else(|| {
        println!(".exe <path-to-pdf>");
        std::process::exit(1);
    });
    // #OCB#{cal_window_id()}#{path}
    let key = format!("#OCB#{}#{}", window_id, path);

    // Load the MuPDF DLL
    let mupdf_dll = unsafe {
        Library::new("mupdf-exp-dll-x86.dll").unwrap_or_else(|err| {
            println!("Failed to load mupdf-exp-dll-x86.dll {}", err);
            std::process::exit(1);
        })
    };

    // Function signatures
    let fz_new_context_imp: Symbol<
        unsafe extern "C" fn(
            *mut ffi::c_void,
            *mut ffi::c_void,
            u32,
            *const ffi::c_char,
        ) -> *mut ffi::c_void,
    > = unsafe {
        mupdf_dll.get(b"fz_new_context_imp").unwrap_or_else(|err| {
            println!("failed to load fz_new_context_imp: {}", err);
            std::process::exit(1);
        })
    };

    let fz_register_document_handlers: Symbol<
        unsafe extern "C" fn(*mut ffi::c_void) -> *mut ffi::c_void,
    > = unsafe {
        mupdf_dll
            .get(b"fz_register_document_handlers")
            .unwrap_or_else(|err| {
                println!("failed to load fz_register_document_handlers: {}", err);
                std::process::exit(1);
            })
    };

    let fz_open_document_w: Symbol<
        unsafe extern "C" fn(*mut ffi::c_void, *mut u16) -> *mut ffi::c_void,
    > = unsafe {
        mupdf_dll.get(b"fz_open_document_w").unwrap_or_else(|err| {
            println!("failed to load fz_open_document_w: {}", err);
            std::process::exit(1);
        })
    };

    let pdf_save_document: Symbol<
        unsafe extern "C" fn(
            *mut ffi::c_void,
            *mut ffi::c_void,
            *const ffi::c_char,
            *mut ffi::c_void,
        ),
    > = unsafe {
        mupdf_dll.get(b"pdf_save_document").unwrap_or_else(|err| {
            println!("failed to load pdf_save_document: {}", err);
            std::process::exit(1);
        })
    };

    // Decrytion process
    unsafe {
        let ctx = fz_new_context_imp(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            268435456u32,
            ffi::CString::new("1.16.1").unwrap().as_ptr(),
        );
        fz_register_document_handlers(ctx);
        let doc = fz_open_document_w(
            ctx,
            ffi::OsStr::new(&key)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect::<Vec<u16>>()
                .as_mut_ptr(),
        );
        let write_options = PDFWriteOptions::new();
        let write_options_ptr = &write_options as *const PDFWriteOptions;
        let out_path = format!("decrypted-{}", get_filename_from_path(&path));
        pdf_save_document(
            ctx,
            doc,
            ffi::CString::new(out_path.clone()).unwrap().as_ptr(),
            write_options_ptr as *mut ffi::c_void,
        );
        println!("Decrypted file saved to {}", out_path);
    }
}

fn get_csproduct_uuid() -> String {
    // Get the UUID of the machine
    // wmic csproduct get UUID
    let output = Command::new("wmic")
        .args(&["csproduct", "get", "UUID"])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout)
        .to_string()
        .lines()
        .nth(1)
        .unwrap_or("")
        .trim()
        .to_string()
}

fn cal_window_id() -> String {
    let text = format!("{}WINDOWID", get_csproduct_uuid().replace("-", ""));
    let hash = md5::compute(text);

    format!("{:x}", hash).split_at(16).0.to_string()
}

fn get_filename_from_path(path: &str) -> &str {
	let path = Path::new(path);
	path.file_name()
		.and_then(|name| name.to_str())
		.unwrap_or("")
}

#[repr(C)]
struct PDFWriteOptions {
    do_incremental: ffi::c_int,
    do_pretty: ffi::c_int,
    do_ascii: ffi::c_int,
    do_compress: ffi::c_int,
    do_compress_images: ffi::c_int,
    do_compress_fonts: ffi::c_int,
    do_decompress: ffi::c_int,
    do_garbage: ffi::c_int,
    do_linear: ffi::c_int,
    do_clean: ffi::c_int,
    do_sanitize: ffi::c_int,
    do_appearance: ffi::c_int,
    do_encrypt: ffi::c_int,
    dont_regenerate_id: ffi::c_int,
    permissions: ffi::c_int,
    opwd_utf8: [ffi::c_char; 128],
    upwd_utf8: [ffi::c_char; 128],
    do_snapshot: ffi::c_int,
    do_preserve_metadata: ffi::c_int,
    do_use_objstms: ffi::c_int,
    compression_effort: ffi::c_int,
}

impl PDFWriteOptions {
    fn new() -> Self {
        Self {
            do_incremental: 0,
            do_pretty: 0,
            do_ascii: 0,
            do_compress: 0,
            do_compress_images: 0,
            do_compress_fonts: 0,
            do_decompress: 0,
            do_garbage: 0,
            do_linear: 0,
            do_clean: 0,
            do_sanitize: 0,
            do_appearance: 0,
            do_encrypt: 1,
            dont_regenerate_id: 0,
            permissions: -1,
            opwd_utf8: [0; 128],
            upwd_utf8: [0; 128],
            do_snapshot: 0,
            do_preserve_metadata: 0,
            do_use_objstms: 0,
            compression_effort: 0,
        }
    }
}
