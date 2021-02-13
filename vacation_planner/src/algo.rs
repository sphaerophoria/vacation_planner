use chrono::{Datelike, IsoWeek, NaiveDate};

use log::debug;

fn round(iter: impl IntoIterator<Item=f32>) -> Vec<u32> {
    let mut ret = Vec::new();

    let mut err = 0f32;

    for item in iter {
        let rounded = (item + err).round();
        err += item - rounded;
        ret.push(rounded as u32)
    }

    ret
}

pub fn calculate_vacation_weeks<'a>(input_dates: impl Iterator<Item=&'a NaiveDate>, remaining_days: u16, start_date: &NaiveDate) -> Vec<IsoWeek>
{
    // General strategy
    // * Assume that vacation days should be spaced by weeks, not days
    // * Discard all holidays except for the last one before the start date
    // * Calculate the ideal amount of time between all days off
    // * Calculate the time between each fixed day
    // * Calculate how much of the remaining time is spent between each two days off
    // * Calculate how much of the total remaining year is spent between each time between days off
    // * Divide up the remaining days into each window, number of days given is
    //   proportional to the time left in the year minus windows of fixed days
    //   that are too close together.
    //    * Note that days can only be given out as whole days, so we round and
    //      accumulate rounding error
    // * Divide up the days in each window, rounding to the nearest week and
    //   keeping track of rounding error
    //
    // Potential improvements:
    // * Where the fixed day falls in the week should be accounted for
    // * Option to allow larger gaps around areas with lots of days off
    // * Add an end date

    let mut fixed_days = input_dates.collect::<Vec::<&'a NaiveDate>>();
    fixed_days.sort();
    let (before_days, after_days): (Vec<&NaiveDate>, Vec<&NaiveDate>) = fixed_days.iter().partition(|date| **date < start_date);

    let relevant_fixed_days = if let Some(previous_day) = before_days.into_iter().last() {
        let mut days = vec![previous_day];
        days.extend(after_days.into_iter());
        days
    } else {
        after_days
    };

    let last_iso_week = NaiveDate::from_ymd(start_date.year(), 12, 31).iso_week().week();
    let mut start_iso_week = start_date.iso_week().week();
    if start_iso_week == 53 {
        start_iso_week = 0;
    }

    let num_remaining_weeks = last_iso_week - start_iso_week;

    let ideal_day_off_interval_weeks = num_remaining_weeks as f32 / (relevant_fixed_days.len() as f32 + remaining_days as f32);

    let mut fixed_day_delta_weeks: Vec<_> = relevant_fixed_days.windows(2).map(|window| window[1].iso_week().week() - window[0].iso_week().week()).collect();

    for week_delta in &mut fixed_day_delta_weeks {
        if (*week_delta as f32) < ideal_day_off_interval_weeks {
            *week_delta = 0;
        }
    }

    let total_week_delta: u32 = fixed_day_delta_weeks.iter().sum();

    let ideal_days_off_between_fixed: Vec<_> = fixed_day_delta_weeks.iter().map(|delta| *delta as f32 / total_week_delta as f32 * remaining_days as f32).collect();

    let days_off_between_fixed = round(ideal_days_off_between_fixed);

    debug!("start_date: {}\n\
        input_dates: {:?}\n\
        num_remaining_weeks: {}\n\
        ideal_weeks_off: {}\n\
        days_off: {}\n\
        relevant_fixed_days: {:?}\n\
        days_off_between_fixed: {:?}\n", start_date, fixed_days, num_remaining_weeks, ideal_day_off_interval_weeks, remaining_days, relevant_fixed_days, days_off_between_fixed);


    let mut ret = Vec::new();
    for (idx, window ) in relevant_fixed_days.windows(2).enumerate() {
        let start_week = window[0].iso_week().week();
        let end_week = window[1].iso_week().week();
        let week_diff = end_week - start_week;
        let num_days_off = days_off_between_fixed[idx];

        let ideal_interval = week_diff as f32 / (num_days_off + 1) as f32;

        let ideal_weeks_off: Vec<_> = (1..num_days_off + 1).map(|day_num| start_week as f32 + ideal_interval * day_num as f32).collect();
        let actual_weeks_off = round(ideal_weeks_off);

        for week in actual_weeks_off {
            ret.push(NaiveDate::from_isoywd(start_date.year(), week, chrono::Weekday::Mon).iso_week());
        }
    }

    ret
}

#[cfg(test)]
mod tests
{
    use chrono::Weekday;

    use super::*;

    #[test]
    fn round_test()
    {
        let input = vec![60.2f32, 15.4f32, 24.6f32];
        let output = round(input);
        assert_eq!(output, vec![60, 16, 24]);
    }

    #[test]
    fn simple_dates_test()
    {
        let input_dates = vec![NaiveDate::from_isoywd(2021, 1, Weekday::Mon), NaiveDate::from_isoywd(2021, 52, Weekday::Mon)];
        let weeks = calculate_vacation_weeks(input_dates.iter(), 1, &NaiveDate::from_ymd(2021, 1, 1));
        let weeks_u32: Vec<_> = weeks.into_iter().map(|week| week.week()).collect();
        assert_eq!(weeks_u32, vec![27]);
    }

    // FIXME: More tests to be written
}
