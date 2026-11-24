/*
struct is the data structure in rust there are 3 types structs
1) name field struct
2) unit struct
3) tuple struct
*/

// name field struct
#[derive(Debug)]
struct User{
	username : String,
	email : String,
	password : String,
	fullname : Option<String>
}
// 3rd type of the struct
Struct Admin;



// by default structs are private if we want to use it in
// public mode we need to add pub before the struct 
fn main()
{
let user = User{
	username: String::from("salal"),
	email: String::from("salalqadir06@gmail.com"),
	password: String::from("file1234@"),
	fullname: Some(String::from("salal")),
};
// 2nd type of struct tuple
struct Point(i32,i32);


let fullname = user.fullname.expect("Some Name");
let username= user.username;
let email = user.email;
let password = user.password;
// println!("user details is given below : {:?}", user);
 let val = Point(32,54);
println!("full name :{:?}", fullname);
println!("username :{:?}", username);
println!("email :{:?}", email);
println!("password :{:?}", password);
println!("first index of the point is {:?}", val.0);
println!("Second index of the point is {:?}", val.1);
}

