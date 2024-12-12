use std::collections::HashMap;

fn main() {
	menggunakan_hash_map();
}

fn menggunakan_vec() {
	let sample_vec = vec![1,2,3,4,5];
	for sample in &sample_vec {
		println!("Angka : {}", sample)
	}

	let modify_sample_vec: Vec<i32> = sample_vec.iter().map(|x| x*5).collect();
	println!("Hasil map : {:?}", modify_sample_vec)
}

fn menggunakan_hash_map() {
	let mut score = HashMap::new();
	score.insert("A", 10);
	score.insert("B", 20);
	score.insert("C", 30);
	score.insert("D", 40);

	for (key, value) in &score {
		println!("{} nilainya {}", key, value);
	}

	for (_, value) in score.iter_mut() {
		*value += 3;
	}
	println!("modify hash_map : {:#?}", score);
}