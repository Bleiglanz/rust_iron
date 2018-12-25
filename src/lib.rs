extern crate rand;
use rand::Rng;
use rand::prelude::*;
use std::str::FromStr;

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


pub fn from_string_input(numbers_str: &str, samples_str: &str) -> String {
    let samples: usize = usize::from_str(samples_str.trim()).unwrap_or(1);
    let numbers_str_vec = numbers_str.split_whitespace();
    let input: Vec<usize> = numbers_str_vec
        .map(|s| usize::from_str(s).unwrap_or(0))
        .filter(|n| *n > 1)
        .collect();
    let wilf_set = wilf(&input);
    let mut html = String::new();
    let mainresult = format!(
        "Eingabe: {:?} und {} ergibt {:?} und {}",
        input,
        samples,
        wilf_set,
        wilf_set.to_html()
    ).to_string();
    html.push_str(&mainresult);
    let mut rng = rand::thread_rng();
    let mut samplecount = 0;
    while samplecount < samples {
        let max_gen = *wilf_set.gen_set.iter().max().unwrap();
        let w = generate_random_input(wilf_set.g1, wilf_set.e, max_gen, &mut rng);
        if w.e == wilf_set.e && w.g1==wilf_set.g1 && wilf_set.g1 > 1 {
            html.push_str(&w.to_html());
            samplecount += 1;
        }
    }
    html
}

fn generate_random_input(g1:usize, e:usize, max:usize, rng:&mut ThreadRng) -> WilfSet {
    let mut randomsample: Vec<usize> = Vec::new();
    let mut gcd = 0;
    while 1 != gcd {
        randomsample.clear();
        randomsample.push(g1);
        for _ in 0..e-1 {
             randomsample.push(rng.gen_range(g1+1, max ));
        }
        gcd = gcd_vec(&randomsample);
    }
    wilf(&randomsample)
}


#[derive(Debug)]
struct WilfSet {
    set: Vec<u8>,
    apery: Vec<usize>,
    max_a: usize,
    sum_a: usize,
    double_avg_a: usize,
    maxgap: usize,
    g1: usize,
    count_set: usize,
    count_gap: usize,
    gen_flags: Vec<usize>,
    gen_set: Vec<usize>,
    e:usize,
}

impl WilfSet {
    fn new(set: Vec<u8>, apery: Vec<usize>, g1: usize) -> WilfSet {
        let max_a: usize = *apery.iter().max().unwrap();
        let min_a: usize = *apery[1..].iter().min().unwrap();
        let count_set = *(&set[0..(max_a - g1 + 1)].iter().map(|s| *s as usize).sum());
        let count_gap = *(&set[0..(max_a - g1 + 1)]
            .iter()
            .map(|s| (1 - s) as usize)
            .sum());
        let sum = apery.iter().sum();
        let double_avg_a = 2 * sum / apery.len();
        let gen_flags = find_generator_flags(&apery, g1);
        let mut gen_set: Vec<usize> = Vec::new();
        for a in gen_flags.iter() {
            if *a > 0 {
                gen_set.push(*a)
            };
        }
        gen_set.sort();
        assert!(g1 < min_a);
        assert_eq!(0, set.len() % g1);
        WilfSet {
            set: set,
            apery: apery,
            max_a: max_a,
            sum_a: sum,
            double_avg_a: double_avg_a,
            g1: g1,
            maxgap: max_a - g1,
            count_set: count_set,
            count_gap: count_gap,
            gen_flags: gen_flags,
            e: gen_set.len(),
            gen_set: gen_set,
        }
    }

    fn to_html(&self) -> String {
        let height = self.set.len() / self.g1;
        let defect = 2*(self.e-1)*self.max_a - self.e*self.double_avg_a-(self.e-2)*(self.g1-1);
        let mut html = String::new();
        html.push_str(&format!("<p title=\"{:?}\">{:?} ->e={},c={},#u={},#g={}  defects=<strong>{} {}</strong></p>",
                                           self,self.gen_set,self.e,self.maxgap+1,self.count_set, self.count_gap, defect, self.e*self.count_gap - (self.maxgap+1)));
        html.push_str("<table class=\"wilf\" width=\"100%\">");
        for r in 0..height {
            html.push_str("<tr>");
            for c in 0..self.g1 {
                let index = (height - r - 1) * self.g1 + c;
                let is_apery = self.apery[c] == index;
                let is_gen   = self.gen_flags[c] == index;
                if 1 == self.set[index] && !is_apery && !is_gen{
                    html.push_str(&format!(
                        "<td title=\"{}\" class=\"wilf-full\">&nbsp</td>",
                        index
                    ));
                } else if 1 == self.set[index] && is_gen {
                    html.push_str(&format!(
                        "<td title=\"{}\" class=\"wilf-generator\">&nbsp</td>",
                        index
                    ));
                } else if 1 == self.set[index] && is_apery && !is_gen {
                    html.push_str(&format!(
                        "<td title=\"{}\" class=\"wilf-apery\">&nbsp</td>",
                        index
                    ));
                } else {
                    html.push_str(&format!(
                        "<td title=\"{}\" class=\"wilf-empty\">&nbsp</td>",
                        index
                    ));
                };
            }
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        html
    }
}

fn find_generator_flags(apery: &Vec<usize>, g1: usize) -> Vec<usize> {
    let len = apery.len();
    let mut gen = apery.clone();
    gen[0]=g1;
    for i in 1..len {
        for j in 1..len {
            let index = (i + j) % len;
            let lambda = apery[i] + apery[j] - apery[index];
            assert!(0 == lambda % g1);
            if 0 == lambda {
                gen[index] = 0
            };
        }
    }
    gen
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
