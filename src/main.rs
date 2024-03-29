//#![warn(clippy::pedantic)]
use std::{str::FromStr, fmt::Display};

fn rnd(n: i32) -> i32 {
	if n == 0 { return 176; }

	let x: i32 = n * 16 - n * 4 - n;
	let y: i64 = (x * 16).into();
	let z: i64 = y * 11367737 / 4294967296;
	let m: i64 = y - ( (y-z) / 2 + z) / 32768 * 65363;
	(m & 0xFFFF) as i32
}

fn get_fish_index(roll: u32, fish: &[u32]) -> u32 {
	let mut total: u32 = fish[0];
	let mut count: u32 = 0;
	while total <= roll {
		count += 1;
		total += fish[count as usize];
	}
	count
}

fn reward_func(mut num: u32, rep: u32) -> u32 {
	for _ in 0..rep {
		num = (num * 176) % 65363;
	}
	num
}

fn array_equals(a: &[u32], r#as: u32, b: &[u32], bs: u32, len: u32) -> bool {
	(0..len).all(|i| a[(r#as + i) as usize] == b[(bs + i) as usize])
}

fn search_seed(seed: u32, fish_table: &[u32], items: &Vec<u32>) -> bool {
	let qwe: Vec<u32> = (1..14).map(|j| {
		let roll: u32 = reward_func(seed, j.try_into().unwrap()) % 100;
		get_fish_index(roll, fish_table)
	}).collect();

	let max_fishfinder: u32 = (std::cmp::Ord::clamp(items.len(), 4, 7) - 4).try_into().unwrap();
	(0..=max_fishfinder).any(|i: u32| {
		array_equals(&qwe, 0, items, 0, (items.len() - (i as usize)).try_into().unwrap())
			&& array_equals(&qwe, 10, items, (items.len() - (i as usize)).try_into().unwrap(), i)
	})
}

fn check_seed(seed: i32, fish_table: &[u32], items: &Vec<u32>) -> bool {
	let mut n: i32 = seed;
	loop {
		if search_seed(n.try_into().unwrap(), fish_table, items) { return true; };

		n = rnd(n);

		if n == seed { return false; }
	}
}

fn get_table_results(items: &Vec<u32>) -> u32 {

	let table_seeds: Vec<i32> = vec![1, 15, 5, 13, 4, 3, 9, 12, 26, 18, 163, 401, 6, 2, 489, 802, 1203];
	let fish: Vec<u32> = vec![20, 15, 5, 5, 10, 15, 5, 10, 5, 5, 5];

	(0..17)
		.filter(|t| check_seed(table_seeds[*t as usize], &fish, items))
		.fold(0, |acc, e| acc | 1 << e)
}


#[derive(Debug,PartialEq)]
enum FishRewards {
	ShiningStarfishx1 = 0,
	PinTunax2 = 1,
	ShiningStarfishx2 = 2,
	Sushifishx2 = 3,
	Popfishx5 = 4,
	Sushifishx3 = 5,
	Whetfishx6 = 6,
	Sleepyfishx2 = 7,
	Popfishx4 = 8,
	Whetfishx4 = 9,
	HumspunConchx1 = 10,
}
impl FromStr for FishRewards {
	type Err = String;
	fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
		use FishRewards::*;
		match input {
			"shiningstarfishx1" | "shiningstarfish" => Ok(ShiningStarfishx1),
			"pintunax2" => Ok(PinTunax2),
			"shiningstarfishx2" => Ok(ShiningStarfishx2),
			"sushifishx2" => Ok(Sushifishx2),
			"popfishx5" => Ok(Popfishx5),
			"sushifishx3" => Ok(Sushifishx3),
			"whetfishx6" => Ok(Whetfishx6),
			"sleepyfishx2" => Ok(Sleepyfishx2),
			"popfishx4" => Ok(Popfishx4),
			"whetfishx4" => Ok(Whetfishx4),
			"humspunconchx1" | "humspunconch" => Ok(HumspunConchx1),
			_ => Err(format!("Couldn't parse {input} into a FishRewards"))
		}
	}
}
impl Display for FishRewards {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use FishRewards::*;
		write!(
			f,
			"{}",
			match self {
				ShiningStarfishx1 => "Shining Starfish x1",
				PinTunax2 => "Pin Tuna x2",
				ShiningStarfishx2 => "Shining Starfish x2",
				Sushifishx2 => "Sushifish x2",
				Popfishx5 => "Popfish x5",
				Sushifishx3 => "Sushifish x3",
				Whetfishx6 => "Whetfish x6",
				Sleepyfishx2 => "Sleepyfish x2",
				Popfishx4 => "Popfish x4",
				Whetfishx4 => "Whetfish x4",
				HumspunConchx1 => "Humspun Conch x1",
			}
		)
	}
}

fn normalize_row_input(row: &str) -> Vec<FishRewards> {
	row
		.split(',')
		.map(|e| e.split_whitespace().collect::<String>())
		.filter(|e| !e.is_empty())
		.map(|e| e.to_lowercase())
		.map(|e| e.parse().unwrap_or_else(|err| panic!("{err}")))
		.collect()
}

fn main() {
	let mut line = String::new();
	const ALL_TABLES_POSSIBLE: u32 = u32::MAX >> (u32::BITS - 17);
	let mut possible_tables = ALL_TABLES_POSSIBLE;
	while 0 != std::io::stdin().read_line(&mut line).unwrap() {
		if line == "\n" {
			if possible_tables != ALL_TABLES_POSSIBLE {
				println!("\n\npossible tables reset");
			} else { possible_tables = ALL_TABLES_POSSIBLE; }
			continue;
		}

		let my_items: Vec<u32> = normalize_row_input(&line)
			.into_iter()
			.map(|e| e as u32)
			.collect();
		let packed_result = get_table_results(&my_items);
		println!("possible tables from this row: {}",
				 (0..17)
				 .filter(|i| packed_result & (1 << i) == 1 << i)
				 .fold(String::new(), |acc, e| format!("{acc} {}", e + 1))
		);
		possible_tables &= packed_result;
		let mut output = String::from("possible tables: {");
		for i in 0..17 {
			if (possible_tables & (1 << i)) == (1 << i) {
				output += &format!("\n\tT{},", i + 1);
			}
		}
		output.pop();
		output += "\n}\n";
		println!("{output}");

		line.clear();
	}
}


#[cfg(test)]
mod tests {
    use crate::{normalize_row_input, get_table_results};

    #[test]
    fn test_normalize_row_input() {
		let test_input = ",,,Pin Tuna\t  x2, Sleepyfish x2,   SushiFISHx3,,Humspun Conch,";
		assert_eq!(
			crate::normalize_row_input(test_input),
			{use crate::FishRewards::*;
			vec![
				PinTunax2,
				Sleepyfishx2,
				Sushifishx3,
				HumspunConchx1
			]}
		);
    }

	#[test]
	fn input_should_be_table_1() {
		let test_input = "popfishx4, popfishx4, sushifishx3, pintunax2, shiningstarfish";
		let normalized_input = normalize_row_input(test_input)
			.into_iter()
			.map(|e| e as u32)
			.collect();
		assert_eq!(
			get_table_results(&normalized_input),
			1
		);
	}
	#[test]
	fn should_be_table_1_or_table_10() {
		let input = "sleepyfishx2, pintuna x2, popfish x5, shining starfish x2, shining starfish";
		assert_eq!(
			get_table_results(&normalize_row_input(input)
							  .into_iter()
							  .map(|e| e as u32)
							  .collect()
			),
			1 | (1 << 9)
		);
	}
}
