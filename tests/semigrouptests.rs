extern crate params;
extern crate mime;
extern crate crossbeam;
extern crate rust_iron;
extern crate rand;

use rand::Rng;
use rust_iron::modules::wilf::{WilfSet, generatewilf};
use rust_iron::modules::semigroup::{Semi, semi};
use crossbeam::thread;
use rust_iron::modules::fast_semigroup::{Fast, fast};

fn computation(primes: &[usize], task: (usize, usize), factor:usize) {
    let start = task.0;
    let stop = task.1;

    fn findmaxindex(s:&[usize], start: usize, factor:usize) -> usize {
        let mut max = start;
        loop {
            if s[max] < factor * s[start] {
                max = max + 1;
            } else {
                break;
            }
        }
        max
    }

    for skip in start..stop {
        let maxindex :usize = findmaxindex(&primes, skip, factor);
        let res2: Fast = fast(&primes[skip..maxindex]);
        let resf: Semi = semi(&primes[skip..maxindex]);
        let res: WilfSet = generatewilf(&primes[skip..maxindex]);
        assert_eq!(res.apery, resf.apery);
        assert_eq!(res.gen_set, resf.gen_set);
        assert_eq!(res.e, resf.e);
        assert_eq!(res.c, resf.c);
        assert_eq!(res.count_set, resf.count_set);
        assert_eq!(resf.max_a, res2.max_a);
        assert_eq!(resf.sum_a, res2.sum_a);
        assert_eq!(resf.count_set, res2.count_set);
        assert_eq!(resf.e, res2.e);
        assert_eq!(resf.c, res2.c);
        assert_eq!(resf.count_set, res2.count_set);
        assert_eq!(resf.count_gap, res2.count_gap);
    }
}

fn mainprimes(cores: usize, start: usize, stop: usize, factor:usize) {
    let interval = (stop - start) / cores;
    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for ti in 0..cores {
        tasks.push((start + ti * interval, start + (ti + 1) * interval))
    }

    let primesvec: Vec<usize> = primal::Primes::all().take(8000000).collect();
    let primes:&[usize] = &primesvec;
    thread::scope(|s| for task in &tasks {
        let sta = task.0;
        let sto = task.1;
        s.spawn(move |_| {
            computation(primes, (sta, sto),  factor);
        });
    }).unwrap();
}

#[test]
fn test_all_primes() {
    mainprimes(4,100,500, 4);
}


#[test]
fn test_random() {
    let mut rng = rand::thread_rng();
    use rand::distributions::{Distribution, Uniform};
    let between = Uniform::from(10..100);
    let mut res:Vec<usize> = Vec::new();
    for _ in 0..1000 {
        res.push(between.sample(&mut rng));
    }
    assert_eq!(res.len(),1000);
}



