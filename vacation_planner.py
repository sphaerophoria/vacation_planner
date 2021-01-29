import json
import requests
import iteround

from collections import defaultdict
from datetime import date
from itertools import *
from functools import *
from isoweek import Week

HOLIDAY_URI = "http://canada-holidays.ca/api/v1/holidays"

def pairwise(iterable):
    "s -> (s0,s1), (s1,s2), (s2, s3), ..."
    a, b = tee(iterable)
    next(b, None)
    return zip(a, b)


def parse_holidays(input):
    holidays = defaultdict(list)
    for holiday in input['holidays']:
        holiday_provinces = holiday['provinces']
        holiday_date = date.fromisoformat(holiday['date'])
        for province in holiday_provinces:
            province_id = province['id']
            holidays[province_id].append(holiday_date)

    return holidays

def get_holidays():
    response = requests.get(HOLIDAY_URI)
    return parse_holidays(json.loads(response.content))

def num_weeks_between(a, b):
    week_b = b.isocalendar()[1]
    week_a = a.isocalendar()[1]

    return week_b - week_a

def get_holiday_weight(delta, num_remaining_weeks):
    return delta / num_remaining_weeks

def holiday_delta_too_low(delta, optimal):
    return delta < optimal

def get_effective_num_remaining_weeks(num_remaining_weeks, optimal_interval, holiday_deltas):
    holiday_deltas_without_low_gaps = filter(lambda delta: holiday_delta_too_low(delta, optimal_interval), holiday_deltas)
    return num_remaining_weeks - sum(holiday_deltas_without_low_gaps, 0)

def get_vacation_week_nums(num_days, start_date, end_date):
    if num_days == 0:
        return []

    end_week = end_date.isocalendar()[1]
    start_week = start_date.isocalendar()[1]
    optimal_interval = float(end_week - start_week) / (num_days + 1)
    vacation_week_nums = []
    for i in range(0, int(num_days)):
        vacation_week_nums.append(start_week + (i + 1) * optimal_interval)

    return list(iteround.saferound(vacation_week_nums, 0))


def main():
    holidays = get_holidays()
    bc_holidays = holidays['BC']
    bc_holidays.sort()

    start_date = date(2021, 2, 14)
    num_vacation_days = 14

    remaining_holidays = list(filter(lambda date: date > start_date, bc_holidays))
    remaining_holidays.sort()

    # Include new years of next year to ensure we are counting the time between
    # the last holiday of the year and the beginning of next year. This also
    # makes some of the math work out better since we can assume that the last
    # holiday lines up with the number of weeks left in the year
    remaining_holidays.append(date(start_date.year + 1, 1, 1))

    num_remaining_holidays = len(remaining_holidays)
    num_remaining_days_off = num_vacation_days + num_remaining_holidays
    num_remaining_weeks = num_weeks_between(start_date, date(start_date.year, 12, 31))

    holiday_deltas = list(map(lambda item: num_weeks_between(item[0], item[1]), pairwise(remaining_holidays)))

    optimal_interval = float(num_remaining_weeks) / num_remaining_days_off

    effective_num_remaining_weeks = get_effective_num_remaining_weeks(num_remaining_weeks, optimal_interval, holiday_deltas)

    delta_weights = list(map(lambda delta: delta / effective_num_remaining_weeks if not holiday_delta_too_low(delta, optimal_interval) else float(0), holiday_deltas))

    delta_num_days = list(iteround.saferound(list(map(lambda x: x * num_vacation_days, delta_weights)), 0))

    assert(sum(delta_num_days, 0) == num_vacation_days)

    vacation_day_weeks = []
    for i in range(0, len(remaining_holidays) - 1):
        vacation_day_weeks.extend(get_vacation_week_nums(delta_num_days[i], remaining_holidays[i], remaining_holidays[i + 1]))

    vacation_day_mondays = list(map(lambda week: Week(start_date.year, week).monday(), vacation_day_weeks))
    vacation_day_mondays_formatted = list(map(lambda date: date.strftime("%Y-%m-%d"), vacation_day_mondays))

    print(f"You should take a vacation day on the weeks of: {vacation_day_mondays_formatted}")

    all_days_off = remaining_holidays
    all_days_off.extend(vacation_day_mondays)
    all_days_off.sort()
    all_days_off_formatted = list(map(lambda date: date.strftime("%Y-%m-%d"), all_days_off))
    print(f"Resulting in the following days off {all_days_off_formatted}")

    day_off_deltas = list(map(lambda item: num_weeks_between(item[0], item[1]), pairwise(all_days_off)))
    print(f"And the following week intervals {day_off_deltas}")

if __name__ == '__main__':
    main()
