use super::WilfSet;

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
