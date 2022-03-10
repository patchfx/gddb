# GDDB

[![crates.io](https://img.shields.io/crates/v/gddb.svg)](https://crates.io/crates/gddb)
[![Cross-compile](https://github.com/patchfx/gddb/actions/workflows/cross-compile.yml/badge.svg)](https://github.com/patchfx/gddb/actions/workflows/cross-compile.yml)
[![Documentation](https://docs.rs/gddb/badge.svg)](https://docs.rs/gddb)
[![Version](https://img.shields.io/badge/rustc-1.51+-lightgray.svg)](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html)
![License](https://img.shields.io/crates/l/gddb.svg)

GDDB is a superfast in-memory database designed for use in Godot.

This database aims to provide an easy frontend to an efficient in-memory database, that can be saved and reloaded.

GDDB saves a Godot dictionary and provides an interface to create, update, retrieve (either single results or all items matching the search) and destroy records.

GDDB started as a fork of [TinyDB](https://github.com/Owez/tinydb) with added functionality and a Godot wrapper.

- [Documentation](https://docs.rs/gddb)
- [Crates.io](https://crates.io/crates/gddb)

## Installation

- git clone https://github.com/patchfx/gddb.git
- cd gddb
- cargo build
- Copy the libgddb.(dll|so) to your Godot project
- Create a new GDNativeLibrary and link to the lib
- Create a new GDNativeScript filed with a class name of 'GDDB'
- Attach the GDNativeLibrary to the GDNativeScript
- Autoload the GDNativeScript

## Example

```gdscript
extends Node

func _ready():
	var data = { "name": "Joe Bloggs" }
	var player_uuid = Database.create("Player", data)
	print(player_uuid)

	var record = Database.find(player_uuid)
	print(record.name)

	record.name = "John Doe"
	Database.update(record.uuid, record.model, record.attributes)

	var updated = Database.find(player_uuid)
	print(updated.name)
```
