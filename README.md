# stlite

Turn an .stl file into a Sqlite database

## use

```
$ stlite MOON_PRISM_POWER.stl
$ sqlite3 MOON_PRISM_POWER.db
SQLite version 3.33.0 2020-08-14 13:23:32
Enter ".help" for usage hints.
sqlite> .schema
CREATE TABLE vertices (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        x REAL,
        y REAL,
        z REAL
    );
CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE triangles (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        normal_x REAL,
        normal_y REAL,
        normal_z REAL,
        v1_id INTEGER,
        v2_id INTEGER,
        v3_id INTEGER
    );
CREATE INDEX triangles_v1_id
        ON triangles (v1_id);
CREATE INDEX triangles_v2_id
        ON triangles (v2_id);
CREATE INDEX triangles_v3_id
        ON triangles (v3_id);
sqlite> select count(*) from triangles;
3698
sqlite> select count(*) from vertices;
1857
sqlite> select * from triangles limit 1;
1|0.642777025699615|-2.54043993663799e-06|0.766053020954132|0|1|2
sqlite> select * from vertices limit 1;
1|8.08660984039307|0.373288989067078|54.1924018859863
```

```
$ stlite -h
stlite 0.1.0
Turn an .stl file into a Sqlite database

USAGE:
    stlite <stl-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <stl-file>
```
