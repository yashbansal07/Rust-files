extern crate classroom_diesel;
extern crate diesel;

use self::classroom_diesel::*;
fn main() {
    let connection = create_connection();
    let firstname = "John";
    let lastname = "Doe";
    let age: i32 = 64;

    let student = create_post(&connection, firstname, lastname, &age);
    println!(
        "Saved student {} with id {}",
        student.firstname, student.id
    );
}