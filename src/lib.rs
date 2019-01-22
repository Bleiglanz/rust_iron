extern crate rand;
use rand::Rng;
use rand::prelude::*;
use std::str::FromStr;

mod html;


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
    let mut rng = rand::thread_rng();
    let samples: usize = usize::from_str(samples_str.trim()).unwrap_or(1);
    let inputsets:Vec<String> = numbers_str.split(|c| c==';' || c==',' || c=='#' || c=='\n' || c=='\r').map(|s| s.trim().to_string()).filter(|s| s.len()>0).collect();
    let mut html = String::new();
    let mut setcounter:usize = 0;
    let mut sampledata:Option<WilfSet> = None;
    for inputset in inputsets {
        setcounter += 1;
        let numbers_str_vec = inputset.split_whitespace();
        let input: Vec<usize> = numbers_str_vec
            .map(|s| usize::from_str(s.trim()).unwrap_or(2))
            .filter(|n| *n > 1)
            .collect();
        if input.len() >= 2 {
            let wilf_set = wilf(&input);
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

fn addsamples(html:&mut String, inputwilf:&WilfSet, samples:usize, rng:&mut ThreadRng){
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



fn generate_random_input(w:&WilfSet, rng:&mut ThreadRng) -> WilfSet {
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
    wilf(&randomsample)
}


#[derive(Debug)]
pub struct WilfSet {
    defect:usize,
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
    c:usize,
    lambda_matrix:Vec<Vec<usize>>,
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
        let mut tmp_lambda_matrix = vec![vec![0;g1];g1];
        let gen_flags = find_generator_flags(&apery, g1, &mut tmp_lambda_matrix);
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
            defect: (gen_set.len() * count_set) - (max_a-g1+1),
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
            c : max_a - g1+1,
            lambda_matrix: tmp_lambda_matrix,
        }
    }

    fn to_html(&self,title:&String) -> String {
        let height = self.set.len() / self.g1;
        let mut html = String::new();
        html.push_str(r#"<div class="l-box-lrg pure-u-1 pure-u-md-5-5">"#);
        let mut copyable_genset = String::new();
            for n in &self.gen_set {
                copyable_genset.push_str(&n.to_string());
                copyable_genset.push_str(" ");
            }
        html.push_str(&format!("<h3>{} {}</h3>",title, copyable_genset));
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
        html.push_str("</table><hr/><br/>");
        // die lambda_matrix
        html.push_str("Apery mod g1 <table class='lambda'>");
        let mut max_apery=0;
        let mut min_diag=self.double_avg_a;
        for i in 0..self.g1{
            html.push_str("<tr>");
            let mut apery=0;
            for j in 0..self.g1 {
                html.push_str(&format!("<td class='lambda'>{}&nbsp</td>",self.lambda_matrix[i][j]));
                apery += self.lambda_matrix[i][j];
            }
            let diag = {
                let mut dsum =0;
                for k in 0..self.g1 {
                    dsum += self.lambda_matrix[k][(i+self.g1-k) % self.g1];
                }
                dsum
            };
            html.push_str(&format!("<td class='lambda'>apery[{}]<strong>{}</apery></td><td>diag[{}] {}</td><td>sum={}</td>",i,apery,i,diag,apery+diag));
            if apery>=max_apery { max_apery=apery};
            if diag<=min_diag {min_diag=diag};
            html.push_str("</tr>");
        }
        html.push_str(&format!("</table></div>"));

        //
        // neu
        // 
        html.push_str(&html::get_apery_for_modulus(&self,self.c));

        let mut res_html = String::new();
        res_html.push_str(r#"<div class="l-box-lrg pure-u-1 pure-u-md-5-5">"#);
        let wilfstr = &format!("<script>document.write(({}/{}).toFixed(4));</script>",self.count_set,self.c);
        res_html.push_str(&format!("{:?} <strong>e</strong>={},<strong>c</strong>={},<strong>#set</strong>={},<strong>#gaps=</strong>{}  <strong>e*#set-c=</strong>{} ratio: {}<br/>",
                                   self.gen_set,self.e,self.maxgap+1,self.count_set, self.count_gap, self.defect, wilfstr));
        res_html.push_str(&format!("max_apery {} min_diag {} ",max_apery,min_diag));
        res_html.push_str(&format!(" (e-2)*max_a {} - e*min_diag {} = &nbsp;&nbsp;{} verglichen mit (e-2)(g1-1) {}<br/>",
                                   (self.e-2)*max_apery,self.e*min_diag, (self.e-2)*max_apery-self.e*min_diag,(self.e-2)*(self.g1-1)));
        res_html.push_str(&format!("Apery {:?} Max gap {} double_avg {}",self.apery, self.c-1,self.double_avg_a));

        let mut neuerinput:String = String::new();
        for u in self.gen_set.iter() {
            neuerinput.push_str(&format!("{} ",u));			
		}
        res_html.push_str(&format!(r#"
            
            <form method="post" action="/">
            <input id="numbers" type="hidden" name="numbers" value="{}">
            <input id="samples" type="hidden" name="samples" value="100">
            <button type="submit" class="pure-button">hier weitermachen</button>
            </form>
            
        "#, neuerinput));
        res_html.push_str("</p></div>");

        if "Input"==title{
            html.push_str(&res_html);
            html.push_str("</div>");
            html
        } else {
            res_html
        }

    }
}

fn find_generator_flags(apery: &Vec<usize>, g1: usize, tmp_lambda:&mut Vec<Vec<usize>>) -> Vec<usize> {
    let len = apery.len();
    let mut gen = apery.clone();
    gen[0]=g1;
    for i in 1..len {
        for j in 1..len {
            let index = (i + j) % len;
            let lambda = apery[i] + apery[j] - apery[index];
            assert!(0 == lambda % g1);
            tmp_lambda[i][j] = lambda / g1;
            if 0 == lambda && index>0 {
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
