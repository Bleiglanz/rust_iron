use super::gcd_vec;

#[derive(Debug)]
pub struct WilfSet {
    defect:usize,
    pub set: Vec<u8>,
    pub apery: Vec<usize>,
    max_a: usize,
    sum_a: usize,
    pub double_avg_a: usize,
    pub maxgap: usize,
    pub g1: usize,
    pub count_set: usize,
    pub count_gap: usize,
    gen_flags: Vec<usize>,
    pub gen_set: Vec<usize>,
    pub e:usize,
    pub c:usize,
    pub lambda_matrix:Vec<Vec<usize>>,
    pub a1:usize,
}

impl WilfSet {
    fn new(set: Vec<u8>, apery: Vec<usize>, g1: usize) -> WilfSet {
        let max_a: usize = *apery.iter().max().unwrap();
        let min_a: usize = *apery[1..].iter().min().unwrap();
        let a1:usize = apery[1];
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
            a1:a1,
        }
    }

    pub fn to_html(&self,title:&String) -> String {
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
        html.push_str(&get_apery_for_modulus(&self,self.c));

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

pub fn generatewilf(inputnumbers: &[usize]) -> WilfSet {
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


pub fn get_apery_for_modulus(wilf: &WilfSet, s: usize) -> String {
    let mut res = String::with_capacity(1000);
    res.push_str(&format!(
        "<div class='l-box-lrg pure-u-1 pure-u-md-5-5'>Apery mod {} <table>",
        s
    ));

    let mut apery_s: Vec<Option<usize>> = vec![None; s];
    apery_s[0] = Some(0);
    for index in 1..wilf.c + s {
        let modul = index % s;
        if index >= wilf.c || 1u8 == wilf.set[index] {
            apery_s[modul] = match apery_s[modul] {
                None => Some(index),
                Some(current) => {
                    if index < current {
                        Some(index)
                    } else {
                        Some(current)
                    }
                }
            };
        }
    }

    //Apery
    res.push_str("<tr>");
    for i in 0..s {
        res.push_str(&format!("<td style='border:1px solid orange'>{}</td>", apery_s[i].unwrap()));
    }
    res.push_str("<td>&nbsp;</td></tr>");

    // Elemente:
    res.push_str("<tr>");
    for i in 0..s {
        let flag = if 1u8 == wilf.set[i] {
            i.to_string()
        } else {
            "-".to_string()
        };
        res.push_str(&format!("<td>{}</td>", flag))
    }
    res.push_str("<td>&nbsp;</td></tr>");

    res.push_str("<tr>");
    for i in 0..s {
        let flag = if 1u8 == wilf.set[i] { '\u{2b1b}' } else { '_' };
        res.push_str(&format!("<td>&nbsp;{}&nbsp;</td>", flag))
    }
    res.push_str("<td>maxgap!</td></tr>");

    let mut diag: Vec<usize> = vec![0; s];
    for d in 0..s {
        for i in 0..s {
            let j = (s + d - i) % s;
            let ap_i = apery_s[i].unwrap();
            let ap_j = apery_s[j].unwrap();
            let ap_ij = apery_s[(i + j) % s].unwrap();
            let l_ij = (ap_i + ap_j - ap_ij) / s;
            diag[d] += l_ij;
        }
    }

    let mut summ_of_all_apery = 0;
    for i in 0..s {
        let mut zeilensumme = 0;
        res.push_str("<tr>");
        for j in 0..s {
            let ap_i = apery_s[i].unwrap();
            let ap_j = apery_s[j].unwrap();
            let ap_ij = apery_s[(i + j) % s].unwrap();
            let l_ij = (ap_i + ap_j - ap_ij) / s;
            zeilensumme += l_ij;
            res.push_str(&format!(
                "<td style='border:1px solid black'><strong>&nbsp;{}&nbsp;</strong></td>",
                l_ij
            ));
        }
        res.push_str(&format!(
            "<td>sum={} apery_mod_s[{}]={} diag[{}]={}",
            zeilensumme,
            i,
            apery_s[i].unwrap(),
            i,
            diag[i]
        ));
        summ_of_all_apery += apery_s[i].unwrap();
        res.push_str("</tr>");
    }

    res.push_str("<tr>");
    for i in 0..s {
        res.push_str(&format!("<td style='border:1px solid grey'>{}</td>", (apery_s[i].unwrap()-i)/s));
    }
    res.push_str("<td>&nbsp;=q</td></tr>");

    res.push_str(&format!("</table>Summe aller Apery{}, doppelter Durchschnitt {}, gaps {} * s {} + s(s-1)/2 ={} </div>",summ_of_all_apery,2*summ_of_all_apery/s, wilf.count_gap, s, wilf.count_gap*s + s*(s-1)/2));
    res
}
