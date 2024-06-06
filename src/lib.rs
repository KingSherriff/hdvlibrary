use rand::Rng;
const N: i32 = 10000;

/// Generate a single random hypervector of length 10,000 with a near 50% spread of 1 and -1.
///
/// # Examples
///
/// ```
/// let hdv = gen_hdv();
///
/// assert_eq!(10000, len(hdv));
/// ```
pub fn gen_hdv() -> Vec<i32> {
    let mut rng = rand::thread_rng();
	let mut vec:Vec<i32> = Vec::new();
	for _ in 0..N {
		let toss: bool = rng.gen_bool(0.5);
		if toss == true {
			vec.push(1);
		} else {
			vec.push(-1);
		}
	}
	return vec;
}

/// Generate a single random hypervector of length 'n' with 1 appearing near 'r'*100 percent of the time.
///
/// # Examples
///
/// ```
/// let hdv = gen_hdv_num(20000, 0.8);
///
/// assert_eq!(20000, len(hdv));
/// ```
pub fn gen_hdv_custom(num: i32, rand: f64) -> Vec<i32> {
    let mut rng = rand::thread_rng();
	let mut vec:Vec<i32> = Vec::new();
	for _ in 0..num {
		let toss: bool = rng.gen_bool(rand);
		if toss == true {
			vec.push(1);
		} else {
			vec.push(-1);
		}
	}
	return vec;
}

/// Generate 'n' random hypervectors of length 10,000 with a near 50% spread of 1 and -1.
///
/// # Examples
///
/// ```
/// let hdvm = gen_hdv_matrix(3);
/// let hdv = hdvm[0];
///
/// assert_eq!(3, len(hdvm));
/// assert_eq!(10000, len(hdv));
/// ```
pub fn gen_hdv_matrix(num: i32) -> Vec<Vec<i32>> {
	let mut vec:Vec<Vec<i32>> = Vec::new();
	for _ in 0..num {
		vec.push(gen_hdv());
	}
	return vec;
}

/// Generate 'n' random hypervectors of length 'size' with a near 'r'*100 percent of the time per hypervector.
///
/// # Examples
///
/// ```
/// let hdvm = gen_hdv_matrix(3, 20000, 0.8);
/// let hdv = hdvm[0];
///
/// assert_eq!(3, len(hdvm));
/// assert_eq!(20000, len(hdv));
/// ```
pub fn gen_hdv_matrix_custom(num: i32, size: i32, chance: f64) -> Vec<Vec<i32>> {
	let mut vec:Vec<Vec<i32>> = Vec::new();
	for _ in 0..num {
		vec.push(gen_hdv_custom(size, chance));
	}
	return vec;
}

pub fn bundle_vec(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
	let mut vec3:Vec<i32> = Vec::new();
	for i in 0..N as usize {
		if vec1[i] == vec2[i] {
			vec3.push(vec1[i]);
		} else {
			vec3.push(0);
		}
	}
	return vec3;
}

pub fn bundle_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
	let mut vec:Vec<i32> = matrix[0].clone();
	for i in 1..matrix.len() as usize {
		for j in 0..N as usize {
			if vec[j] == matrix[i][j] {
				vec.push(matrix[i][j]);
			} else {
				vec.push(0);
			}
		}
	}
	return vec;
}

/// Create a new HDV from an old one by shifting its elements.
pub fn shift(vec: Vec<i32>) -> Vec<i32> {
	let mut new_vec = vec.clone();
	new_vec.rotate_right(1);
	return new_vec;
}

/// Create a new HDV from an old one by shifting its elements 'n' amount.
pub fn shift_custom(vec: Vec<i32>, shift_amount: usize) -> Vec<i32> {
	let mut new_vec = vec.clone();
	new_vec.rotate_right(shift_amount);
	return new_vec;
}