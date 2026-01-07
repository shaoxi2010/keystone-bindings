#[cfg(feature = "build-from-src")]
use cmake;
#[cfg(feature = "use-system-lib")]
use pkg_config;

#[cfg(feature = "build-from-src")]
fn build_keystone() {
    let dest = cmake::Config::new("keystone")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("BUILD_LIBS_ONLY", "1")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("LLVM_TARGETS_TO_BUILD", "all")
        // Prevent python from leaving behind `.pyc` files which break `cargo package`
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=keystone");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
    }
}

#[cfg(feature = "fix-cmake")]
fn patch_keystone() {
    use std::{fs, path::Path};
    let llvm_cmake_path = Path::new("keystone/llvm/CMakeLists.txt");
    if llvm_cmake_path.exists() {
        let content = fs::read_to_string(llvm_cmake_path).unwrap();

        let new_content = content
            .replace(
                "cmake_minimum_required(VERSION 2.8.7)",
                "cmake_minimum_required(VERSION 3.10)",
            )
            .replace(
                "cmake_policy(SET CMP0051 OLD)",
                "cmake_policy(SET CMP0051 NEW)",
            );

        fs::write(llvm_cmake_path, new_content).unwrap();
        println!("cargo:warning=Patched LLVM CMakeLists.txt for build");
    }
    let keystone_cmake_path = Path::new("keystone/CMakeLists.txt");
    if keystone_cmake_path.exists() {
        let content = fs::read_to_string(keystone_cmake_path).unwrap();

        let new_content = content
            .replace(
                "cmake_minimum_required(VERSION 2.8.7)",
                "cmake_minimum_required(VERSION 3.10)",
            )
            .replace(
                "cmake_policy(SET CMP0051 OLD)",
                "cmake_policy(SET CMP0051 NEW)",
            );
        fs::write(keystone_cmake_path, new_content).unwrap();
        println!("cargo:warning=Patched Keystone CMakeLists.txt for build");
    }
}

#[cfg(feature = "use-system-lib")]
fn link_cpp() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
    }
}

fn main() {
    if cfg!(feature = "use-system-lib") {
        #[cfg(feature = "use-system-lib")]
        pkg_config::find_library("keystone").expect("Could not find system keystone");
        #[cfg(feature = "use-system-lib")]
        link_cpp();
    } else {
        #[cfg(feature = "fix-cmake")]
        patch_keystone();
        #[cfg(feature = "build-from-src")]
        build_keystone();
    }
}
