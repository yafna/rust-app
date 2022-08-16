use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Square {
    id: i64,
    nhash: i64,
    x: u32,
    y: u32,
}

pub fn connect() -> Connection {
    Connection::open("n_data.db").unwrap()
}

fn create_table(conn: Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE square (
            id    INTEGER PRIMARY KEY,
            nhash  INTEGER NOT NULL,
            X  INTEGER NOT NULL,
            Y INTEGER NOT NULL
        )",
        (),
    )?;
    Ok(())
}

pub fn insert_square(conn: Connection, me: Square) -> Result<()> {
    conn.execute(
        "INSERT INTO square (nhash, x, y) VALUES (?1, ?2, ?3)",
        (&me.nhash, &me.x, &me.y),
    )?;
    Ok(())
}

pub fn insert_square_data(conn: Connection, nhash: i64, x:i32, y:i32) -> Result<()> {
    conn.execute(
        "INSERT INTO square (nhash, x, y) VALUES (?1, ?2, ?3)",
        (nhash, x, y),
    )?;
    Ok(())
}

pub fn get_squares(conn: Connection) -> Result<Vec<Square>> {
    let mut squares: Vec<Square> = Vec::new();
    let mut stmt = conn.prepare("SELECT id, nhash, x, y FROM square")?;
    let item_iter = stmt.query_map([], |row| {
        Ok(Square {
            id: row.get(0)?,
            nhash: row.get(1)?,
            x: row.get(2)?,
            y: row.get(3)?,
        })
    })?;

    for item in item_iter {
        squares.push(item.unwrap());
    };
    Ok(squares)
}


#[cfg(test)]
#[test]
fn test_connect() {
    connect();
    // create_table(connect());
}

fn connect_in_memory_create_paste() -> Result<()> {
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
            y: row.get(3)?,
        })
    })?;

    for item in item_iter {
        println!("Found item {:?}", item.unwrap());
    }
    Ok(())
}

#[test]
fn test_connect_in_memory_create_paste() {
    connect_in_memory_create_paste().expect("test panic message");
}