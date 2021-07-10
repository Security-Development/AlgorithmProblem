
fn main(){
	println!("{:?}", hello(253));
}

fn hello(number: i32) -> i32 {

	let mut count = 0;
	let mut number = number;

	while number > 0 {
		count += number % 10;
		number /= 10;
	}

	return count;
}
