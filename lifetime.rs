/*
what is lifetime is ??
lifetime tells the compiler how long reference is valid 
*/

// 1st example 
fn main(){
	// let first ;
	// {
	// 	let r  = 5;
	// 	first = &r; 
	// }
	// println!("{:?}", first); we are getting there because scope is droped 
//2nd example
let string1 = String::from("abcd");
let string2 = String::from("xyz");
let result = longest(&string1, &string2);
println!("Longest String is {:?}", result);
}
// 2nd example 
//fn longest(x : &str, y :&str) -> &str { // with this get error
	// we do not know scope 
	fn longest<'a>(x : &'a str, y :&'a str) -> &'a str {
	if x.len() > y.len()
	{
		x
	}
	else 
	{
		y
	}
}