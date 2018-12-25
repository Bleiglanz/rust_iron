fn gcd(mut m: usize, mut n: usize) -> usize {
    assert!(m != 0 && n != 0);
    while m > 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn gcd_vec(numbers: &[usize]) -> usize {
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    d
}

use std::io::Write;
use std::str::FromStr;

pub fn from_string_input(numbers_str: &str, samples_str: &str) -> String {
    let samples: usize = usize::from_str(samples_str.trim()).unwrap_or(1);
    let numbers_str_vec = numbers_str.split_whitespace();
    let input: Vec<usize> = numbers_str_vec
        .map(|s| usize::from_str(s).unwrap_or(0))
        .filter(|n| *n > 1)
        .collect();
    let wilf_set = wilf(&input);
    format!(
        "Eingabe: {:?} und {} ergibt {:?} und {}",
        input,
        samples,
        wilf_set,
        wilf_set.to_html()
    )
    .to_string()
}

#[derive(Debug)]
struct WilfSet {
    set: Vec<u8>,
    apery: Vec<usize>,
    max_a: usize,
    maxgap: usize,
    e: usize,
    g1: usize,
    count: usize,
}

impl WilfSet {
    fn new(set: Vec<u8>, apery: Vec<usize>, g1: usize) -> WilfSet {
        let max_a: usize = *apery.iter().max().unwrap();
        let min_a: usize = *apery[1..].iter().min().unwrap();
        let count = *(&set[0..(max_a - g1 + 1)].iter().map(|s| *s as usize).sum());
        assert!(g1 < min_a);
        assert_eq!(0, set.len() % g1);
        WilfSet {
            set: set,
            apery: apery,
            max_a: max_a,
            g1: g1,
            e: 0,
            maxgap: max_a - g1,
            count: count,
        }
    }

    fn to_html(&self) -> String {
        let height = self.set.len() / self.g1;
        let mut html = String::new();
        html.push_str("<table class=\"wilf\" width=\"100%\">");
        for r in 0..height {
            html.push_str("<tr>");
            for c in 0..self.g1 {
                let index = (height - r - 1) * self.g1 + c;
                let is_apery = self.apery[c] == index;
                if 1 == self.set[index] && !is_apery {
                    html.push_str(&format!("<td title=\"{}\" class=\"wilf-full\">&nbsp</td>",index));
                } else if 1 == self.set[index] && is_apery {
                    html.push_str(&format!("<td title=\"{}\" class=\"wilf-apery\">&nbsp</td>",index));
                } else {
                    html.push_str(&format!("<td title=\"{}\" class=\"wilf-empty\">&nbsp</td>",index));
                };
            }
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        html
    }
}

fn wilf(inputnumbers: &[usize]) -> WilfSet {
    // teilerfremd machen und sortieren
    let d = gcd_vec(inputnumbers);
    let mut input: Vec<usize> = inputnumbers.iter().map(|x| (x / d) as usize).collect();
    input.sort();
    assert!(input.len() > 0 && 1 == gcd_vec(&input));
    let maximal_input: usize = *(input.iter().max().unwrap());
    let minimal_input: usize = *(input.iter().min().unwrap());
    let limit: usize = maximal_input * maximal_input + minimal_input;
    let mut wilfset: Vec<u8> = vec![0u8; limit];
    let mut aperyset: Vec<usize> = vec![0usize; minimal_input];
    wilfset[0] = 1;
    let mut i: usize = 1;
    let mut runlength = 1;
    while i < limit && runlength < minimal_input {
        for k in input.iter() {
            if i >= *k && 1 == wilfset[i - k] && 0 == wilfset[i] {
                wilfset[i] = 1;
                runlength += 1;
                let residue = i % minimal_input;
                if residue > 0 {
                    if 0 == aperyset[residue] {
                        aperyset[residue] = i;
                    }
                }
            }
        }
        if 0 == wilfset[i] {
            runlength = 0;
        }
        i += 1;
    }
    let mut result: Vec<u8> = wilfset[0..i].to_vec();
    let missing = minimal_input - result.len() % minimal_input;
    for _ in 0..missing {
        result.push(1);
    }
    assert_eq!(0, result.len() % minimal_input);
    WilfSet::new(result, aperyset, minimal_input)
}

// fn main() {
//     let mut numbers = Vec::new();
//     for arg in std::env::args().skip(1) {
//         numbers.push(usize::from_str(&arg).unwrap());
//     }
//     if 0 == numbers.len() {
//         writeln!(std::io::stderr(), "Usage: rust_gcd NUMBER ..").unwrap();
//     } else {
//         let d = gcd_vec(&numbers[..]);
//         println!("gcd of {:?} is {}", numbers, d);
//         let mut minimal: Vec<usize> = numbers.iter().map(|x| (x / d) as usize).collect();
//         println!("minimal set is {:?}", minimal);
//         minimal.sort();
//         println!("minimal set sorted is {:?}", minimal);
//         let wilfmenge = wilf(&minimal);
//         println!("wilf of {:?} is {:?}", minimal, wilfmenge);

//         extern crate rand;
//         use rand::Rng;
//         for _ in 1..100 {
//             let mut rng = rand::thread_rng();
//             let mut randomsample: Vec<usize> = Vec::new();
//             randomsample.push(rng.gen_range(80, 120));
//             for _ in 1..10 {
//                 randomsample.push(rng.gen_range(100, 300));
//             }
//             let dr = gcd_vec(&randomsample[..]);
//             println!("gcd of {:?} is {}", randomsample, dr);
//             let mut minimal: Vec<usize> = randomsample.iter().map(|x| (x / dr) as usize).collect();
//             println!("minimal set is {:?}", minimal);
//             minimal.sort();
//             println!("minimal set sorted is {:?}", minimal);
//             wilf(&minimal);
//         }
//     }
// }
