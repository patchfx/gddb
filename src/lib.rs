//! # GDDB
//!
//! GDDB is a superfast in-memory database designed for use in Godot.
//!
//! This database aims to provide an easy frontend to an efficient in-memory database, that can be saved and reloaded.
//!
//! GDDB saves a Godot dictionary and provides an interface to create, update, retrieve (either single results or all items matching the search) and destroy records.
//!
//! GDDB started as a fork of [TinyDB](https://github.com/Owez/tinydb) with added functionality and a Godot wrapper.
//!
//! - [Documentation](https://docs.rs/gddb)
//! - [Crates.io](https://crates.io/crates/gddb)
//!
//! ## Rust Example 🚀
//!
//! An example of utilising GDDB within your Rust library.
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use gddb::Database;
//!
//! #[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
//! struct PlayerStruct {
//!     name: String
//! }
//!
//! fn main() {
//!     let player = PlayerStruct { name: "Joe Bloggs".into() };
//!     let mut db = Database::new("GAME", None, false);
//!
//!     db.create(my_struct.clone());
//!
//!     let results = db.find(|s: &PlayerStruct| &s.name, "Joe Bloggs".into());
//!
//!     assert_eq!(results.unwrap(), &player);
//! }
//! ```
//!
//! # Installation
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! gddb = "0.1.0"
//! ```
//! # Implementation notes
//!
//! - This database does not save 2 duplicated items, either ignoring or raising an
//! error depending on end-user preference.
//! - This project is not intended to be used inside of any critical systems due to
//! the nature of dumping/recovery. If you are using this crate as a temporary and
//! in-memory only database, it should preform at a reasonable speed (as it uses
//! [HashSet] underneath).
//!
//! # Essential operations
//!
//! Some commonly-used operations for the [Database] structure.
//!
//! | Operation                               | Implamentation          |
//! |-----------------------------------------|-------------------------|
//! | Create database                         | [Database::new]         |
//! | Create database from file               | [Database::from]        |
//! | Load database or create if non-existant | [Database::auto_from]   |
//! | Query all matching items                | [Database::query]       |
//! | Query for item                          | [Database::find]  |
//! | Contains specific item                  | [Database::contains]    |
//! | Update/replace item                     | [Database::update] |
//! | Delete item                             | [Database::destroy] |
//! | Dump database                           | [Database::dump_db]     |

pub mod database;
pub mod error;
pub mod gddb;
pub mod record;
use gdnative::prelude::*;

mod prelude {
    pub use crate::database::*;
    pub use crate::error::*;
    pub use crate::gddb::*;
    pub use crate::record::*;

    pub use core::fmt::Display;
    pub use hashbrown::HashSet;
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use snailquote::unescape;
    pub use std::fs::File;
    pub use std::hash;
    pub use std::io::prelude::*;
    pub use std::path::PathBuf;
    pub use uuid::Uuid;
}

use prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<GDDB>();
}

godot_init!(init);
