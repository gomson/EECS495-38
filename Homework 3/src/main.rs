use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::env;

fn main(){
    start_to_end("a","d");
    start_to_end("a","b");
    start_to_end("a","c");
}

fn start_to_end(start_alph: &str, end_alph: &str){
	let mut args: Vec<_> = env::args().collect(); 
    let mut resultInReverseOrder: String =  String:: new();
    let mut result: String = String:: new();
    let mut visitedVertex: HashSet<String> = HashSet::new();
    let mut childToParent: HashMap<String, String> = HashMap::new();
    let mut inputInLine: Vec<String> = Vec::new();
    let f = File::open(&args[1]).expect("");
    let mut lines = BufReader::new(f).lines();
    let mut isFound: bool = false;
    while let Some(Ok(line)) = lines.next() {
        if line.len() >= 1 {
            inputInLine.push(line.to_owned().to_string());
        }
    }
    


    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in inputInLine {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut cnt: usize = 0;
        let mut key: String = String::new();
        // let mut value: HashSet<String> = HashSet::new();
        for i in 0..words.len() {
            if i == 0 {
                key = words[i].clone().to_owned().to_string();
                let mut keyCopy = key.clone();
                let mut valueCopy: HashSet<String> = HashSet::new();
                graph.insert(keyCopy.clone(), valueCopy.clone());               
            }
            else if i >= 1 {
                let mut keyCopy = key.clone().to_owned().to_string();
                let mut valueCopy = graph[&keyCopy].clone();
                valueCopy.insert(words[i].clone().to_owned().to_string());
                graph.insert(keyCopy.clone(), valueCopy.clone());               
            }
        }
    }



    for (key, value) in &graph.clone() {
        for elementInValue in value {
            let mut keyCopy = key.clone().to_owned().to_string();
            let mut elementInValueCopy = elementInValue.clone().to_owned().to_string();
            if keyCopy != elementInValueCopy {
                let mut valueCopy = graph[&elementInValueCopy].clone();
                valueCopy.insert(keyCopy.clone());
                graph.insert(elementInValueCopy.clone(), valueCopy.clone());
            }
        }
    }

    

    let mut start = String::new();
    let mut end = String::new();

    start = start_alph.to_owned().to_string();
    end = end_alph.to_owned().to_string();

    let mut dq: VecDeque<String> = VecDeque::new();

    dq.push_back(start.clone());

    let mut it = String::new();
    it = dq[0].clone();
    while(!dq.is_empty()){
        let mut valueCopy = graph[&it].clone();
        if valueCopy.contains(&end.clone()) {
            childToParent.insert(end.clone(), it.clone());
            isFound = true;
            break;
        }
        else if !valueCopy.contains(&end.clone()) {
            if !visitedVertex.contains(&it.clone()) {
                visitedVertex.insert(it.clone());
                for j in graph[&it].clone() {
                    dq.push_back(j.clone());
                    if !visitedVertex.contains(&j.clone()) {
                        childToParent.insert(j.clone(), it.clone());
                    }
                }
                dq.pop_front();
                it = dq[0].clone();
            }
            else if visitedVertex.contains(&it.clone()) {
                dq.pop_front();
                it = dq[0].clone();             
            }
        }
    }
    if isFound {
        let mut jt = end;
        while jt != start {
			resultInReverseOrder.push_str(" ");
            resultInReverseOrder.push_str(&jt);
            jt = childToParent[&jt].clone();
        }
		resultInReverseOrder.push_str(" ");
        resultInReverseOrder.push_str(&start.clone());
        result = resultInReverseOrder.clone();
        unsafe {
            let mut resultInVector = result.as_mut_vec();
            resultInVector.reverse();
        }
    }
    else if !isFound {
        result.push_str("No Exists.\n");
    }

    println!("{} -> {}: {}",start_alph.to_owned().to_string(),end_alph.to_owned().to_string(),result);


}
