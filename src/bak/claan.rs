#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type=crate::schema::sql_types::Claan)]
pub enum Claan {
    EarthStriders,
    FireDancers,
    ThunderWalkers,
    WaveRiders,
}

impl ToSql<crate::schema::sql_types::Claan, Pg> for Claan {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Claan::EarthStriders => out.write_all(b"EARTH_STRIDERS")?,
            Claan::FireDancers => out.write_all(b"FIRE_DANCERS")?,
            Claan::ThunderWalkers => out.write_all(b"THUNDER_WALKERS")?,
            Claan::WaveRiders => out.write_all(b"WAVE_RIDERS")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Claan, Pg> for Claan {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"EARTH_STRIDERS" => Ok(Claan::EarthStriders),
            b"FIRE_DANCERS" => Ok(Claan::FireDancers),
            b"THUNDER_WALKERS" => Ok(Claan::ThunderWalkers),
            b"WAVE_RIDERS" => Ok(Claan::WaveRiders),
            _ => Err("Unrecognized enum variant for Claan".into()),
        }
    }
}
