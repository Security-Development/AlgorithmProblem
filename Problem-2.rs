fn main() {
	let mut target: [i32; 3] = [1, 2, 3];

	println!("{}", Check(&mut target));
}

fn Check(arr: &mut [i32]) -> bool{
	let mut inspection: [i32; 100] = [0; 100];
	let max = arr.len() as i32;
	let mut mode: bool = false;
	for n in 0..=max -1 {
		if arr[n as usize] < 1 || arr[n as usize] > max {
			mode = false;
		}
		inspection[arr[n as usize] as usize] += 1;
	}

	for n in 1 ..=max{
		if inspection[n as usize] != 1{
			mode = false;
			break;
		}
		mode = true;
	}
	return mode;
}
