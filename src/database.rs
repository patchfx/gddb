use crate::prelude::*;
use core::fmt::Display;
use hashbrown::HashSet;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::File;
use std::hash;
use std::io::prelude::*;
use std::path::PathBuf;
use uuid::Uuid;

pub trait RecordCheck: PartialEq + Default + Display {}
impl<T> RecordCheck for T where T: PartialEq + Default + Display {}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Record {
    pub uuid: String,
    pub model: String,
    pub attributes: String,
}

impl Record {
    pub fn new(model: String) -> Self {
        let uuid = Uuid::new_v4().to_string();

        Self {
            uuid: uuid.clone(),
            model,
            attributes: "".into(),
        }
    }
}

/// The primary database structure, allowing storage of a generic type with
/// dumping/saving options avalible.
///
/// The generic type used should primarily be structures as they resemble a
/// conventional database model and should implament [hash::Hash] and [Eq] for
/// basic in-memory storage with [Serialize] and [Deserialize] being implamented
/// for file operations involving the database (these are also required).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Database<T: hash::Hash + Eq> {
    /// Friendly name for the database, preferibly in `slug-form-like-this` as
    /// this is the fallback path
    ///
    /// This is used when dumping the database without a [Database::save_path]
    /// being defined and a friendly way to order a database
    pub label: String,

    /// The overwrite path to save the database as, this is recommended otherwise
    /// it will end up as `./Hello\ There.gddb` if [Database::label] is "Hello
    /// There"
    ///
    /// Primarily used inside of [Database::dump_db].
    pub save_path: Option<PathBuf>,

    /// If the database should return an error if it tries to insert where an
    /// identical item already is. Setting this as `false` doesn't allow
    /// duplicates, it just doesn't flag an error.
    pub strict_dupes: bool,

    /// In-memory [HashSet] of all items
    pub items: HashSet<T>,
}

impl<Record: hash::Hash + Eq + Serialize + DeserializeOwned> Database<Record> {
    /// Creates a new database instance from given parameters.
    ///
    /// - To add a first item, use [Database::create].
    /// - If you'd like to load a dumped database, use [Database::from]
    pub fn new(
        label: impl Into<String>,
        save_path: impl Into<Option<PathBuf>>,
        strict_dupes: bool,
    ) -> Self {
        Database {
            label: label.into(),
            save_path: save_path.into(),
            strict_dupes,
            items: HashSet::new(),
        }
    }

    /// Creates a database from a `.gddb` file.
    ///
    /// This retrives a dump file (saved database) from the path given and loads
    /// it as the [Database] structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gddb::Database;
    /// use serde::{Serialize, Deserialize};
    /// use std::path::PathBuf;
    ///
    ///
    /// /// Makes a small testing database.
    /// fn make_db() {
    ///     let mut test_db: Database<Record> = Database::new("test", None, false);
    ///     test_db.create(Record::new("Test".into()));
    ///     test_db.dump_db();
    /// }
    ///
    /// /// Get `test_db` defined in [make_db] and test.
    /// fn main() {
    ///     make_db();
    ///
    ///     let db = Database::from(
    ///         PathBuf::from("test.gddb")
    ///     ).unwrap();
    ///
    ///     assert_eq!(
    ///         db.len(),
    ///         1
    ///     ); // Check that the database still has added [ExampleStruct].
    /// }
    /// ```
    pub fn from(path: impl Into<PathBuf>) -> Result<Self, DatabaseError> {
        let stream = get_stream_from_path(path.into())?;
        let decoded: Database<Record> = bincode::deserialize(&stream[..]).unwrap();

        Ok(decoded)
    }

    /// Adds a new item to the in-memory database.
    ///
    /// If this is the first item added to the database, please ensure it's the
    /// only type you'd like to add. Due to generics, the first item you add
    /// will be set as the type to use (unless removed).
    pub fn create(&mut self, item: Record) -> Result<(), DatabaseError> {
        if self.strict_dupes {
            if self.items.contains(&item) {
                return Err(DatabaseError::DupeFound);
            }
        }

        self.items.insert(item);
        return Ok(());
    }

    /// Replaces an item inside of the database with another
    /// item, used for updating/replacing items easily.
    ///
    /// [Database::update] can be used in conjunction to find and replace
    /// values individually if needed.
    pub fn update(&mut self, item: &Record, new: Record) -> Result<(), DatabaseError> {
        self.destroy(item)?;
        self.create(new)?;

        Ok(())
    }

