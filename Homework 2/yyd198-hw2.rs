use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};

fn is1stDeletion(src: String, des: String) -> bool{
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 1 && srcInVector != desInVector && srcInVector.len() == (desInVector.len() + 1) {
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.insert(i, srcInVector[i]);
		}
	}
	result = result || (srcCopy == desCopy);
	return result;
}
fn is1stTransposition(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 2 && srcInVector != desInVector && srcInVector.len() == desInVector.len() {
			let mut i: usize = 0;
			while i < desInVector.len() - 1 {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < desInVector.len() - 1 {
				let exchange = 	desInVector[i];
				desInVector[i] = desInVector[i + 1];
				desInVector[i + 1] = exchange;
			}	
		}
	}
	result = result || (srcCopy == desCopy);
	return result;
}
fn is1stReplacement(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 1 && desInVector.len() >= 1 && srcInVector != desInVector && srcInVector.len() == desInVector.len() {
			let mut cnt: usize = 0;
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					cnt = cnt + 1;
				}
				i = i + 1;
			}
			result = (cnt == 1);
		}
	}
	return result;	
}
fn is1stInsertion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 1 && desInVector.len() >= 2 && srcInVector != desInVector && srcInVector.len() == desInVector.len() - 1 {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.remove(i);
		}
	}
	result = result || (srcCopy == desCopy);
	return result;		
}
fn is1stEdit(src: String, des: String) -> bool {
	let mut result: bool = false;
	if src != des {
		result = result || is1stDeletion(src.clone(), des.clone());
		result = result || is1stTransposition(src.clone(), des.clone());
		result = result || is1stReplacement(src.clone(), des.clone());
		result = result || is1stInsertion(src.clone(), des.clone());
	}
	return result;
}
fn is2ndDeletionAndDeletion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 3 && desInVector.len() >= 1 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() + 2 {
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.insert(i, srcInVector[i]);
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.insert(i, srcInVector[i]);				
		}
	}
	result = result || (srcCopy == desCopy);
	return result;
}
fn is2ndDeletionAndTransposition(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		if srcInVector.len() >= 3 && desInVector.len() >= 2 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() + 1 {
			let mut i: usize = 0;
			while i < desInVector.len() - 1 {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < desInVector.len() - 1 {
				let exchange = desInVector1[i];
				desInVector1[i] = desInVector1[i + 1];
				desInVector1[i + 1] = exchange;
				desInVector2.insert(i, srcInVector[i]);
			}
		}
	}
	result = is1stDeletion(srcCopy.clone(), desCopy1.clone());
	result = result || is1stTransposition(srcCopy.clone(), desCopy2.clone());
	return result;	
}
fn is2ndDeletionAndReplacement(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 1 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() + 1 {
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < desInVector.len() {
				desInVector1.insert(i, srcInVector[i]);
				desInVector2[i] = srcInVector[i];
			}
		}		
	}
	result = is1stReplacement(srcCopy.clone(), desCopy1.clone());	
	result = result || is1stDeletion(srcCopy.clone(), desCopy2.clone());
	return result;	
}
fn is2ndDeletionAndInsertion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 2 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < srcInVector.len() - 1 {
				desInVector1.remove(i);
				desInVector2.insert(i, srcInVector[i]);
			}
		}	
	}
	result = is1stDeletion(srcCopy.clone(), desCopy1.clone());
	result = result || is1stInsertion(srcCopy.clone(), desCopy2.clone());
	return result;
}
fn is2ndTranspositionAndTransposition(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		if srcInVector.len() >= 3 && desInVector.len() >= 3 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < srcInVector.len() - 2 {
				let mut exchange = desInVector1[i].clone();
				desInVector1[i] = desInVector1[i + 2].clone();
				desInVector1[i + 2] = exchange.clone();
				exchange = desInVector1[i + 1].clone();
				desInVector1[i + 1] = desInVector1[i + 2].clone();
				desInVector1[i + 2] = exchange.clone();
				exchange = desInVector2[i].clone();
				desInVector2[i] = desInVector2[i + 1].clone();
				desInVector2[i + 1] = exchange.clone();
				i = 0;
				while i < srcInVector.len() {
					if srcInVector[i] != desInVector2[i] {
						break;
					}
					i = i + 1;
				}
				if i < desInVector2.len() - 1 {
					exchange = desInVector2[i].clone();
					desInVector2[i] = desInVector2[i + 1].clone();
					desInVector2[i + 1] = exchange.clone();					
				}
			}
		}
	}
	result = (srcCopy == desCopy1);
	result = result || (srcCopy == desCopy2);
	return result;
}
fn is2ndTranspositionAndReplacement(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1a = des.clone();
	let mut desCopy1b = des.clone();
	let mut desCopy2 = des.clone();
	let mut desCopy3 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1a = desCopy1a.as_mut_vec();
		let mut desInVector1b = desCopy1b.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		let mut desInVector3 = desCopy3.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 2 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < srcInVector.len() - 1 {
				let mut exchange = desInVector1a[i].clone();
				desInVector1a[i] = desInVector1a[i + 1].clone();
				desInVector1a[i + 1] = exchange.clone();
				desInVector1a[i] = srcInVector[i].clone();
				exchange = desInVector1b[i].clone();
				desInVector1b[i] = desInVector1b[i + 1].clone();
				desInVector1b[i + 1] = exchange.clone();
				desInVector1b[i + 1] = srcInVector[i + 1].clone();
			}
			if i < srcInVector.len() - 2 {
				desInVector2[i] = srcInVector[i];
				let mut exchange = desInVector3[i].clone();
				desInVector3[i] = desInVector3[i + 1].clone();
				desInVector3[i + 1] = exchange.clone();
			}
		}
	}
	result = (desCopy1a == srcCopy);
	result = result || (desCopy1b == srcCopy);
	result = result || is1stTransposition(srcCopy.clone(), desCopy2.clone());
	result = result || is1stReplacement(srcCopy.clone(), desCopy3.clone());	
	return result;
}
fn is2ndTranspositionAndInsertion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	let mut desCopy3 = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		let mut desInVector3 = desCopy3.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 3 && srcInVector != desInVector && !is1stEdit(src.clone(), des.clone()) && srcInVector.len() + 1 == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < desInVector.len() - 2 {
				desInVector1.remove(i);
				let mut exchange = desInVector2[i].clone();
				desInVector2[i] = desInVector2[i + 1].clone();
				desInVector2[i + 1] = exchange.clone();
				exchange = desInVector3[i].clone();
				desInVector3[i] = desInVector3[i + 2].clone();
				desInVector3[i + 2] = exchange.clone();
				desInVector3.remove(i + 1);
			}
		}
	}
	result = is1stTransposition(srcCopy.clone(), desCopy1.clone());
	result = result || is1stInsertion(srcCopy.clone(), desCopy2.clone());
	result = result || (srcCopy == desCopy3);
	return result;
}
fn is2ndReplacementAndReplacement(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 2 && desInVector.len() >= 2 && srcInVector != desInVector && srcInVector.len() == desInVector.len() {
			let mut cnt: usize = 0;
			let mut i: usize = 0;
			while i < desInVector.len() {
				if srcInVector[i] != desInVector[i] {
					cnt = cnt + 1;
				}
				i = i + 1;
			}
			result = (cnt == 2);
		}
	}
	return result;	
}
fn is2ndReplacementAndInsertion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	let mut desCopy1 = des.clone();
	let mut desCopy2 = des.clone();
	unsafe {
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		let mut desInVector1 = desCopy1.as_mut_vec();
		let mut desInVector2 = desCopy2.as_mut_vec();
		if srcInVector.len() >= 1 && desInVector.len() >= 2 && srcInVector != desInVector && srcInVector.len() + 1 == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			if i < desInVector.len() - 1 {
				desInVector1.remove(i);
				desInVector2[i] = srcInVector[i].clone();
			}
		}		
	}
	result = is1stReplacement(srcCopy.clone(), desCopy1.clone());
	result = result || is1stInsertion(srcCopy.clone(), desCopy2.clone());
	return result;	
}
fn is2ndInsertionAndInsertion(src: String, des: String) -> bool {
	let mut result: bool = false;
	let mut srcCopy = src.clone();
	let mut desCopy = des.clone();
	unsafe{
		let mut srcInVector = srcCopy.as_mut_vec();
		let mut desInVector = desCopy.as_mut_vec();
		if srcInVector.len() >= 1 && desInVector.len() >= 3 && srcInVector != desInVector && srcInVector.len() + 2 == desInVector.len() {
			let mut i: usize = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.remove(i);
			i = 0;
			while i < srcInVector.len() {
				if srcInVector[i] != desInVector[i] {
					break;
				}
				i = i + 1;
			}
			desInVector.remove(i);			
		}		
	}
	result = (desCopy == srcCopy);
	return result;	
}
fn is2ndEdit(src: String, des: String) -> bool {
	let mut result: bool = false;
	if src != des {
		result = result || is2ndDeletionAndDeletion(src.clone(), des.clone());
		result = result || is2ndDeletionAndTransposition(src.clone(), des.clone());
		result = result || is2ndDeletionAndReplacement(src.clone(), des.clone());
		result = result || is2ndDeletionAndInsertion(src.clone(), des.clone());
		result = result || is2ndTranspositionAndTransposition(src.clone(), des.clone());
		result = result || is2ndTranspositionAndReplacement(src.clone(), des.clone());
		result = result || is2ndTranspositionAndInsertion(src.clone(), des.clone());
		result = result || is2ndReplacementAndReplacement(src.clone(), des.clone());
		result = result || is2ndReplacementAndInsertion(src.clone(), des.clone());
		result = result || is2ndInsertionAndInsertion(src.clone(), des.clone());
	}
	return result;
}


