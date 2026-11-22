fn main()
{
/*
C++: You can manage memory manually, or use tools such as smart 
pointers and RAII.
Java: Garbage Collector manages memory.
JavaScript: Garbage Collector manages memory.
Rust: Ownership rules manage memory automatically at
compile time.
*/ 

// what is the garbage collector
/*
A Garbage Collector (GC) is a mechanism in programming 
languages that automatically finds and removes memory that
your program no longer needs.
*/


// what rust offers 
/*
2. What is ownership?
Ownership means that every value in Rust has an owner.

There are three main ownership rules:
1. Every value has one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, Rust automatically
drops the value and releases its resources.
*/
    let mut name1 = String::from("Salal");

 {

        let name2 = &mut name1;
        name2.push_str(" Qadir");
    	println!("name2 {:?}",name2.clone() );
 }
        println!("name1 {:?}",name1.clone() );

    // let name2 = name1; its not allowed in the rust
    // /compile-time error because name1 no longer owns the String
    let name2 = &name1; // then we can do refernce
    /*
	6. Mutable borrowing
	What if you want to change the original String without
	transferring ownership?
	Rust provides mutable references using &mut.
	let mut name = String::from("Salal");
    let name2 = &mut name;
    name2.push_str(" Qadir");
    println!("{}", name2);
    */

    println!("name1 {:?}",name1 );
    println!("name2 {:?}",name2 );
    // name2.push_str("Qadir");
    // println!("name2 {:?}",name2.clone() );


}



