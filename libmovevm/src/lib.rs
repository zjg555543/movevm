#![cfg_attr(feature = "backtraces", feature(backtrace))]
#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

mod version;
mod adapt;
mod api;
mod call;

// We only interact with this crate via `extern "C"` interfaces, not those public
// exports. There are no guarantees those exports are stable.
// We keep them here such that we can access them in the docs (`cargo doc`).

use move_package::BuildConfig;
use move_core_types::{
    account_address::AccountAddress, errmap::ErrorMapping, identifier::Identifier,
};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_test_utils::gas_schedule::CostTable;
use std::path::PathBuf;

type NativeFunctionRecord = (AccountAddress, Identifier, Identifier, NativeFunction);


/// Default directory where saved Move resources live
pub const DEFAULT_STORAGE_DIR: &str = "storage";

/// Default directory for build output
pub const DEFAULT_BUILD_DIR: &str = ".";

/// Extension for resource and event files, which are in BCS format
const BCS_EXTENSION: &str = "bcs";

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Move {
    /// Path to a package which the command should be run with respect to.
    #[clap(long = "path", short = 'p', global = true, parse(from_os_str))]
    pub package_path: Option<PathBuf>,

    /// Print additional diagnostics if available.
    #[clap(short = 'v', global = true)]
    pub verbose: bool,

    /// Package build options
    #[clap(flatten)]
    pub build_config: BuildConfig,
}



