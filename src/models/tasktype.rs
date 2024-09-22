use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(sql_type = crate::database::schema::sql_types::Tasktype)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Tasktype {
    Quest,
    Activity,
}

impl ToSql<crate::database::schema::sql_types::Tasktype, Pg> for Tasktype {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Tasktype::Quest => out.write_all(b"quest")?,
            Tasktype::Activity => out.write_all(b"activity")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::database::schema::sql_types::Tasktype, Pg> for Tasktype {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"quest" => Ok(Tasktype::Quest),
            b"activty" => Ok(Tasktype::Activity),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
