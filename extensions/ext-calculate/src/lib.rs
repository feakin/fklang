use crate::inc_salsa::File;
use crate::inc_salsa::SourceProgram;
use crate::inc_salsa::sum;

pub mod inc_salsa;

// salsa should be in crate root.

#[salsa::jar(db = Db)]
pub struct Jar(File, SourceProgram, sum);

// ANCHOR: jar_db
pub trait Db: salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db

// ANCHOR: jar_db_impl
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db_impl