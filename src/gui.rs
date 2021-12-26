use thiserror::Error;
use vacation_planner::Planner;

use std::ffi::c_void;
use std::cell::RefCell;
use chrono::{NaiveDate, NaiveDateTime};
use log::error;

mod imp {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(unused)]
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Error, Debug)]
pub enum GuiError {
    #[error("Failed to create Gui")]
    CreationFailed
}

pub struct Gui {
    inner: *mut imp::Gui,
    #[allow(unused)]
    planner: Box<RefCell<Planner>>,
}

impl Gui {
    pub fn new(planner: Planner) -> Result<Gui, GuiError> {
        let planner = Box::new(RefCell::new(planner));
        unsafe {
            let callbacks = imp::GuiCallbacks {
                data: (&*planner as *const RefCell<Planner>) as *const c_void,
                setNumVacationDays: Some(set_num_vacation_days),
                setFixedVacationDays: Some(set_fixed_vacation_days),
                setStartDate: Some(set_start_date),
                setProvince: Some(set_province),
                getVacationDays: Some(get_vacation_days),
                getHolidays: Some(get_holidays),
                freeDateList: Some(free_date_list),
            };

            let inner = imp::makeGui(callbacks);

            if inner.is_null() {
                return Err(GuiError::CreationFailed);
            }

            Ok(Gui {
                inner,
                planner,
            })
        }
    }

    pub fn exec(&mut self) {
        unsafe {
            imp::exec(self.inner);
        }
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        unsafe {
            imp::destroyGui(self.inner);
        }
    }
}

fn planner_from_data(data: *const c_void) -> &'static RefCell<Planner> {
    unsafe {
        let planner = data as *const RefCell<Planner>;
        &*planner as &RefCell<Planner>
    }
}

fn msecs_to_date(msecs: &u64) -> NaiveDate {
    let secs = msecs / 1000;
    let nsecs = (msecs % 1000) * 1000 * 1000;
    // FIXME: casts are danger :(
    NaiveDateTime::from_timestamp(secs as i64, nsecs as u32).date()
}

fn date_to_msecs(date: &NaiveDate) -> u64 {
    date.and_hms(0, 0, 0).timestamp_millis() as u64
}

unsafe extern "C" fn set_num_vacation_days(num: u16, data: *const c_void) {
    let mut planner = planner_from_data(data).borrow_mut();
    planner.set_num_vacation_days(num);
}


unsafe extern "C" fn set_fixed_vacation_days(msecs_since_epochs: *const u64, num_dates: u64, data: *const c_void) {
    let mut planner = planner_from_data(data).borrow_mut();


    // FIXME: Spooky u64 -> usize cast
    let dates = std::slice::from_raw_parts(msecs_since_epochs, num_dates as usize)
        .iter()
        .map(msecs_to_date)
        .collect::<Vec<_>>();

    // FIXME: Error propagation or logging or something
    if let Err(e) = planner.set_fixed_vacation_days(dates) {
        error!("Failed to set fixed vacation days: {}", e);
    }
}

unsafe extern "C" fn set_start_date(date: u64, data: *const c_void) {
    let mut planner = planner_from_data(data).borrow_mut();
    planner.set_start_date(msecs_to_date(&date));
}

unsafe extern "C" fn set_province(_province: u32, _data: *const c_void) {
    // TODO
}

unsafe extern "C" fn get_vacation_days(msecs_since_epochs: *mut *mut u64, size: *mut u64, data: *const c_void) {
    let planner = planner_from_data(data).borrow_mut();
    let out = planner.suggested_vacation_days().iter().map(date_to_msecs).collect::<Vec<_>>();
    *size = out.len() as u64;
    if *size != 0 {
        let mut b = out.into_boxed_slice();
        *msecs_since_epochs = b.as_mut_ptr();
        std::mem::forget(b);
    } else {
        *msecs_since_epochs = std::ptr::null_mut();
    }
}

unsafe extern "C" fn get_holidays(msecs_since_epochs: *mut *mut u64, size: *mut u64, data: *const c_void) {
    let planner = planner_from_data(data).borrow_mut();
    let out = planner.holidays().iter().map(date_to_msecs).collect::<Vec<_>>();
    *size = out.len() as u64;
    if *size != 0 {
        let mut b = out.into_boxed_slice();
        *msecs_since_epochs = b.as_mut_ptr();
        std::mem::forget(b);
    } else {
        *msecs_since_epochs = std::ptr::null_mut();
    }
}


unsafe extern "C" fn free_date_list(data: *mut u64) {
    Box::from_raw(data);
}
