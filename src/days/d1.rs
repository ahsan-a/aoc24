use std::{collections::HashMap, fs};

pub fn main() {
	let (mut s1, mut s2): (Vec<u32>, Vec<u32>) = fs::read_to_string("./inputs/1.txt")
		.unwrap()
		.lines()
		.map(|l| {
			let mut split = l.split_whitespace();
			(
				split.next().unwrap().parse::<u32>().unwrap(),
				split.next().unwrap().parse::<u32>().unwrap(),
			)
		})
		.unzip();

	s1.sort();
	s2.sort();

	let acc: u32 = s1.iter().zip(s2.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
	println!("pt1: {}", acc);

	let quants = s2.iter().fold(HashMap::new(), |mut acc, &x| {
		*acc.entry(x).or_insert(0) += 1;
		acc
	});

	let pt2: u32 = s1.iter().map(|&l| l * quants.get(&l).unwrap_or(&0)).sum();
	println!("{}", pt2);
}
