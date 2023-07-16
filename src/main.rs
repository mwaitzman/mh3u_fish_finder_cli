use std::{str::FromStr, fmt::Display};

#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type unsigned = u32;
#[allow(non_camel_case_types)]
type array<t> = Vec<t>;
#[allow(non_camel_case_types)]
type long__long = i64;


#[allow(dead_code)]
static ITEMS: [&str; 12] = [
	"",
	"Shining Starfish x1",
	"Pin Tuna x2",
	"Shining Starfish x2",
	"Sushifish x2",
	"Popfish x5",
	"Sushifish x3",
	"Whetfish x6",
	"Sleepyfish x2",
	"Popfish x4",
	"Whetfish x4",
	"Humspun Conch"
];



fn rnd(n: int) -> int {
	if n == 0 {return 176}

	let x: int = n * 16 - n * 4 - n;
	let y: long__long = (x * 16).into();
	let z: long__long = y * 11367737 / 4294967296;
	let m: long__long = y - ( (y-z) / 2 + z) / 32768 * 65363;
	(m & 0xFFFF) as int
}


fn get_fish_index(roll: unsigned, fish: &array<unsigned>) -> unsigned {
	let mut total: unsigned = fish[0];
	let mut count: unsigned = 0;
	while total <= roll {
		count += 1;total += fish[count as usize];
	}
	count
}


fn reward_func(mut num: unsigned, rep: unsigned) -> unsigned {
	let mut i = 0;while i < rep {
		num = (num * 176) % 65363;

		i += 1;}
	num
}


fn array_equals(a: &array<unsigned>, r#as: unsigned, b: &array<unsigned>, bs: unsigned, len: unsigned) -> bool {
//	dbg!(&a, &r#as, &b, &bs, &len);
	let mut i: unsigned = 0;while i < len {
		if a[(r#as + i) as usize] != b[(bs + i) as usize] {return false}

		i += 1;}
	true
}




fn clamp<T: Ord>(val: T, min_val: T, max_val: T) -> T {
	::std::cmp::max(::std::cmp::min(val, max_val), min_val)
}


fn search_seed(seed: unsigned, fish_table: &array<unsigned>, items: &array<unsigned>) -> bool {
//	dbg!(&seed, &fish_table, &items);
	let mut qwe: array<unsigned> = vec![0; 13];
	let mut j: int = 0;while j < (qwe.len()).try_into().unwrap() {
		j+= 1;let roll: unsigned = reward_func(seed, j.try_into().unwrap()) % 100;
		qwe[(j-1) as usize] = get_fish_index(roll, fish_table);
	}
	let max_fishfinder: unsigned = (clamp(items.len(), 4, 7) - 4).try_into().unwrap();
	let mut i: unsigned = 0;while i <= max_fishfinder {
		if array_equals(&mut qwe, 0, items, 0, (items.len() - (i as usize)).try_into().unwrap())
			&& array_equals(&mut qwe, 10, items, (items.len() - (i as usize)).try_into().unwrap(), i) {
				return true;
			}
			i += 1;}
	false
}


fn check_seed(seed: int, fish_table: &array<unsigned>, items: &array<unsigned>) -> bool {
	let mut n: int = seed;
	loop {
		if search_seed(n.try_into().unwrap(), fish_table, items) {return true};

		n = rnd(n);

		if n == seed {return false}
	}
}


fn get_table_results(items: &array<unsigned>) -> unsigned {

	let table_seeds: array<int> = vec!{1, 15, 5, 13, 4, 3, 9, 12, 26, 18, 163, 401, 6, 2, 489, 802, 1203};
	let mut fish: array<unsigned> = vec!{20, 15, 5, 5, 10, 15, 5, 10, 5, 5, 5};

	//// bitfield of results
	let mut res: unsigned = 0;
	let mut t: unsigned = 0;while t < 17 {
		if check_seed(table_seeds[t as usize], &mut fish, items) {
			res |= 1 << t;
		}
	t += 1;}
assert_eq!(
	(0..17)
		.filter(|t| check_seed(table_seeds[*t as usize], &mut fish, items))
		.fold(0, |acc, e| acc | 1 << e),
	res
);
assert_eq!(
	res,
	{
		let mut packed_result = 0;
		for t in 0..17 {
			if check_seed(table_seeds[t as usize], &mut fish, items) {
				packed_result |= 1 << t;
			}
		}
		packed_result
	}
);
	res
}


#[derive(Debug,PartialEq)]
#[allow(dead_code)]
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
			"shiningstarfishx1" => Ok(ShiningStarfishx1),
			"pintunax2" => Ok(PinTunax2),
			"shiningstarfishx2" => Ok(ShiningStarfishx2),
			"sushifishx2" => Ok(Sushifishx2),
			"popfishx5" => Ok(Popfishx5),
			"sushifishx3" => Ok(Sushifishx3),
			"whetfishx6" => Ok(Whetfishx6),
			"sleepyfishx2" => Ok(Sleepyfishx2),
			"popfishx4" => Ok(Popfishx4),
			"whetfishx4" => Ok(Whetfishx4),
			"humspunconchx1" => Ok(HumspunConchx1),
			"humspunconch" => Ok(HumspunConchx1),
			_ => Err(input.to_owned())
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
		.map(|e| e.split_whitespace().collect())
		.map(|e: String| e.to_lowercase())
		.map(|e| e.parse().unwrap_or_else(|_| panic!("Couldn't parse {e} as a FishReward")))
		.collect()
}

fn main() {
    println!("Hello, world!");
	let mut line = String::new();
	while 0 != std::io::stdin().read_line(&mut line).unwrap() {
		if line == "\n" {
			continue;
		}
		dbg!(&line);
		let mut my_items: array<unsigned> = normalize_row_input(&line).into_iter().map(|e| e as unsigned).collect();

		let packed_result: unsigned = get_table_results(&mut my_items);
		let mut output = String::from("possible tables: {");
		for i in 0..17 {
			if (packed_result & (1 << i)) == (1 << i) {
				output += &format!("\n\tT{},", i + 1);
			}
		}
		output.pop();
		output += "\n}\n";
		println!("{output}");
		line.clear();
	}


/*	let possible_tables_for_this_row = (0..17)
		.filter(|i| ((result << i) & 1) == 1)
		.fold(String::new(), |acc, e| format!("{acc}, {e}"))
		.split_off(2);
	println!("{possible_tables_for_this_row}");*/
	println!("Goodbye, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn normalize_row_input_0() {
		use crate::normalize_row_input;
		let test_input = "Pin Tuna  x2, Sleepyfish x2,   SushiFISHx3,Humspun Conch";
		assert_eq!(
			normalize_row_input(test_input),
			{use crate::FishRewards::*;
			vec![
				PinTunax2,
				Sleepyfishx2,
				Sushifishx3,
				HumspunConchx1
			]}
		);
    }
}
