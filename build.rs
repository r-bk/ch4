use std::{env, path::Path, process::Command};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tera::{Context, Tera};

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
        let system =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
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

fn format_file(path: &std::path::Path) -> bool {
    let path_str = path.to_str().unwrap();
    Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(path_str)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn write_file(tera: &Tera, context: &Context, file_name: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let file_data = tera
        .render("main_template.rs", context)
        .expect("failed to render template");
    let file_path = std::path::Path::new(&out_dir).join(file_name);
    std::fs::write(&file_path, file_data).expect("failed to write file");
    format_file(&file_path);
}

fn write_main() {
    let tera = match Tera::new("templates/*.rs") {
        Ok(t) => t,
        Err(e) => {
            panic!("Tera parsing error(s): {e}");
        }
    };

    for async_value in &["true", "false"] {
        let mut context = Context::new();
        context.insert("async", async_value);
        let file_name = format!(
            "{}_main.rs",
            if *async_value == "true" {
                "async"
            } else {
                "std"
            }
        );
        write_file(&tera, &context, &file_name);
    }
}
