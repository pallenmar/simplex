fn main() {


	let a = vec![1.0,2.0,3.0,4.0,5.0];
	let b = vec![1.0,2.0,3.0,4.0,5.0];
	let a_dot_b = dot(&a, &b);
	println!("{}", a_dot_b);
}

fn transpose_to_row(a: &Vec<Vec<f32>>) -> Vec<f32>{

	let mut vec: Vec<f32> = vec![0.0; a.len()];

	for i in 0..a.len(){
		vec[i] = a[i][0];
	}

	return vec;

}

fn transpose_to_col(a: &Vec<f32>) -> Vec<Vec<f32>>{
	let mut vec: Vec<Vec<f32>> = vec![vec![0.0]; a.len()];
	
	for i in 0..a.len(){
		vec[i][0] = a[i];
	}

	return vec;
}

// changed to vec, since i wanted it to be easier to transpose things
fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {

	if a.len() != b.len() {panic!("lengths of arrays are not the same!")};

	let mut dot_product = 0.0;
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
	
		let a = vec![1.0, 2.0];
		let b = vec![1.0, 2.0, 3.0];
		dot(&a, &b);
}
	#[test]
	fn test_output() {
		let a = vec![1.0,2.0];
		let b=  vec![1.0,2.0];
		assert_eq!(dot(&a,&b),5.0);
 	}	

	#[test]
	fn test_transpose() {
		let a = vec![1.0, 2.0];
		let ans = vec![vec![1.0], vec![2.0]];

		assert_eq!(ans, transpose_to_col(&a));
		assert_eq!(transpose_to_row(&transpose_to_col(&a)), a);
	}

}
