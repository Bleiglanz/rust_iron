use super::gcd_vec;

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
    pub e:usize,
    pub c:usize,
    pub a1:usize,
}

impl Semi {
    fn new(set:Vec<usize>, apery: Vec<usize>, g1: usize) -> Self {
        let max_a: usize = *apery.iter().max().unwrap();
        let min_a: usize = *apery[1..].iter().min().unwrap();
        let a1: usize = apery[1];
        let count_set = set.len();
        let sum = apery.iter().sum();
        let count_gap = (sum - ((g1-1)*g1)/2)/g1;
        let double_avg_a = 2 * sum / apery.len();
        let gen_set = vec![1usize;1];
        assert!(g1 < min_a);
        Semi {
            apery: apery,
            max_a: max_a,
            sum_a: sum,
            double_avg_a: double_avg_a,
            g1: g1,
            maxgap: max_a - g1,
            count_set: count_set,
            count_gap: count_gap,
            e: gen_set.len(),
            gen_set: gen_set,
            c: max_a - g1 + 1,
            a1: a1,
        }
    }
}





pub fn semi(inputnumbers: &[usize]) -> Semi {

    // teilerfremd machen und sortieren
    let d = gcd_vec(inputnumbers);
    let mut input: Vec<usize> = inputnumbers.iter().map(|x| (x / d) as usize).collect();
    input.sort();
    assert!(input.len() > 0 && 1 == gcd_vec(&input));
    let maximal_input: usize = *(input.iter().max().unwrap());
    let minimal_input: usize = *(input.iter().min().unwrap());
    let mut aperyset: Vec<usize> = vec![0usize; minimal_input];

    // neu: window hat länge 2m
    let mut result: Vec<usize> = Vec::with_capacity(1000);

    // fenster der länge 2g_1
    let m = minimal_input;
    let width = 2 * maximal_input;
    let mut window = vec![0usize;width]; // fenster hat die länge 2m
    window[0] = 1;

    let mut i: usize = m;
    let mut h: usize = 0; // verbrauchte fenster
    let mut lookupindex = m;

    let mut runlength = 1; // anzahl aufeinanderfoldender hits
    let upperbound = maximal_input * maximal_input+1;
    let mut hit:bool = false;

    while i < upperbound && runlength < minimal_input {

        for k in input.iter() {
            assert!(m<=lookupindex && lookupindex<width,"index");
            if  lookupindex >= *k && 1 == window[lookupindex-k] && 0 == window[lookupindex] {
                result.push(i);
                runlength += 1;
                hit = true;
                window[lookupindex]=1;
                let residue = i % m;
                if residue > 0 {
                    if 0 == aperyset[residue] {
                        aperyset[residue] = i;
                    }
                }
                continue;
            }
        }
        if !hit { runlength = 0 };
        hit = false;
        i += 1;
        if lookupindex == width-1 {
            for j in 0..maximal_input{
                window[j]=window[j+m];
                window[j+m]=0;
            }
            h=h+1;
            lookupindex=m;
        } else {
            lookupindex += 1;
        }
    }
    Semi::new(result, aperyset, minimal_input)
}