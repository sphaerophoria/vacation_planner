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

trait ToNaiveDate {
    fn to_naivedate(&self) -> Option<NaiveDate>;
}

impl ToNaiveDate for QDateTime {
    fn to_naivedate(&self) -> Option<NaiveDate> {
        // QML calendar dates are based off UTC time, but represented in local
        // time. To get the desired date we need to determine what the UTC date
        // is for the given local time
        let (date, time) = self.get_date_time();

        let (y, m, d) = date.get_y_m_d();
        let h = time.get_hour();
        let date = NaiveDate::from_ymd_opt(y, m as u32, d as u32);
        if date.is_none() {
            return None;
        }

        let date = date.unwrap();
        let time = NaiveTime::from_hms(h as u32, 0, 0);
        let naive_local = NaiveDateTime::new(date, time);

        debug!("naive_local: {}", naive_local);

        let local = Local.from_local_datetime(&naive_local).unwrap();
        debug!("local: {}", local);

        debug!("naive_utc: {}", local.naive_utc());
        Some(local.naive_utc().date())
    }
}

trait ToQDateTime {
    fn to_qdatetime(&self) -> QDateTime;
}

impl ToQDateTime for NaiveDate {
    fn to_qdatetime(&self) -> QDateTime {
        // QML calendar dates are based off UTC time, but represented in local
        // time. To keep consistency with the QML calendar we need to convert
        // our date to a local timestamp that corresponds with 00:00:00 on the
        // desired UTC date
        let date_time = NaiveDateTime::new(*self, NaiveTime::from_hms(0, 0, 0));

        let local_date_time = Local.from_utc_datetime(&date_time).naive_local();
        let qdate = QDate::from_y_m_d(
            local_date_time.year(),
            local_date_time.month() as i32,
            local_date_time.day() as i32,
        );
        let qtime = QTime::from_h_m_s_ms(local_date_time.hour() as i32, 0, None, None);
        QDateTime::from_date_time_local_timezone(qdate, qtime)
    }
}

#[derive(QObject)]
#[allow(non_snake_case)]
struct VacationPlanner {
    base: qt_base_class!(trait QObject),
    numVacationDays: qt_property!(u16; WRITE set_num_vacation_days),
    fixedVacationDays: qt_property!(QVariantList; WRITE set_fixed_vacation_days),
    startDate: qt_property!(QDateTime; NOTIFY start_date_signal WRITE set_start_date),
    start_date_signal: qt_signal!(),
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
            .map(NaiveDate::to_qdatetime)
            .collect::<QVariantList>();

        debug!("Holidays: {:?}", planner.holidays());

        VacationPlanner {
            base: Default::default(),
            // inputs
            numVacationDays: Default::default(),
            fixedVacationDays: Default::default(),
            startDate: Default::default(),
            start_date_signal: Default::default(),
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
            match date.to_naivedate() {
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
        let start_date = match date.to_naivedate() {
            Some(date) => date,
            None => {
                error!("Failed to parse QDate");
                return;
            }
        };

        self.planner.set_start_date(start_date);
        self.start_date_signal();
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
            .map(NaiveDate::to_qdatetime)
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
