//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
pub mod datum;
#[macro_use]
pub mod elog;
pub mod cmp;
pub mod errcodes;
pub mod ffi;
pub mod htup;
pub mod oids;
pub mod panic;
pub mod pg_try;
pub mod polyfill;
pub(crate) mod thread_check;
pub mod tupdesc;

pub mod utils;

// Various SqlTranslatable mappings for SQL generation
mod sql_translatable;

pub use datum::Datum;

pub use htup::*;
pub use oids::*;
pub use pg_try::*;
#[cfg(any(feature = "pg12", feature = "gp7"))]
pub use polyfill::*;
pub use utils::*;

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "__sigsetjmp"]
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "illumos"
))]
extern "C" {
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
