extern crate reqwest;
extern crate vacation_planner;
extern crate chrono;
extern crate pretty_env_logger;

use std::error::Error;

use chrono::NaiveDate;

use vacation_planner::Planner;
use vacation_planner::Province;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut planner = Planner::from_web(Province::BC)?;
    planner.set_num_vacation_days(14);
    planner.add_vacation_day(NaiveDate::from_ymd(2021, 02, 02))?;
    planner.add_vacation_day(NaiveDate::from_ymd(2021, 03, 02))?;
    planner.set_start_date(NaiveDate::from_ymd(2021, 02, 16));
    println!("{:?}", planner.holidays());
    for day in planner.fixed_vacation_days() {
        print!("{} ", day);
    }
    println!();


    println!("{:?}", planner.suggested_vacation_weeks().iter().map(|isoweek| NaiveDate::from_isoywd(2021, isoweek.week(), chrono::Weekday::Mon)).collect::<Vec<_>>());

    Ok(())
}