// fn read_and_correct<R:Read>(reader:R) -> Vec<String> {
// 	let mut wordsInVector: Vec<String> = Vec::new();
// 	let mut words = BufReader::new(reader).lines();
// 	while let Some(Ok(word)) = words.next(){
// 		wordsInVector.push(word);
// 	}
// 	return wordsInVector;
// }

fn main(){
	// correct

	// read_and_correct(stdin())
	// let f = File::open("./corpus.txt").unwrap();
	// let mut lines = BufReader::new(f).lines();
	// while let Some(Ok(word)) = lines.next() {
	// 	if word.len() >= 1 {
	// 		corpusInHashSet.insert(word);
	// 	}
	// }
	// for i in &corpusInHashSet {
	// 	println!("{} {}", i, i.len());
	// }
	//  cargo run < input.txt
	let mut corpus = String::new();
	let f1 = File::open("./corpus.txt").unwrap();
	let mut br1 = BufReader::new(f1);
	br1.read_to_string(&mut corpus).unwrap();
	let corpusInVector: Vec<&str> = corpus.split(|c| c == ' ' || c == '\n').filter(|s| !s.is_empty()).collect();
	let mut corpusInHashSet: HashSet<String> = HashSet::new();
	for i in &corpusInVector {
		if i.len() >= 1 {
			corpusInHashSet.insert(i.to_owned().to_string());
		}
	}
	// for i in &corpusInHashSet {
	// 	println!("{} {}", i, i.len());
	// }
	// println!("\n");
	let mut inputInVector: Vec<String> = Vec::new();
	let f2 = File::open("./input.txt").unwrap();
	let mut lines = BufReader::new(f2).lines();
	while let Some(Ok(word)) = lines.next() {
		if word.len() >= 1 {
			inputInVector.push(word.to_owned().to_string());
		}
	}
	// for i in &inputInVector {
	// 	println!("{} {}", i, i.len());
	// }
	// println!("{}", "Yuanhui".to_string() == "Yuanhui".to_string());
	let mut output: HashMap<String, String> = HashMap::new();
	let mut foundEq: bool = false;
	let mut found1st: bool = false;	
	let mut found2nd: bool = false;
	for i in &inputInVector {
		for j in &corpusInHashSet {
			if !foundEq && i == j {
				output.insert(i.to_owned().to_string(), " ".to_string());
				println!("{}", i.to_owned().to_string());
				foundEq = true;
				break;
			}
		}
		for k in &corpusInHashSet {
			if foundEq {
				break;
			}
			if !foundEq && !found1st && is1stEdit(i.to_string().clone(), k.to_string().clone()) {
				println!("{}, {}", i.to_owned().to_string(), k.to_string().clone());
				output.insert(i.to_string().clone(), k.to_string().clone());
				found1st = true;
				break;
			} 
		}
		for l in &corpusInHashSet {
			if foundEq || found1st {
				break;
			}
			if !foundEq && !found1st && !found2nd && is2ndEdit(i.to_string().clone(), l.to_string().clone()) {
				println!("{}, {}", i.to_owned().to_string(), l.to_string().clone());
				output.insert(i.to_string().clone(), l.to_string().clone());
				found2nd = true;
				break;
			}
		}
		if !foundEq && !found1st && !found2nd {
			println!("{}, -", i.to_owned().to_string());
			output.insert(i.to_string().clone(), "-".to_string());
		}
		foundEq = false;
		found1st = false;
		found2nd = false;			
	}
}