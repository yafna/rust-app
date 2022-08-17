use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Square {
    id: i64,
    ind_value: i64,
    x: u32,
    y: u32,
}

pub fn connect() -> Connection {
    Connection::open("n_data.db").unwrap()
}

fn create_tables(conn: Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE square (
	        id	INTEGER NOT NULL,
	        legend_ind_value	INTEGER NOT NULL,
        	x	INTEGER NOT NULL,
        	y	INTEGER NOT NULL,
        	PRIMARY KEY(id)
            );",
        (),
    )?;
    conn.execute(
        "CREATE TABLE legend (
            id    INTEGER NOT NULL,
            ind_value INTEGER NOT NULL,
            r INTEGER NOT NULL,
            g INTEGER NOT NULL,
            b INTEGER NOT NULL,
            PRIMARY KEY(id)
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE grid (
            id   INTEGER NOT NULL,
            legend_ind_value INTEGER NOT NULL,
            value INTEGER NOT NULL,
            x  INTEGER NOT NULL,
            y INTEGER NOT NULL,
            PRIMARY KEY(id)
        )",
        (),
    )?;
    Ok(())
}

pub fn insert_square(conn: &Connection, me: Square) -> Result<()> {
    conn.execute(
        "INSERT INTO square (ind_value, x, y) VALUES (?1, ?2, ?3)",
        (&me.ind_value, &me.x, &me.y),
    )?;
    Ok(())
}

pub fn insert_grid(conn: &Connection, legend_ind_value: usize, value: i64, x: usize, y: usize) -> Result<()> {
    conn.execute(
        "INSERT INTO grid (legend_ind_value, value, x, y) VALUES (?1, ?2, ?3, ?4)",
        (&legend_ind_value, &value, &x, &y),
    )?;
    Ok(())
}

pub fn insert_legend(conn: &Connection, ind_value: usize, r: u8, g: u8, b: u8) -> Result<()> {
    conn.execute(
        "INSERT INTO legend (ind_value, r, g, b) VALUES (?1, ?2, ?3, ?4)",
        (&ind_value, &r, &g, &b),
    )?;
    Ok(())
}

pub fn insert_square_data(conn: &Connection, ind_value: usize, x: u32, y: u32) -> Result<()> {
    conn.execute(
        "INSERT INTO square (legend_ind_value, x, y) VALUES (?1, ?2, ?3)",
        (&ind_value, &x, &y),
    )?;
    Ok(())
}

pub fn get_squares(conn: &Connection) -> Result<Vec<Square>> {
    let mut squares: Vec<Square> = Vec::new();
    let mut stmt = conn.prepare("SELECT id, ind_value, x, y FROM square")?;
    let item_iter = stmt.query_map([], |row| {
        Ok(Square {
            id: row.get(0)?,
            ind_value: row.get(1)?,
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
            ind_value  INTEGER NOT NULL,
            X  INTEGER NOT NULL,
            Y INTEGER NOT NULL
        )",
        (),
    )?;

    let me = Square {
        id: 0,
        ind_value: 254555242,
        x: 1,
        y: 3,
    };
    conn.execute(
        "INSERT INTO square (ind_value, x, y) VALUES (?1, ?2, ?3)",
        (&me.ind_value, &me.x, &me.y),
    )?;

    conn.execute(
        "INSERT INTO square (ind_value, x, y) VALUES (?1, ?2, ?3)",
        (&me.ind_value, &me.x, &me.y),
    )?;

    let mut stmt = conn.prepare("SELECT id, ind_value, x, y FROM square")?;
    let item_iter = stmt.query_map([], |row| {
        Ok(Square {
            id: row.get(0)?,
            ind_value: row.get(1)?,
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