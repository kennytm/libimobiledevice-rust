extern crate pkg_config;

fn main() {
    pkg_config::probe_library("libimobiledevice-1.0").unwrap();
}


