/*
trait is a set of behaviours(method) that method that different
struct or type

basically trait is a contract. tell the struct what functions it
must provided
example for we have 3 users Admin, Teacher and Student but these
all three roles have common function login
*/

// we declaring the login 



/* now we are creating 1 structs rather 3 we will create a enum
   of role
*/
#[derive(Debug)]
pub enum Role {
	Student,
	Teacher,
	Admin
}
#[derive(Debug)]
pub struct User {
	pub name : String,
	pub email : String,
	pub age : i8,
	pub password : String,
	pub role : Role
}
pub trait Login {
	fn login(&self);
}
impl Login for User {
	 fn login(&self)
	{
		let role = match self.role{
			Role::Student => "Student",
			Role::Admin => "Admin",
			Role::Teacher => "Teacher"
		};
		println!("{} Logged In As {:?} ",self.name, role );
	}
}
 fn main() {
let _user = User{
	name: String::from("salal"),
	email: String::from("salalqadir06@gmail.com"),
	password: String::from("file1234@"),
	age: 25,
	role : Role::Admin
};

_user.login();
}