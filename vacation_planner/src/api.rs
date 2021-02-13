#[derive(Deserialize)]
pub struct Province
{
    pub id: String,
}

#[derive(Deserialize)]
pub struct Holiday
{
    pub date: String,
    pub provinces: Vec<Province>,
}

pub type Holidays = Vec<Holiday>;
#[derive(Deserialize)]
pub struct HolidaysResponse
{
    pub holidays: Holidays,
}
