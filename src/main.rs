/*
	************************************************************************
	*
	*    Author : David Billsbrough <billsbrough@gmail.com>
	*   License : GNU General Public License -- version 2
	*  Warranty : None
	*  Language : Rust 1.78
	*   Purpose : Calculate the difference in days between two dates
	*
	*   Created : Friday, May 31, 2024 at 20:04:26 PM (EDT)
	*   Version : $Revision: 0.25 $
	*
	************************************************************************
*/

//  $Id: main.rs,v 0.25 2024/06/01 22:51:17 kc4zvw Exp kc4zvw $

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, FixedOffset, Local, Utc};
use datetime::convenience::Today;
use chrono::{NaiveDate, NaiveDateTime};


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

type Output = SystemTime;

fn get_home_dir() -> String {
	// let mut myHOME: str = path.display();
	let my_home = String::from("/home/kc4zvw");
	// println!("My $HOME directory is {}.", my_home);
	return my_home
}

fn formatted_date(_d: Output) -> String {

	let local_time = Local::now();
	let _utc_now: DateTime<Utc> = Utc::now();

	println!("{:?}", local_time);

	let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
	let _est_timezone = FixedOffset::west_opt(5 * 3600);
	let _cst_timezone = FixedOffset::west_opt(6 * 3600);
	let local_timezone = Local::now();

	let formatted = format!("{}", local_time.format("%A, %B %d, %Y at %H:%M:%S %p"));

	let formatted2 = format!("{}", local_time.format("%A, %B %d, %Y at %H:%M:%S %p (%Z)"));

	println!("Local time now is {}", local_time);
	println!("UTC time now is {}", utc_time);
	// println!("Time in East standard time now is {}", utc_time.with_timezone(&est_timezone));
	// println!("Time in Central standard time now is {}", utc_time.with_timezone(&_cst_timezone));
	println!("{}", "");

	let time_str: String = formatted2;
	let result: String = formatted;

	return result; 
}

pub fn unix_coverter(date: String) -> i64 {

	let vdate: Vec<&str> = date.split(&['/', ' ', ':', '~'][..]).collect();
	// assert_eq!(vdate, ["2024", "05", "28", ""]);

	let month = vdate[0].parse::<u32>().unwrap();
	let day = vdate[1].parse::<u32>().unwrap();
	let year = vdate[2].parse::<i32>().unwrap();

	/* parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"), */

	let date_fmt = NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(12, 00, 00).unwrap();

	// println!("{:?}", date_fmt);

	let date_unix: i64 = NaiveDateTime::timestamp(&date_fmt);
	let result_unix = date_unix * 1;
 
	return result_unix
}

fn process_line(line: &str) {

	let local_time = Local::now();

	
	println!("{:?}", local_time);

	let today:LocalDate = LocalDate::today();

	// let (curdate, curname) = local_time.split_at(11);

	println!("{:?}", today);
	//println!("{:?}", curname);

	let _year: u32 = 2024;
	let _month: i32 = 6;
	let _day: u32 = 2;

	println!("{}", _year);
	println!("{}", _month);
	println!("{}", _day);

	let aline = line.trim();
	// println!("Line: '{:?}'", aline);

	let (event_date, event_name) = aline.split_at(11);

	let vdate: Vec<&str> = event_date.split(&['/', ' ', ':', '~'][..]).collect();
	// assert_eq!(vdate, ["2024", "05", "28", ""]);

	let year = vdate[0].parse::<i32>().unwrap();
	let month = vdate[1].parse::<u32>().unwrap();
	let day = vdate[2].parse::<u32>().unwrap();

	let _parse_from_str = NaiveDateTime::parse_from_str;

	let _now: i64 = 0;

	let date_ymd = String::from(month.to_string() + "/" + &day.to_string() + "/" + &year.to_string()
			+ " 12:00");

	let date_now = String::from(_month.to_string() + "/" + &_day.to_string() + "/" + &_year.to_string()
			+ " 12:00");

	// println!("{}", date_ymd);
	// println!("{}", date_now);

	let date_target: i64 = unix_coverter(date_ymd);
	let date_source: i64 = unix_coverter(date_now);
	let day_count: i64 = ((date_target - date_source) / 86400) + 1;
	let diff: i64;

	if day_count <= -2 {
		diff = day_count.abs();
		println!("It was {:?} days ago since {}.", diff, event_name);
	} else if day_count == -1 {
		println!("Yesterday was {}.", event_name);
	} else if day_count == 0 {
		println!("Today is {}.", event_name);
	} else if day_count == 1 {
		println!("Tomorrow is {}.", event_name);
	} else {
		println!("There are {:?} days until {}.", day_count, event_name);
	}
}


// ### ================ Main program begins here ================ ###

fn main() {

	let today_now = SystemTime::now();

	let _date_str: String = formatted_date(today_now);

	println!();
	println!("Days To Go Calculator (Rust version)");
	println!();
	println!("Today is {} (local).", _date_str);
	println!();

	let mut _filename = String::from(".calendar");
	let mut _home = get_home_dir();
	let s1 = String::from(_home);
	let s2 = String::from("/");
	let s3 = String::from(_filename);
	let calendar_file = s1 + &s2 + &s3;
	let filename = calendar_file.clone();

	// println!("{:?}", calendar_file);

	// File .calendar must exist in the home directory
	if let Ok(lines) = read_lines(filename) {
		// Consumes the iterator, returns an (Optional) String
		for line in lines.flatten() {
			// println!("{}", line);
			process_line(&line);
		}
	}

	println!();
	println!("End of report");
}

/* **** End of script **** */
