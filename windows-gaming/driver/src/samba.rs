use std::fs::{create_dir, File};
use std::path::Path;
use std::io::Write;
use std::fmt::Write as FmtWrite;

use users::get_user_by_name;
use nix::unistd::chown;

use common::config::SambaConfig;

pub fn is_installed() -> bool {
    Path::new("/usr/sbin/smbd").is_file()
}

pub fn setup(tmp: &Path, samba: &SambaConfig, usernet: &mut String) {
    //Samba still needs to be installed on the host OS
    assert!(is_installed(), "Optional samba dependency not installed!");

    write!(usernet, ",smb={}", samba.path).expect("Failed to append samba config to network adapter");

}
