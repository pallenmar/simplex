fn main() {

	let a = [1, 2, 3, 4, 5];
	let b = [1, 2, 3, 4, 5];

	let a_dot_b = dot(&a, &b);

	println!("{}", a_dot_b);
}

fn dot(a: &[i32], b: &[i32]) -> i32 {

	if a.len() != b.len() {panic!("lengths of arrays are not the same!")};

	let mut dot_product = 0;
	for i in 0..a.len(){
		dot_product += a[i] * b[i];
	};

	dot_product
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	#[should_panic(expected="lengths of arrays are not the same!")]
	fn test_unequal() {
	
		let a = [1, 2];
		let b = [1, 2, 3];
		dot(&a, &b);
}
	#[test]
	fn test_output() {
		let a = [1,2];
		let b=  [1,2];
		assert_eq!(dot(&a,&b),5);
 	}	

}