    /// Loads database from existant path or creates a new one if it doesn't already
    /// exist.
    ///
    /// This is the recommended way to use gddb if you are wanting to easily
    /// setup an entire database instance in a short, consise manner. Similar to
    /// [Database::new] and [Database::from], this function will also have to be
    /// given a strict type argument and you will still have to provide `script_dupes`
    /// even if the database is likely to load an existing one.
    ///
    /// This function does make some assumptions about the database name and uses
    /// the 2nd to last part before a `.`. This means that `x.y.z` will have the
    /// name of `y`, not `x` so therefore it is recommended to have a database
    /// path with `x.gddb` or `x.db` only.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gddb::*;
    /// use std::path::PathBuf;
    /// use serde::{Serialize, Deserialize};
    ///
    /// fn main() {
    ///     let dummy_db: Database<Record> = Database::new("cool", None, false); // create demo db for `db_from`
    ///
    ///     let db_from_path = PathBuf::from("cool.gddb");
    ///     let db_from: Database<Record> = Database::auto_from(db_from_path, false).unwrap(); // automatically load it
    ///
    ///     let db_new_path = PathBuf::from("xyz.gddb");
    ///     let db_new: Database<Record> = Database::auto_from(db_new_path, false).unwrap(); // automatically create new as "xyz" doesn't exist
    /// }
    /// ```
    pub fn auto_from(path: impl Into<PathBuf>, strict_dupes: bool) -> Result<Self, DatabaseError> {
        let path_into = path.into();

        if path_into.exists() {
            Database::from(path_into)
        } else {
            let db_name = match path_into.file_stem() {
                Some(x) => match x.to_str() {
                    Some(y) => String::from(y),
                    None => return Err(DatabaseError::BadDbName),
                },
                None => return Err(DatabaseError::BadDbName),
            };

            Ok(Database::new(db_name, Some(path_into), strict_dupes))
        }
    }

    /// Removes an item from the database.
    ///
    /// See [Database::update] if you'd like to update/replace an item easily,
    /// rather than individually deleting and adding.
    ///
    /// # Errors
    ///
    /// Will return [DatabaseError::ItemNotFound] if the item that is attempting
    /// to be deleted was not found.
    pub fn destroy(&mut self, item: &Record) -> Result<(), DatabaseError> {
        if self.items.remove(item) {
            Ok(())
        } else {
            Err(DatabaseError::ItemNotFound)
        }
    }

    /// Dumps/saves database to a binary file.
    ///
    /// # Saving path methods
    ///
    /// The database will usually save as `\[label\].gddb` where `\[label\]`
    /// is the defined [Database::label] (path is reletive to where gddb was
    /// executed).
    ///
    /// You can also overwrite this behaviour by defining a [Database::save_path]
    /// when generating the database inside of [Database::new].
    pub fn dump_db(&self) -> Result<(), DatabaseError> {
        let mut dump_file = self.open_db_path()?;
        bincode::serialize_into(&mut dump_file, self).unwrap();

        Ok(())
    }

    /// Query the database for a specific item.
    ///
    /// # Syntax
    ///
    /// ```none
    /// self.find(|[p]| [p].[field], [query]);
    /// ```
    ///
    /// - `[p]` The closure (Will be whatever the database currently is saving as a schema).
    /// - `[field]` The exact field of `p`. If the database doesn't contain structures, don't add the `.[field]`.
    /// - `[query]` Item to query for. This is a generic and can be of any reasonable type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use gddb::Database;
    ///
    /// #[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
    /// struct ExampleStruct {
    ///     my_age: i32
    /// }
    ///
    /// fn main() {
    ///     let my_struct = ExampleStruct { my_age: 329 };
    ///     let mut my_db = Database::new("query_test", None, false);
    ///
    ///     my_db.create(my_struct.clone());
    ///
    ///     let results = my_db.find(|s: &ExampleStruct| &s.my_age, 329);
    ///
    ///     assert_eq!(results.unwrap(), &my_struct);
    /// }
    /// ```
    pub fn find<Q: RecordCheck, V: Fn(&Record) -> &Q>(
        &self,
        value: V,
        query: Q,
    ) -> Result<&Record, DatabaseError> {
        for item in self.items.iter() {
            if value(item).eq(&query) {
                return Ok(item);
            }
        }

        Err(DatabaseError::ItemNotFound)
    }

