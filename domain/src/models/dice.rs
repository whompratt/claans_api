use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(sql_type = infrastructure::schema::sql_types::Dice)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
}

impl ToSql<infrastructure::schema::sql_types::Dice, Pg> for Dice {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Dice::D4 => out.write_all(b"d4")?,
            Dice::D6 => out.write_all(b"d6")?,
            Dice::D8 => out.write_all(b"d8")?,
            Dice::D10 => out.write_all(b"d10")?,
            Dice::D12 => out.write_all(b"d12")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<infrastructure::schema::sql_types::Dice, Pg> for Dice {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"d4" => Ok(Dice::D4),
            b"d6" => Ok(Dice::D6),
            b"d8" => Ok(Dice::D8),
            b"d10" => Ok(Dice::D10),
            b"d12" => Ok(Dice::D12),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
