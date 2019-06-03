extern crate rand;

use rand::Rng;
use rand::prelude::*;
use std::str::FromStr;

pub mod semigroup;
pub mod wilf;
pub mod fast_semigroup;

pub fn gcd(mut m: usize, mut n: usize) -> usize {
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

pub fn from_string_input(numbers_str: &str, samples_str: &str) -> String {
    let mut rng = rand::thread_rng();
    let samples: usize = usize::from_str(samples_str.trim()).unwrap_or(1);
    let inputsets:Vec<String> = numbers_str.split(|c| c==';' || c==',' || c=='#' || c=='\n' || c=='\r').map(|s| s.trim().to_string()).filter(|s| s.len()>0).collect();
    let mut html = String::new();
    let mut setcounter:usize = 0;
    let mut sampledata:Option<self::wilf::WilfSet> = None;
    for inputset in inputsets {
        setcounter += 1;
        let numbers_str_vec = inputset.split_whitespace();
        let input: Vec<usize> = numbers_str_vec
            .map(|s| usize::from_str(s.trim()).unwrap_or(2))
            .filter(|n| *n > 1)
            .collect();
        if input.len() >= 2 {
            let wilf_set = wilf::generatewilf(&input);
            html.push_str(&wilf_set.to_html(&"Input".to_string()));
            if 1==setcounter {
                sampledata = Some(wilf_set);
            }
        }
    }
    match sampledata {
        Some(w) => addsamples(&mut html, &w, samples, &mut rng),
        None => (),
    }
    html
}

fn addsamples(html:&mut String, inputwilf:&wilf::WilfSet, samples:usize, rng:&mut ThreadRng){
    let mut old_c = inputwilf.c;
    let mut old_s = inputwilf.count_set;
    for _ in 0 .. samples {
        let w = generate_random_input(inputwilf,rng);
        let kleiner:bool = old_c * (w.count_set) < w.c * (old_s);
        if w.e ==inputwilf.e && w.g1==inputwilf.g1 && w.g1 > 1 && kleiner {
            html.push_str(&w.to_html(&"Sample".to_string()));
            old_s=w.count_set;
            old_c=w.c;
        }
    }
}

fn generate_random_input(w:&wilf::WilfSet, rng:&mut ThreadRng) -> wilf::WilfSet {
    let mut randomsample: Vec<usize> = Vec::new();
    let mut gcd = 0;
    let g1 = w.g1;
    let wmax = w.c;
    while 1 != gcd {
        randomsample.clear();
        randomsample.push(g1);
        for _ in 0..w.e-1 {
            if g1+1 < wmax {randomsample.push(rng.gen_range(g1+1, wmax ))};
        }
        gcd = gcd_vec(&randomsample);
    }
    wilf::generatewilf(&randomsample)
}


