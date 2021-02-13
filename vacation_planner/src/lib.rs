#[macro_use]
extern crate serde;
extern crate chrono;
extern crate futures;
extern crate thiserror;
extern crate reqwest;
extern crate log;

use chrono::NaiveDate;

use std::collections::{BTreeSet, HashMap};

mod error;
pub use error::Error;

mod api;
mod algo;

#[derive(PartialEq, Eq, Hash)]
pub enum Province
{
    AB,
    BC,
    MB,
    NB,
    NL,
    NS,
    NT,
    NU,
    ON,
    PE,
    QC,
    SK,
    YT,
}

impl Province
{
    fn from_api_id(id: &str) -> Result<Province, Error>
    {
        match id
        {
            "AB" => Ok(Province::AB),
            "BC" => Ok(Province::BC),
            "MB" => Ok(Province::MB),
            "NB" => Ok(Province::NB),
            "NL" => Ok(Province::NL),
            "NS" => Ok(Province::NS),
            "NT" => Ok(Province::NT),
            "NU" => Ok(Province::NU),
            "ON" => Ok(Province::ON),
            "PE" => Ok(Province::PE),
            "QC" => Ok(Province::QC),
            "SK" => Ok(Province::SK),
            "YT" => Ok(Province::YT),
            _ => Err(Error::ProvinceParse),
        }
    }
}

pub type CanadianHolidays = HashMap<Province, Vec<NaiveDate>>;

pub struct Planner
{
    holidays: CanadianHolidays,
    province: Province,
    num_vacation_days: u16,
    fixed_vacation_days: BTreeSet<NaiveDate>,
    start_date: NaiveDate,
}

impl Planner
{
    pub fn new(mut holidays: CanadianHolidays, province: Province) -> Planner
    {
        for (_, province_holidays) in &mut holidays {
            province_holidays.sort();
        }

        Planner {
            holidays,
            province,
            num_vacation_days: 0,
            fixed_vacation_days: BTreeSet::new(),
            start_date: chrono::Local::today().naive_local(),
        }
    }

    pub fn from_web(province: Province) -> Result<Planner, Error>
    {
        let api_response =
            reqwest::blocking::get("https://canada-holidays.ca/api/v1/holidays")?
            .text()?;

        let parsed_response = serde_json::from_str::<api::HolidaysResponse>(&api_response)?;


        let mut holidays = CanadianHolidays::new();

        for holiday in parsed_response.holidays {

            let date = NaiveDate::parse_from_str(
                &holiday.date,
                "%Y-%m-%d")?;

            for province in holiday.provinces {
                let province_key = Province::from_api_id(&province.id)?;
                let province_holidays = holidays.entry(province_key).or_insert_with(|| Vec::new());

                province_holidays.push(date);
            }
        }

        for (_, province_holidays) in &mut holidays {
            province_holidays.sort();
        }


        Ok(Planner {
            holidays,
            province,
            num_vacation_days: 0,
            fixed_vacation_days: BTreeSet::new(),
            start_date: chrono::Local::today().naive_local(),
        })
    }


    pub fn holidays(&self) -> &[NaiveDate]
    {
        &self.holidays[&self.province]
    }

    pub fn fixed_vacation_days(&self) -> impl Iterator<Item=&NaiveDate>
    {
        self.fixed_vacation_days.iter()
    }

    pub fn set_num_vacation_days(&mut self, num: u16) {
        self.num_vacation_days = num;
    }

    pub fn add_vacation_day(&mut self, date: NaiveDate) -> Result<(), Error>
    {
        if self.remaining_days() == 0 {
            return Err(Error::InsertionFail);
        }

        self.fixed_vacation_days.insert(date);

        Ok(())
    }

    pub fn set_start_date(&mut self, date: NaiveDate) {
        self.start_date = date;
    }

    pub fn suggested_vacation_weeks(&self) -> Vec<chrono::IsoWeek> {
        let mut merged_dates = self.holidays[&self.province].clone();
        merged_dates.extend(self.fixed_vacation_days.iter());
        algo::calculate_vacation_weeks(merged_dates.iter(), self.remaining_days(), &self.start_date)
    }

    fn remaining_days(&self) -> u16
    {
        self.num_vacation_days - (self.fixed_vacation_days.len() as u16)
    }
}
