/*
 * Terra Mach
 * Copyright [2020] Terra Mach Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>
 */

use std::path::{PathBuf, Path};
use std::{env, fmt};
use std::fmt::{Display, Formatter};

pub fn warning(warn: impl AsRef<str>) {
    println!("cargo:warning={}", warn.as_ref());
}

pub fn output_directory() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn crate_directory() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

pub fn rerun_if_changed(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().to_str().unwrap());
}

pub fn env_var(name: impl AsRef<str>) -> Option<String> {
    let name = name.as_ref();
    rerun_if_env_changed(name);
    env::var(name).ok()
}

pub fn rerun_if_env_changed(name: impl AsRef<str>) {
    println!("cargo:rerun-if-env-changed={}", name.as_ref())
}

pub fn add_link_libs(libs: &[impl AsRef<str>]) {
    libs.iter().for_each(|s| add_link_lib(s.as_ref()))
}

pub fn add_link_lib(lib: impl AsRef<str>) {
    println!("cargo:rustc-link-lib={}", lib.as_ref());
}

#[derive(Clone, Debug)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn is_windows(&self) -> bool {
        self.system == "windows"
    }

    pub fn library_to_filename(&self, name: impl AsRef<str>) -> PathBuf {
        let name = name.as_ref();
        if self.is_windows() {
            format!("{}.lib", name).into()
        } else {
            format!("lib{}.a", name).into()
        }
    }

    pub fn as_strs(&self) -> (&str, &str, &str, Option<&str>) {
        (
            self.architecture.as_str(),
            self.vendor.as_str(),
            self.system.as_str(),
            self.abi.as_deref(),
        )
    }

    pub fn to_triplet(&self) -> String {
        format!("{}-{}-{}", self.architecture, self.vendor, self.system)
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            &self.architecture, &self.vendor, &self.system
        )?;

        if let Some(ref abi) = self.abi {
            write!(f, "-{}", abi)
        } else {
            Result::Ok(())
        }
    }
}

pub fn target() -> Target {
    let target_str = env::var("TARGET").unwrap();
    parse_target(target_str)
}

pub fn target_crt_static() -> bool {
    cfg!(target_feature = "crt-static")
}

pub fn host() -> Target {
    let host_str = env::var("HOST").unwrap();
    println!("HOST: {}", host_str);
    parse_target(host_str)
}

fn parse_target(target_str: impl AsRef<str>) -> Target {
    let target_str = target_str.as_ref();
    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();
    if target.len() < 3 {
        panic!("Failed to parse TARGET {}", target_str);
    }

    let abi = if target.len() > 3 {
        Some(target[3].clone())
    } else {
        None
    };

    Target {
        architecture: target[0].clone(),
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi,
    }
}

pub fn build_release() -> bool {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => true,
        "debug" => false,
        _ => panic!("PROFILE '{}' is not supported by this build script"),
    }
}
