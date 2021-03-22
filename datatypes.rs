fn main(){
	// there are mainly 4 primitive data types in rust
	let int32 :i32 = 52; // stores 32 bit integer data
	let int64 :i64 = 55; // stores 64 bit integer data
	let float32 :f32 = 52.5; // stores 32 bit float data
	let float64 :f64 = 102.5; // stores 64 bit float data
	let string = "hello"; // stores string 
	let boolean:bool = false; // stores boolean value
	println!("The value of int32 is {:?}", int32);
	println!("The value of int64 is {:?}", int64);
	println!("The value of float32 is {:?}", float32);
	println!("The value of float64 is {:?}", float64);
	println!("The value of string is {}", string);
	println!("The value of boolean is {:?}", boolean);
}