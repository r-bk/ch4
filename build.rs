use std::{env, path::Path};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use zyn::TokenStream;

#[path = "templates/main_template.rs"]
mod main_template;

fn main() {
    built::write_built_file().expect("built failed");
    gen_ch4_version();
    export_sysinfo();
    write_main();
}

fn export_sysinfo() {
    let na = Some("n/a".to_string());
    let mut name = na.clone();
    let mut os_version = na.clone();
    let mut cpu_vendor = na.clone();
    let mut cpu_brand = na.clone();

    if sysinfo::IS_SUPPORTED_SYSTEM {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        name = System::name().or_else(|| na.clone());
        os_version = System::long_os_version().or_else(|| na.clone());
        let cpu = system.cpus().first();
        cpu_vendor = cpu
            .map(|cpu| cpu.vendor_id().to_string())
            .or_else(|| na.clone());
        cpu_brand = cpu
            .map(|cpu| cpu.brand().to_string())
            .or_else(|| na.clone());
    }

    println!("cargo:rustc-env=CH4_SYSINFO_NAME={}", name.unwrap());
    println!(
        "cargo:rustc-env=CH4_SYSINFO_OS_VERSION={}",
        os_version.unwrap()
    );
    println!(
        "cargo:rustc-env=CH4_SYSINFO_CPU_VENDOR={}",
        cpu_vendor.unwrap()
    );
    println!(
        "cargo:rustc-env=CH4_SYSINFO_CPU_BRAND={}",
        cpu_brand.unwrap()
    );
}

fn gen_ch4_version() {
    let mut ch4_version = env::var("CARGO_PKG_VERSION").unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir);
    match built::util::get_repo_head(path) {
        Ok(Some((_, _, short_hash))) => {
            ch4_version = format!("{ch4_version} git:{short_hash}");
        }
        Ok(None) => {}
        Err(_) => {}
    }

    println!("cargo:rustc-env=CH4_VERSION={ch4_version}");
}

fn write_file(tokens: TokenStream, file_name: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let file_path = std::path::Path::new(&out_dir).join(file_name);
    let syntax_tree: zyn::syn::File =
        zyn::syn::parse2(tokens).expect("failed to parse generated tokens");
    let pretty = prettyplease::unparse(&syntax_tree);
    std::fs::write(&file_path, pretty).expect("failed to write file");
}

fn write_main() {
    for is_async in [true, false] {
        let file_name = if is_async {
            "async_main.rs"
        } else {
            "std_main.rs"
        };
        write_file(main_template::render(is_async), file_name);
    }
    println!("cargo:rerun-if-changed=templates/main_template.rs");
}