    /// Query the database for all matching items.
    ///
    /// # Syntax
    ///
    /// ```none
    /// self.query(|[p]| [p].[field], [query]);
    /// ```
    ///
    /// - `[p]` The closure (Will be whatever the database currently is saving as a schema).
    /// - `[field]` The exact field of `p`. If the database doesn't contain structures, don't add the `.[field]`.
    /// - `[query]` Item to query for. This is a generic and can be of any reasonable type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use gddb::Database;
    ///
    /// #[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
    /// struct ExampleStruct {
    ///     uuid: String,
    ///     age: i32,
    /// }
    ///
    /// fn main() {
    ///     let mut my_db = Database::new("query_test", None, false);
    ///
    ///     my_db.create(ExampleStruct { uuid: "test1".into(), age: 20 });
    ///     my_db.create(ExampleStruct { uuid: "test2".into(), age: 20 });
    ///     my_db.create(ExampleStruct { uuid: "test3".into(), age: 18 });
    ///
    ///     let results = my_db.query(|s: &ExampleStruct| &s.age, 20);
    ///
    ///     assert_eq!(results.unwrap().len(), 2);
    /// }
    /// ```
    pub fn query<Q: PartialEq, V: Fn(&Record) -> &Q>(
        &self,
        value: V,
        query: Q,
    ) -> Result<Vec<&Record>, DatabaseError> {
        let mut items: Vec<&Record> = vec![];
        for item in self.items.iter() {
            if value(item) == &query {
                items.push(item);
            }
        }

        if items.len() > 0 {
            return Ok(items);
        }

        Err(DatabaseError::ItemNotFound)
    }

    /// Searches the database for a specific value. If it does not exist, this
    /// method will return [DatabaseError::ItemNotFound].
    ///
    /// This is a wrapper around [HashSet::contains].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gddb::Database;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
    /// struct ExampleStruct {
    ///     item: i32
    /// }
    ///
    /// fn main() {
    ///     let exp_struct = ExampleStruct { item: 4942 };
    ///     let mut db = Database::new("Contains example", None, false);
    ///
    ///     db.create(exp_struct.clone());
    ///
    ///     assert_eq!(db.contains(&exp_struct), true);
    /// }
    /// ```
    pub fn contains(&self, query: &Record) -> bool {
        self.items.contains(query)
    }

    /// Returns the number of database entries
    /// method will return i32.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gddb::Database;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
    /// struct ExampleStruct {
    ///     item: i32
    /// }
    ///
    /// fn main() {
    ///     let exp_struct = ExampleStruct { item: 4942 };
    ///     let mut db = Database::new("Contains example", None, false);
    ///
    ///     db.create(exp_struct.clone());
    ///
    ///     assert_eq!(db.len(), 1);
    /// }
    /// ```
    pub fn len(&self) -> i32 {
        self.items.len() as i32
    }

    /// Opens the path given in [Database::save_path] (or auto-generates a path).
    fn open_db_path(&self) -> Result<File, DatabaseError> {
        let definate_path = self.smart_path_get();

        if definate_path.exists() {
            std::fs::remove_file(&definate_path)?;
        }

        Ok(File::create(&definate_path)?)
    }

    /// Automatically allocates a path for the database if [Database::save_path]
    /// is not provided. If it is, this function will simply return it.
    fn smart_path_get(&self) -> PathBuf {
        if self.save_path.is_none() {
            return PathBuf::from(format!("{}.gddb", self.label));
        }

        PathBuf::from(self.save_path.as_ref().unwrap())
    }
}

