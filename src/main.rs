extern crate chrono;
extern crate pretty_env_logger;
extern crate reqwest;
extern crate vacation_planner;
extern crate log;

use std::error::Error;

mod gui;

use gui::Gui;

use vacation_planner::Planner;
use vacation_planner::Province;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let planner = Planner::from_web(Province::BC)?;
    let mut gui = Gui::new(planner).unwrap();
    gui.exec();

    Ok(())
}
