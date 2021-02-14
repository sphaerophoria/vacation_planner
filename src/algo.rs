use chrono::{Duration, NaiveDate};

fn compute_times_between_days(days: &[NaiveDate]) -> Vec<Duration> {
    days
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

/// Remove fixed_days that do not effect vacation days after start_date
/// Includes 1 fixed_day before start_date since there could be days off to take
/// in the current window
fn get_relevant_fixed_days(fixed_days: Vec<NaiveDate>, start_date: &NaiveDate) -> Vec<NaiveDate> {
    let (before_days, after_days): (Vec<NaiveDate>, Vec<NaiveDate>) =
        fixed_days.iter().partition(|date| *date < start_date);

    if let Some(previous_day) = before_days.into_iter().last() {
        let mut days = vec![previous_day];
        days.extend(after_days.into_iter());
        days
    } else {
        after_days
    }
}

/// Computes how many days should go in each bucket. Iteratively assigns days to
/// each bucket, choosing the largest bucket on each iteration
fn compute_days_per_bucket(times_between_days: &[Duration], num_days: u16) -> Vec<i32> {
    let mut unsorted_days = num_days;
    let mut days_off_between_fixed = vec![0; times_between_days.len()];
    while unsorted_days > 0 {
        let idx = times_between_days
            .iter()
            .enumerate()
            .max_by(|&(a_idx, a), &(b_idx, b)| {
                (*a / (days_off_between_fixed[a_idx] + 1))
                    .cmp(&(*b / (days_off_between_fixed[b_idx] + 1)))
            })
            .map(|(idx, _)| idx)
            .unwrap();

        days_off_between_fixed[idx] += 1;
        unsorted_days -= 1;
    }

    days_off_between_fixed
}

/// Computes the ideal days off by dividing each window by the days in the bucket
fn compute_ideal_days_off(fixed_days_off: &[NaiveDate], days_per_bucket: &[i32]) -> Vec<NaiveDate> {
    let mut ret = Vec::new();
    for (idx, window) in fixed_days_off.windows(2).enumerate() {
        let num_days_between = (window[1] - window[0]) / (days_per_bucket[idx] + 1);

        for i in 1..days_per_bucket[idx] + 1{
            ret.push(window[0] + num_days_between * i);
        }
    }

    ret
}

// FIXME: If I was a better developer I would have written tests here, however
// iteration was faster just playing with the gui
pub fn calculate_vacation_days<'a>(
    mut fixed_days: Vec<NaiveDate>,
    remaining_days: u16,
    start_date: &NaiveDate,
) -> Vec<NaiveDate> {
    // Potential improvements:
    // * Option to allow larger gaps around areas with lots of days off
    // * Do not pick weekends
    // * Add an end date

    fixed_days.sort();
    let relevant_fixed_days = get_relevant_fixed_days(fixed_days, start_date);
    let times_between_days = compute_times_between_days(&relevant_fixed_days);
    let days_off_between = compute_days_per_bucket(&times_between_days, remaining_days);
    let ideal_days_off = compute_ideal_days_off(&relevant_fixed_days, &days_off_between);

    ideal_days_off
}
