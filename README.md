# GDDB

GDDB is a superfast in-memory database designed for use in Godot.

This database aims to provide an easy frontend to an efficient in-memory database, that can be saved and reloaded.

GDDB saves a Godot dictionary and provides an interface to create, update, retrieve (either single results or all items matching the search) and destroy records.

GDDB started as a fork of [TinyDB](https://github.com/Owez/tinydb) with added functionality and a Godot wrapper.

- [Documentation](https://docs.rs/gddb)
- [Crates.io](https://crates.io/crates/gddb)

## Rust Example 🚀

An example of utilising GDDB within your Rust library.

```rust
use serde::{Serialize, Deserialize};
use gddb::Database;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
struct PlayerStruct {
    name: String
}

fn main() {
    let player = PlayerStruct { name: "Joe Bloggs".into() };
    let mut db = Database::new("GAME", None, false);

    db.create(player.clone());

    let results = db.find(|s: &PlayerStruct| &s.name, "Joe Bloggs".into());

    assert_eq!(results.unwrap(), &player);
}
```

## Godot Example

- Copy the libgddb.(dll|so) to your Godot project
- Create a new GDNativeLibrary and link to the lib 
- Create a new GDNativeScript filed with a class name of 'Database'
- Attach the GDNativeLibrary to the GDNativeScript
- Autoload the GDNativeScript

```gdscript
extends Node

func _ready():
	var data = { "name": "Joe Bloggs" }
	var player_uuid = Database.create("Player", data)
	print(player_uuid)

	var record = Database.find(player_uuid)
	print(record.name)
```
## Installation

Simply add the following to your `Cargo.toml` file:

```toml
[dependencies]
gddb = "0.1.0"
```
