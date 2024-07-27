//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // rustc-env=KEY=VALUE，这个命令会将KEY=VALUE设置为环境变量，这个环境变量只在编译时有效，编译完成后就会被清除。
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo::{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // #[cfg(feature = " ")]是条件编译属性，允许在编译是根据和特定条件排除代码
    // 使用rustc-cfg=feature=""命令，可以在编译时启用某特性
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo::{}", your_command);
}
