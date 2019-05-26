//use super::gcd_vec;
//use std::collections::HashSet;

#[derive(Debug)]
pub struct Semi {
    pub apery: Vec<usize>,
    max_a: usize,
    sum_a: usize,
    double_avg_a: usize,
    pub maxgap: usize,
    pub g1: usize,
    pub count_set: usize,
    count_gap: usize,
    pub gen_set: Vec<usize>,
    pub e: usize,
    pub c: usize,
    pub a1: usize,
}

impl Semi {
    fn new(count_set: usize, apery: Vec<usize>, g1: usize, gens: Vec<usize>) -> Self {
        let max_a: usize = *apery.iter().max().unwrap();
        let min_a: usize = *apery[1..].iter().min().unwrap();
        let a1: usize = apery[1];
        let sum = apery.iter().sum();
        let count_gap = (sum - ((g1 - 1) * g1) / 2) / g1;
        let double_avg_a = 2 * sum / apery.len();
        let gen_set = gens;
        assert!(g1 < min_a);
        Semi {
            apery,
            max_a,
            sum_a: sum,
            double_avg_a,
            g1: g1,
            maxgap: max_a - g1,
            count_set,
            count_gap: count_gap,
            e: gen_set.len(),
            gen_set: gen_set,
            c: max_a - g1 + 1,
            a1: a1,
        }
    }
}


pub fn semi(inputnumbers: &[usize]) -> Semi {

    // nicht nötig wenn nur primzahlen richtig sortiert reinkommen
    // teilerfremd machen und sortieren
    // let d = gcd_vec(inputnumbers);
    // let mut input: Vec<usize> = inputnumbers.iter().map(|x| (x / d) as usize).collect();
    // input.sort();
    let maximal_input: usize = *inputnumbers.last().unwrap();
    let width=2*maximal_input;
    let m: usize = *inputnumbers.first().unwrap();
    let mut aperyset: Vec<usize> = vec![0; m];
    let mut count_set = 0usize;
    let mut window = vec![-1isize; width]; // fenster hat die länge 2max
    let mut i: usize = m; // startindex
    let mut windowindex = m; // am anfang = i
    let mut runlength = 1usize; // anzahl aufeinanderfoldender hits
    let mut hit: bool = false;
    let mut minimal_generators: Vec<usize> = Vec::with_capacity(m);
    minimal_generators.push(m);
    window[0]=0;
    while runlength < m {
        let residue = i % m;
        if 0 == residue {
            count_set += 1;
            runlength += 1;
            hit = true;
            window[windowindex] = i as isize;
        } else if aperyset[residue]>0 && i > aperyset[residue] {
            count_set += 1;
            runlength += 1;
            hit = true;
            window[windowindex] = i as isize;
        }
        else {
            for k in inputnumbers[1..].iter() {
                if windowindex >= *k && window[windowindex - k] >= 0 {
                    count_set += 1;
                    runlength += 1;
                    hit = true;
                    window[windowindex] = i as isize;
                    aperyset[residue] = i;
                    if 0==window[windowindex - k] {
                        minimal_generators.push(i);
                    }
                    break;
                }
            }
        }
        if !hit { runlength = 0 };
        hit = false;
        i += 1;
        if windowindex == width - 1 {
            for j in 0..maximal_input {
                window[j] = window[j + maximal_input];
            }
            windowindex = maximal_input;
        } else {
            windowindex += 1;
        }
    }
    Semi::new(count_set-m+1, aperyset, m, minimal_generators)
}