/// Reads a given path and converts it into a [Vec]<[u8]> stream.
fn get_stream_from_path(path: PathBuf) -> Result<Vec<u8>, DatabaseError> {
    if !path.exists() {
        return Err(DatabaseError::DatabaseNotFound);
    }

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests addition to in-memory db
    #[test]
    fn item_add() -> Result<(), DatabaseError> {
        let mut my_db = Database::new("Adding test", None, true);

        my_db.create(Record::new("Test".into()))?;

        Ok(())
    }

    /// Tests removal from in-memory db
    #[test]
    fn item_remove() -> Result<(), DatabaseError> {
        let mut my_db = Database::new("Removal test", None, true);

        let testing_struct = Record::new("Testing".into());

        my_db.create(testing_struct.clone())?;
        my_db.destroy(&testing_struct)?;

        Ok(())
    }

    #[test]
    fn item_update() -> Result<(), DatabaseError> {
        let mut db: Database<Record> = Database::new("Update test", None, true);

        let testing_struct = Record::new("Test".into());

        db.create(testing_struct.clone())?;

        let mut updated_struct = testing_struct.clone();
        updated_struct.attributes = "Testing".into();
        db.update(&testing_struct, updated_struct)?;
        let record = db.find(|f| &f.uuid, testing_struct.uuid)?;
        let attributes: String = "Testing".into();
        assert_eq!(attributes, record.attributes);
        Ok(())
    }

    #[test]
    fn db_dump() -> Result<(), DatabaseError> {
        let mut my_db = Database::new(
            String::from("Dumping test"),
            Some(PathBuf::from("test.gddb")),
            true,
        );

        my_db.create(Record::new("Testing".into()))?;
        my_db.create(Record::new("Testing".into()))?;

        my_db.dump_db()?;

        Ok(())
    }
    /// Tests [Database::find]
    #[test]
    fn find_db() {
        let mut my_db = Database::new(
            String::from("Query test"),
            Some(PathBuf::from("test.gddb")),
            true,
        );

        let staging = Record::new("Staging".into());

        my_db.create(Record::new("Testing".into())).unwrap();
        my_db.create(staging.clone()).unwrap();
        my_db.create(Record::new("Production".into())).unwrap();

        assert_eq!(
            my_db.find(|f| &f.model, "Staging".into()).unwrap(),
            &staging
        ); // Finds "Staging" by searching [DemoStruct::model]
    }

    /// Tests [Database::query]
    #[test]
    fn query_db() {
        let mut my_db = Database::new(
            String::from("Query test"),
            Some(PathBuf::from("test.gddb")),
            false,
        );

        my_db.create(Record::new("Testing".into())).unwrap();
        my_db.create(Record::new("Testing".into())).unwrap();
        my_db.create(Record::new("Staging".into())).unwrap();

        assert_eq!(
            my_db.query(|f| &f.model, "Testing".into()).unwrap().len(),
            2
        ); // Finds "Testing" by searching [DemoStruct::model]
    }

    /// Tests a [Database::from] method call
    #[test]
    fn db_from() -> Result<(), DatabaseError> {
        let mut my_db = Database::new(
            String::from("Dumping test"),
            Some(PathBuf::from("test.gddb")),
            false,
        );

        let demo_mock = Record::new("Testing".into());

        my_db.create(demo_mock.clone()).unwrap();

        my_db.dump_db()?;

        let db: Database<Record> = Database::from(PathBuf::from("test.gddb"))?;
        assert_eq!(db.label, String::from("Dumping test"));

        Ok(())
    }

    /// Test if the database contains that exact item, related to
    /// [Database::contains].
    #[test]
    fn db_contains() {
        let exp_struct = Record::new("Testing".into());

        let mut db = Database::new(String::from("Contains example"), None, false);
        db.create(exp_struct.clone()).unwrap();
        assert_eq!(db.contains(&exp_struct), true);
    }

    /// Tests [Database::auto_from]'s ability to create new databases and fetch
    /// already existing ones; an all-round test of its purpose.
    #[test]
    fn auto_from_creation() {
        let _dummy_db: Database<Record> = Database::new(String::from("alreadyexists"), None, false);

        let from_db_path = PathBuf::from("alreadyexists.gddb");
        let _from_db: Database<Record> = Database::auto_from(from_db_path, false).unwrap();

        let new_db_path = PathBuf::from("nonexistant.gddb");
        let _net_db: Database<Record> = Database::auto_from(new_db_path, false).unwrap();
    }

    /// Tests [Database::len] returns the number of database entries
    #[test]
    fn len() {
        let mut db: Database<Record> = Database::new(
            String::from("Query test"),
            Some(PathBuf::from("test.gddb")),
            true,
        );

        let demo_mock = Record::new("Testing".into());

        db.create(demo_mock.clone()).unwrap();

        assert_eq!(db.len(), 1);
    }
}
