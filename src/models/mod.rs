use serde_derive::{Serialize,Deserialize};
use postgres::rows::Row;

#[derive(Debug, Serialize)]
pub struct System {
    pub sysnr: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct Terminplan {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct Equipment {
    pub strukturnr: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meldung {
    pub tanr: String,
    pub beschreibung: String,
    pub platz: String,
    pub status: String,
}

impl Meldung {
    pub fn sql() -> &'static str { "select turnaround_nr_, beschreibung, verantw_arbeitspl_, meld_userstatus from ex_ziw28 order by 1" }
    pub fn from_row(row: Row) -> Meldung {
        Meldung {
            tanr: row.get(0),
            beschreibung: row.get(1),
            platz: row.get(2),
            status: row.get(3),
        }
    }
}