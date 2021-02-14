extern crate chrono;
extern crate pretty_env_logger;
extern crate qmetaobject;
extern crate reqwest;
extern crate vacation_planner;
#[macro_use]
extern crate cstr;
extern crate log;

use log::*;
use std::error::Error;

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};

use vacation_planner::Planner;
use vacation_planner::Province;

use qmetaobject::*;

fn qdate_to_naivedate(datetime: QDateTime) -> Option<NaiveDate> {
    // QDateTime unit is UTC always, but our app is in localtime, and we lose
    // the timezone on rust conversion... so we need to convert this to a UTC
    // DateTime then convert it to local
    let (date, time) = datetime.get_date_time();

    let (y, m, d) = date.get_y_m_d();
    let h = time.get_hour();
    let date = NaiveDate::from_ymd_opt(y, m as u32, d as u32);
    if date.is_none() {
        return None;
    }

    let date = date.unwrap();
    let time = NaiveTime::from_hms(h as u32, 0, 0);
    let utc = NaiveDateTime::new(date, time);

    debug!("{}", utc);

    let local = Local.from_local_datetime(&utc).unwrap();
    debug!("local: {}", local);

    debug!("naive_utc: {}", local.naive_utc());
    Some(local.naive_utc().date())
}

fn naivedate_to_qdate(date: &NaiveDate) -> QDateTime {
    // Our whole app is written in localtime, so we need to convert
    // Input local date -> output utc date
    let date_time = NaiveDateTime::new(*date, NaiveTime::from_hms(0, 0, 0));
    let qdate = QDate::from_y_m_d(
        date_time.year(),
        date_time.month() as i32,
        date_time.day() as i32,
    );
    let qtime = QTime::from_h_m_s_ms(date_time.hour() as i32, 0, None, None);
    QDateTime::from_date_time_local_timezone(qdate, qtime)
}

#[derive(QObject)]
#[allow(non_snake_case)]
struct VacationPlanner {
    base: qt_base_class!(trait QObject),
    numVacationDays: qt_property!(u16; WRITE set_num_vacation_days),
    fixedVacationDays: qt_property!(QVariantList; WRITE set_fixed_vacation_days),
    startDate: qt_property!(QDateTime; WRITE set_start_date),
    province: qt_property!(u32; WRITE set_province),
    vacationDays: qt_property!(QVariantList; NOTIFY vacation_days_signal),
    vacation_days_signal: qt_signal!(),
    holidays: qt_property!(QVariantList; NOTIFY holidays_signal),
    holidays_signal: qt_signal!(),
    province_list: qt_property!(QVariantList),
    planner: Planner,
}

impl Default for VacationPlanner {
    fn default() -> Self {
        // FIXME: VacationPlanner should probably be instantiated from rust to
        // avoid hard crash on failure
        let planner = Planner::from_web(Province::BC).expect("Failed to create planner");
        let holidays = planner
            .holidays()
            .iter()
            .map(naivedate_to_qdate)
            .collect::<QVariantList>();

        debug!("Holidays: {:?}", planner.holidays());

        VacationPlanner {
            base: Default::default(),
            // inputs
            numVacationDays: Default::default(),
            fixedVacationDays: Default::default(),
            startDate: Default::default(),
            province: Default::default(),
            // Outputs
            vacationDays: Default::default(),
            vacation_days_signal: Default::default(),
            holidays: holidays,
            holidays_signal: Default::default(),
            province_list: Default::default(),
            planner: planner,
        }
    }
}

impl VacationPlanner {
    fn set_num_vacation_days(&mut self, num_vacation_days: u16) {
        info!("num_vacation_days: {}", num_vacation_days);
        self.planner.set_num_vacation_days(num_vacation_days);
        self.update_vacation_days();
    }

    fn set_fixed_vacation_days(&mut self, fixed_days: QVariantList) {
        let mut dates = Vec::new();
        for day in &fixed_days {
            let date = QDateTime::from_qvariant(day.clone()).unwrap();
            match qdate_to_naivedate(date) {
                Some(date) => dates.push(date),
                None => error!("Failed to parse QDate"),
            }
        }

        debug!("vacation_days: {:?}", dates);

        if let Err(e) = self.planner.set_fixed_vacation_days(dates) {
            error!("Failed to set vacation days: {}", e);
        }

        self.update_vacation_days();
    }

    fn set_start_date(&mut self, date: QDateTime) {
        let start_date = match qdate_to_naivedate(date) {
            Some(date) => date,
            None => {
                error!("Failed to parse QDate");
                return;
            }
        };

        self.planner.set_start_date(start_date);
        self.update_vacation_days();
    }

    fn set_province(&mut self, _idx: u32) {
        // FIXME: hookup gui for province selection
    }

    fn update_vacation_days(&mut self) {
        self.vacationDays = self
            .planner
            .suggested_vacation_days()
            .iter()
            .map(|date| naivedate_to_qdate(&date))
            .collect::<QVariantList>();

        self.vacation_days_signal();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    qml_register_type::<VacationPlanner>(cstr!("VacationPlanner"), 1, 0, cstr!("VacationPlanner"));

    // FIXME: Embed the QML in release
    let qml_data_path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/Planner.qml");

    let mut engine = qmetaobject::QmlEngine::new();
    engine.load_file(qml_data_path.into());
    engine.exec();

    Ok(())
}
