fn main(){
	// For defining a variable we have to use 'let'. By default we cant change the value of a variable inside the programm after the variable being defined but we can do that is we add 'mut' after let
	let y = 500; // This is immutable
	let mut z = 500; // this is mutable
	println!("The value of x is {:?}", y );
	println!("The value of x is {:?}", z );

}