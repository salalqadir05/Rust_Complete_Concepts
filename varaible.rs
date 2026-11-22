
fn main(){
	let num1 : i8 = 127;
	let num2 : i8 = -128; // i8 means is -128 to 127 8 bit 	
	println!("Positive i8 {}", num1);
	println!("Negative i8 {}", num2);
	let num3 : i16 = 32767;
	let num4 : i16 = -32768; // i16 means is -32768 to 32767 15 bit 
	println!("Positive i16 {}", num3);
	println!("Negative i16 {}", num4); 

	// 2^(16-1) -1 this is the formula for integers in the rust

	//there is one another function issize
	let num5 : isize = 23423; // its match of the computer memory
	println!("num 5 of isize {:?}",num5 ); 
	let f_num1: f64 = 245.5; // Seconds
    let u_num1: usize = 1000;
    let u_num2: usize = 5;
    println!("f_num1 {:?}",f_num1 );
    println!("u_num1 {:?}",u_num1 );
    println!("u_num2 {:?}",u_num2 );
    let c_char1 : char = 'A';
    let c_char2: char = '5';
    let c_char3: char = '😊';
    println!("c_char1 {:?}",c_char1 );
    println!("c_char2 {:?}",c_char2 );
    println!("c_char3 {:?}",c_char3 );
    // there is 2 types of the string 
    // 1) string 
    // 2) &string &str
    // string is borrowing the data and can grow and changed and heap allocated
    // &str borrowing the data and cannot be regrow and rechanged and can refered to different string
    let st1: String = String::from("Salal");
    let st2: &str = "Salal";
    println!("s_string {:?}",st1 );
    println!("string {:?}",st2 );

}
