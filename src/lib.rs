use rusqlite::{params, NO_PARAMS};

pub fn create_stl_db(
    conn: &mut rusqlite::Connection,
    index_stl: &nom_stl::IndexMesh,
) -> rusqlite::Result<()> {
    let triangles = index_stl.triangles();
    let vertices = index_stl.vertices();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vertices (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        x REAL,
        y REAL,
        z REAL
    )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS triangles (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        normal_x REAL,
        normal_y REAL,
        normal_z REAL,
        v1_id INTEGER,
        v2_id INTEGER,
        v3_id INTEGER
    )",
        NO_PARAMS,
    )?;

    let tx = conn.transaction()?;

    {
        let mut vertex_insert = tx.prepare(
            "INSERT INTO vertices (x, y, z)
                 VALUES (?1, ?2, ?3)",
        )?;

        let mut triangle_insert = tx.prepare(
            "INSERT INTO triangles (normal_x, normal_y, normal_z, v1_id, v2_id, v3_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for [x, y, z] in vertices {
            vertex_insert.execute(params![*x as f64, *y as f64, *z as f64])?;
        }

        for triangle in triangles {
            let [nx, ny, nz] = triangle.normal();
            let [i, j, k] = triangle.vertices_indices();

            triangle_insert.execute(params![
                nx as f64, ny as f64, nz as f64, i as i64, j as i64, k as i64
            ])?;
        }

        tx.execute(
            "CREATE INDEX IF NOT EXISTS triangles_v1_id
        ON triangles (v1_id)",
            NO_PARAMS,
        )?;

        tx.execute(
            "CREATE INDEX IF NOT EXISTS triangles_v2_id
        ON triangles (v2_id)",
            NO_PARAMS,
        )?;

        tx.execute(
            "CREATE INDEX IF NOT EXISTS triangles_v3_id
        ON triangles (v3_id)",
            NO_PARAMS,
        )?;
    }

    tx.commit()?;

    Ok(())
}
