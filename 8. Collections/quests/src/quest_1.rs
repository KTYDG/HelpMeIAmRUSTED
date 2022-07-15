use self::{tools::{create, print_v}, math::{median, average, mode_of_v}};

pub fn quest() {
    let mut v = create();
    print_v(&v);
    average(&v);
    median(&mut v);
    mode_of_v(&v);
}
mod tools;
mod math;
