use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Square {
    id: i64,
    nhash: i64,
    x: u32,
    y: u32
}


fn connect_create_paste() -> Result<()>  {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE square (
            id    INTEGER PRIMARY KEY,
            nhash  INTEGER NOT NULL,
            X  INTEGER NOT NULL,
            Y INTEGER NOT NULL
        )",
        (),
    )?;

    let me = Square {
        id: 0,
        nhash: 254555242,
        x: 1,
        y: 3,
    };
    conn.execute(
        "INSERT INTO square (nhash, x, y) VALUES (?1, ?2, ?3)",
        (&me.nhash, &me.x, &me.y),
    )?;

    let mut stmt = conn.prepare("SELECT id, nhash, x, y FROM square")?;
    let item_iter = stmt.query_map([], |row| {
        Ok(Square {
            id: row.get(0)?,
            nhash: row.get(1)?,
            x: row.get(2)?,
            y: row.get(3)?
        })
    })?;

    for item in item_iter {
        println!("Found item {:?}", item.unwrap());
    }
    Ok(())
}


#[cfg(test)]
#[test]
fn test_connect_create_paste() {
    connect_create_paste().expect("TODO: panic message");
}