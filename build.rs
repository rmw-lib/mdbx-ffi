use bindgen::callbacks::{IntKind, ParseCallbacks};
use std::{env, path::PathBuf};

fn link_print(x: u8) {
    let directory = std::env::var("CARGO_MANIFEST_DIR").expect("TARGET couldn't be decoded.");
    println!("cargo:rustc-link-lib=dylib=ntdll");
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-search=native={}\\lib\\x{}", x, directory);
}

fn link() {
    let target = std::env::var("TARGET").expect("TARGET couldn't be decoded.");

    match target.as_ref() {
        "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
            link_print(64);
        }

        "i686-pc-windows-gnu" | "i686-pc-windows-msvc" => {
            link_print(86);
        }

        _ => {}
    }
}

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "MDBX_SUCCESS"
            | "MDBX_KEYEXIST"
            | "MDBX_NOTFOUND"
            | "MDBX_PAGE_NOTFOUND"
            | "MDBX_CORRUPTED"
            | "MDBX_PANIC"
            | "MDBX_VERSION_MISMATCH"
            | "MDBX_INVALID"
            | "MDBX_MAP_FULL"
            | "MDBX_DBS_FULL"
            | "MDBX_READERS_FULL"
            | "MDBX_TLS_FULL"
            | "MDBX_TXN_FULL"
            | "MDBX_CURSOR_FULL"
            | "MDBX_PAGE_FULL"
            | "MDBX_MAP_RESIZED"
            | "MDBX_INCOMPATIBLE"
            | "MDBX_BAD_RSLOT"
            | "MDBX_BAD_TXN"
            | "MDBX_BAD_VALSIZE"
            | "MDBX_BAD_DBI"
            | "MDBX_LOG_DONTCHANGE"
            | "MDBX_DBG_DONTCHANGE"
            | "MDBX_RESULT_TRUE"
            | "MDBX_UNABLE_EXTEND_MAPSIZE"
            | "MDBX_PROBLEM"
            | "MDBX_LAST_LMDB_ERRCODE"
            | "MDBX_BUSY"
            | "MDBX_EMULTIVAL"
            | "MDBX_EBADSIGN"
            | "MDBX_WANNA_RECOVERY"
            | "MDBX_EKEYMISMATCH"
            | "MDBX_TOO_LARGE"
            | "MDBX_THREAD_MISMATCH"
            | "MDBX_TXN_OVERLAPPING"
            | "MDBX_LAST_ERRCODE" => Some(IntKind::Int),
            _ => Some(IntKind::UInt),
        }
    }
}

fn main() {
    let mut mdbx = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    mdbx.push("libmdbx");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header(mdbx.join("mdbx.h").to_string_lossy())
        .allowlist_var("^(MDBX|mdbx)_.*")
        .allowlist_type("^(MDBX|mdbx)_.*")
        .allowlist_function("^(MDBX|mdbx)_.*")
        .rustified_enum("^(MDBX_option_t|MDBX_cursor_op)")
        .bitfield_enum("^(MDBX_constants|MDBX_log_level_t|MDBX_debug_flags_t|MDBX_env_flags_t|MDBX_txn_flags_t|MDBX_db_flags_t|MDBX_put_flags_t|MDBX_copy_flags_t|MDBX_env_delete_mode_t|MDBX_dbi_state_t|MDBX_page_type_t|MDBX_error_t)")
        .size_t_is_usize(true)
        .ctypes_prefix("::libc")
        .parse_callbacks(Box::new(Callbacks))
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate_comments(false)
        .disable_header_comment()
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut mdbx = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    mdbx.push("libmdbx");

    let mut builder = cc::Build::new();

    builder
        .file(mdbx.join("mdbx.c"))
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wbad-function-cast")
        .flag_if_supported("-Wuninitialized");

    let flags = format!("{:?}", builder.get_compiler().cflags_env());
    builder.define("MDBX_BUILD_FLAGS", flags.as_str());
    builder.define("MDBX_TXN_CHECKOWNER", "0");

    builder.compile("libmdbx.a");
    link()
}
