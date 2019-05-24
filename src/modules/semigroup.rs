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

    let maximal_input: usize = *(inputnumbers.iter().max().unwrap());
    let width=2*maximal_input;
    let m: usize = *(inputnumbers.iter().min().unwrap());
    let mut aperyset: Vec<usize> = vec![0usize; m];
    let mut count_set = 0usize;
    let mut window = vec![-1isize; width]; // fenster hat die länge 2m
    let mut i: usize = m;
    let mut h: usize = 0; // verbrauchte fenster
    let mut windowindex = m;
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
        } else {
            for k in inputnumbers.iter() {
                if windowindex >= *k && window[windowindex - k] >= 0 {
                    count_set += 1;
                    runlength += 1;
                    hit = true;
                    window[windowindex] = i as isize;
                    if 0 == aperyset[residue] {
                        aperyset[residue] = i;
                    }
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
                window[j] = window[j + m];
                window[j + m] = -1;
            }
            h = h + 1;
            windowindex = maximal_input;
        } else {
            windowindex += 1;
        }
    }

    Semi::new(count_set-m+1, aperyset, m, minimal_generators)
}

