//! Testing doctest with tarpaulin

#![warn(
  unreachable_pub,
  missing_docs,
  missing_debug_implementations
)]
#![doc(html_logo_url = "TODO.svg")]
#![doc(html_root_url = "https://localhost/nowhere/0.1.0")]
#![doc(issue_tracker_base_url = "NOWHERE")]

use std::env;
use std::path::PathBuf;

// doctest
/// Get my own package path for e.g. setting configuration from
/// ```rust
/// # use test_tarpaulin_env::*;
/// # use std::path::PathBuf;
/// #
/// # fn main() {
///     let test_dir = get_my_path("test_data");
///     assert_ne!(test_dir, PathBuf::from(""));
///     println!("test_dir evaluated to = lossy?:{}", test_dir.display());
/// # }
/// ```
// the below will panic! as bug when CARGO_MANIFEST_DIR cannot be unwrapped
pub fn get_my_path(test_dir_name: &str) -> PathBuf {
    let mut test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    test_dir.push("tests");
    test_dir.push("data");
    test_dir.push(test_dir_name);
    test_dir
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn cargo_manifest_path() {
    let test_dir = get_my_path("");
    println!("Test dir evaluated to = {}", test_dir.display());
    assert_ne!(test_dir, PathBuf::from(""));
  }
}
