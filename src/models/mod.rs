use serde_derive::{Serialize, Deserialize};
use postgres::rows::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meldung {
    pub sapnr: String,
    pub status: String,
    pub system: String,
    pub tanr: String,
    pub kurztext: String,
    pub equipment: String,
    pub frequenz: String,
    pub terminplan: String,
    pub pid: String,
    pub datum: String,
    pub platz: String,
}

impl Meldung {
    pub fn sql() -> &'static str {
        r#"select
  meldung                   as sapnr,
  m.kurztext_codierung      as status,
  left(m.turnaround_nr_,4)  as system,
  m.turnaround_nr_          as tanr,
  m.beschreibung            as kurztext,
  m.technische_identnummer  as equipment,
  m.zusaetzl__bemerk_2      as frequenz,
  m.zusaetzl__bemerk_3      as terminplan,
  m.rui_schema_nr_          as pid,
  left(m.meldungsdatum,10)  as datum,
  m.verantw_arbeitspl_      as platz
from public.ex_ziw28 m
order by m.turnaround_nr_
        "#
    }

    pub fn from_row(row: Row) -> Meldung {
        Meldung {
            sapnr: row.get(0),
            status: row.get(1),
            system: row.get(2),
            tanr: row.get(3),
            kurztext: row.get(4),
            equipment: row.get(5),
            frequenz: row.get(6),
            terminplan: row.get(7),
            pid: row.get(8),
            datum: row.get(9),
            platz: row.get(10),
        }
    }
}