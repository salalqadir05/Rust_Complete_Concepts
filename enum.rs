/* enums are custom data types that allow you to define
a type that have multiple possible varient 

how its different struct ?
struct is entity having different multiple fields together 
enum is representation one of the multiple possible varaient 

*/
#[derive(Debug)]
enum IpAddrType{
	_Ipv4,
	_Ipv6
}
#[derive(Debug)]
struct Computer{
	_ipaddr : IpAddrType,
	_address : String
}
#[derive(Debug)]
enum Role{
	Admin,
	Developer,
	Customer
}
#[derive(Debug)]
struct User{
	_username : String,
	_email : String,
	_password : String,
	_fullname : Option<String>,
	_role : Role
}
fn main()
{
	let details_computer = Computer{
		_ipaddr : IpAddrType::_Ipv4,
		_address : String::from("127.0.0.1")
	};
	println!("Details of the Computer {:?}", details_computer);
	let _user = User{
	_username: String::from("salal"),
	_email: String::from("salalqadir06@gmail.com"),
	_password: String::from("file1234@"),
	_fullname: Some(String::from("salal")),
	_role : Role::Admin
};
	let _user1 = User{
	_username: String::from("ahmad"),
	_email: String::from("ahmad@gmail.com"),
	_password: String::from("file1234@"),
	_fullname: Some(String::from("ahmad")),
	_role : Role::Developer
};
	let _user2 = User{
	_username: String::from("ali"),
	_email: String::from("ali@gmail.com"),
	_password: String::from("file1234@"),
	_fullname: Some(String::from("ali")),
	_role : Role::Customer
};
println!("List of the Users\n");
println!("{:?}\n{:?}\n{:?}",_user,_user1,_user2 );
}