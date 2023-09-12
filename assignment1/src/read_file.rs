// #![allow(dead_code)]
// #![allow(unused)]
use calamine::{DataType, Excel, Range};
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use std::collections::HashMap;
// use std::fs;
use std::io;
use std::io::prelude::*;
// use std::str::FromStr;

use crate::*;

pub fn read_txt_file(
    file_path: String,
    id: i32,
    temp: &mut HashMap<String, Employee>,
) -> Result<(), io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut flag = true;
    for entries in content.lines() {
        if flag {
            flag = false;
            continue;
        }
        // split and bind values
        let mut values = entries.split('|');

        let emp_id = values
            .next()
            .expect("No emp id")
            .parse::<i32>()
            .unwrap_or(0);
        let emp_name_data = values.next().expect("No emp name").to_string();
        let dept_id = values
            .next()
            .expect("No dept id")
            .parse::<i32>()
            .unwrap_or(0);
        let mobile_no = values.next().expect("No emobile number").to_string();
        let email = values.next().expect("No email").to_string();

        if emp_id == id {
            let emp = Employee {
                emp_id: emp_id,
                emp_name: emp_name_data,
                dept_id: dept_id,
                mobile_no: mobile_no,
                email: email,
            };
            temp.insert(emp_id.to_string(), emp);
        }
    }

    Ok(())
}

pub fn read_dept_file_data(
    file_path: String,
    id: i32,
    temp: &mut HashMap<String, Dept>,
    file_name: &str,
) -> Result<(), io::Error> {
    let mut excel = Excel::open(file_path).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();

    let mut flag = true;
    for row in r.rows() {
        if flag {
            flag = false;
            continue;
        }

        if file_name == "dept" {
            let dept_id = match row[0] {
                DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                _ => 0,
            };

            if dept_id == id {
                let dept_title = match &row[1] {
                    DataType::String(v) => v,
                    _ => "",
                };
                let dept_strength = match row[2] {
                    DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                    _ => 0,
                };

                let dept = Dept {
                    dept_id: dept_id,
                    dept_title: dept_title.to_string(),
                    dept_strength: dept_strength,
                };
                temp.insert(dept_id.to_string(), dept);
            }
        }
    }
    Ok(())
}

pub fn read_salary_data(
    file_path: String,
    id: i32,
    temp: &mut HashMap<String, Salary>,
    file_name: &str,
) -> Result<(), io::Error> {
    let mut excel = Excel::open(file_path).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();

    let mut flag = true;
    for row in r.rows() {
        if flag {
            flag = false;
            continue;
        }

        if file_name == "salary" {
            let emp_id = match row[0] {
                DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                _ => 0,
            };
            if emp_id == id {
                let salary_id = match row[1] {
                    DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                    _ => 0,
                };

                let salary_date = match &row[2] {
                    DataType::String(v) => v,
                    _ => "",
                };

                let salary = match row[3] {
                    DataType::Float(v) => v,
                    _ => 0_f64,
                };
                let salary_status = match &row[4] {
                    DataType::String(v) => v,
                    _ => "",
                };

                let salary = Salary {
                    emp_id: emp_id,
                    salary_id: salary_id,
                    salary_date: salary_date.to_string(),
                    salary: salary,
                    salary_status: salary_status.to_string(),
                };

                temp.insert(emp_id.to_string(), salary);
            }
        }
    }
    Ok(())
}

fn date_from_float(dte: f64) -> NaiveDate {
    let start = NaiveDate::from_ymd_opt(1900, 1, 1).expect("Not a valid date");

    let date = start.checked_add_signed(Duration::days(
        (dte - 2.0).to_string().parse::<i64>().unwrap(),
    ));

    date.unwrap()
}

pub fn read_leave_data(
    file_path: String,
    id: i32,
    temp: &mut HashMap<String, Leave>,
    file_name: &str,
) -> Result<(), io::Error> {
    let mut excel = Excel::open(file_path).expect("");
    let r = excel.worksheet_range("Sheet1").expect("Sheet not found");

    let mut flag = true;
    for row in r.rows() {
        if file_name == "leave" {
            let emp_id = match row[0] {
                DataType::Float(v) => v.to_string().parse::<i32>().unwrap_or(0),
                _ => 0,
            };
            if emp_id == id {
                let leave_id = match row[1] {
                    DataType::Float(v) => v.to_string().parse::<i32>().unwrap_or(0),
                    _ => 0,
                };

                let leave_from = match row[2] {
                    DataType::Float(v) => v,
                    _ => 0_f64,
                };

                let leave_to = match row[3] {
                    DataType::Float(v) => v,
                    _ => 0_f64,
                };
                let leave_type = match &row[4] {
                    DataType::String(v) => v,
                    _ => "",
                };
                let mut leave_count = 0;

                let leave_month_to = date_from_float(leave_to).month();
                let leave_day = date_from_float(leave_to).day();
                let leave_month_from = date_from_float(leave_from).month();

                let current_month = Utc::now().month();

                if current_month == leave_month_to {
                    if current_month == leave_month_from {
                        leave_count = (leave_to - leave_from) as i32;
                    } else {
                        leave_count = leave_day as i32;
                    }
                } else {
                    leave_count = 0;
                }

                println!("leave_from2 {:?}", leave_count);

                let leave = Leave {
                    emp_id: emp_id,
                    leave_id: leave_id,
                    leave_from: leave_from,
                    leave_to: leave_to,
                    leave_type: leave_type.to_string(),
                    leave_count: leave_count,
                };

                temp.insert(emp_id.to_string(), leave);
            }
        }
    }
    Ok(())
}
