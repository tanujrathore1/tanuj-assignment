#![allow(dead_code)]
#![allow(unused)]
use clap::Parser;
use std::collections::HashMap;
// use std::fs::OpenOptions;
// use std::io::Write;

mod read_file;
use read_file::*;

#[derive(Parser, Debug)]
struct InputFilePath {
    #[clap(long)]
    e: String,
    #[clap(long)]
    d: String,
    #[clap(long)]
    s: String,
    #[clap(long)]
    l: String,
    #[clap(long)]
    o: String,
    #[clap(long)]
    id: i32,
}

#[derive(Debug)]
pub struct Employee {
    emp_id: i32,
    emp_name: String,
    dept_id: i32,
    mobile_no: String,
    email: String,
}

#[derive(Debug)]
pub struct Dept {
    dept_id: i32,
    dept_title: String,
    dept_strength: i32,
}

#[derive(Debug)]
pub struct Salary {
    emp_id: i32,
    salary_id: i32,
    salary_date: String,
    salary: f64,
    salary_status: String,
}

#[derive(Debug)]
pub struct Leave {
    emp_id: i32,
    leave_id: i32,
    leave_from: f64,
    leave_to: f64,
    leave_type: String,
    leave_count: i32,
}

#[derive(Debug)]
struct Output {
    emp_id: i32,
    emp_name: String,
    dept_title: String,
    mobile_no: String,
    email: String,
    salary_status: String,
    on_leave: i32,
}

fn main() {
    let file_path = InputFilePath::parse();

    let mut emp_data: HashMap<String, Employee> = HashMap::new();
    read_txt_file(file_path.e, file_path.id, &mut emp_data)
        .expect("not able to read daat from file");

    let emp_id = emp_data
        .get(&file_path.id.to_string())
        .expect("not able to fetch emp id")
        .emp_id;

    // dept file data
    let dept_id = emp_data
        .get(&file_path.id.to_string())
        .expect("not able to fetch dept id")
        .dept_id;
    let mut dept_data: HashMap<String, Dept> = HashMap::new();
    read_dept_file_data(file_path.d, dept_id, &mut dept_data, "dept")
        .expect("not able to read daat from file");

    // salary data
    let mut salary_data: HashMap<String, Salary> = HashMap::new();
    read_salary_data(file_path.s, emp_id, &mut salary_data, "salary")
        .expect("not able to read daat from file");

    // leave data
    let mut leave_data: HashMap<String, Leave> = HashMap::new();
    read_leave_data(file_path.l, emp_id, &mut leave_data, "leave")
        .expect("not able to read daat from file");

    let dept_data_id = dept_id.to_string();
    let out = Output {
        emp_id: emp_id,
        emp_name: emp_data
            .get(&file_path.id.to_string())
            .expect("No employee name")
            .emp_name
            .clone(),
        dept_title: dept_data
            .get(&dept_data_id)
            .expect("No department tile")
            .dept_title
            .clone(),
        mobile_no: emp_data
            .get(&file_path.id.to_string())
            .expect("No mobile number")
            .mobile_no
            .clone(),
        email: emp_data
            .get(&file_path.id.to_string())
            .expect("No email found")
            .email
            .clone(),
        salary_status: salary_data
            .get(&file_path.id.to_string())
            .expect("Salary status not known")
            .salary_status
            .clone(),
        on_leave: leave_data
            .get(&file_path.id.to_string())
            .expect("No leave data found")
            .leave_count,
    };

    // Write to a file
    let mut content = String::new();
    let emp_name_heading = "Emp_name";
    let emp_id_heading = "Emp_id";
    let dept_title_heading = "dept_title";
    let mobile_no_heading = "mobile_number";
    let email_heading = "email";
    let salary_status_heading = "salary_status";
    let on_leave_heading = "on_leave";

    let heading = format!(
        "{}#{}#{}#{}#{}#{}#{}",
        emp_id_heading,
        emp_name_heading,
        dept_title_heading,
        mobile_no_heading,
        email_heading,
        salary_status_heading,
        on_leave_heading
    );

    content.push_str(&heading);
    let file_data = format!(
        "{:?}#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}",
        out.emp_id,
        out.emp_name,
        out.dept_title,
        out.mobile_no,
        out.email,
        out.salary_status,
        out.on_leave
    );

    content.push_str(&file_data);

    std::fs::write(file_path.o, content).expect("not able to write data in file");
}
