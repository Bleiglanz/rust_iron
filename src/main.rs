extern crate iron;
extern crate params;
extern crate dotenv;
extern crate clap;
#[macro_use]
extern crate mime;
extern crate crossbeam;
extern crate rust_iron;

use clap::{Arg, App};
use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use std::env;
use rust_iron::modules::*;
use rust_iron::modules::wilf::{WilfSet, generatewilf};
use rust_iron::modules::semigroup::{Semi, semi};
use crossbeam::thread;
use rust_iron::modules::fast_semigroup::{Fast, fast};
use std::io::Write;

fn computation(primes: &[usize], modul:usize, residue:usize, task: (usize, usize), factor1: usize, factor2: usize, detail: bool, iterate:bool) {
    let start = task.0;
    let stop = task.1;

    fn findmaxindex(s: &[usize], start: usize, factor: usize) -> usize {
        if 1 == factor {
            start
        } else {
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
    }

    let mut out = std::fs::File::create(format!("./out_mod{}residue{},{}pto{}p_n{}to{}.csv", modul, residue, factor1, factor2, start, stop)).expect("Unable to create file");
    let head = "       n;     n+k;       k;modul; resi;fak1;fak2;     p_n;    p_n+1;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S); stable; f/p\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);


    for skip in start..stop {

        let minindex: usize = findmaxindex(&primes, skip, factor1);
        let maxindex: usize = findmaxindex(&primes, skip, factor2) + 1;



        let stable = fast(&primes[minindex..maxindex]);
        let startindex = if iterate {minindex+2} else {maxindex-1 };


        for i in startindex..maxindex {

            let first: usize = (primes[skip]).clone();
            let gens: &[usize] = &primes[minindex..i];
            if gens.len()<2 { continue };

            assert_eq!(first % modul, residue);


            let res2: Fast = if iterate { fast(&gens) } else { stable.clone() };//&primes[skip..maxindex]);
            let ausgabe = format!("{:8};{:8};{:8};{:5};{:5};{:4};{:4};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:.6}\n",
                                  skip + 1, i, i - (skip + 1),
                                  modul, residue,
                                  factor1, factor2,
                                  first, primes[minindex+1], primes[i - 1],
                                  res2.g1, res2.e,
                                  res2.count_set, res2.maxgap, res2.maxgap as i64 - (res2.maxgap as i64/ res2.g1 as i64) * res2.g1 as i64,
                                  if stable.maxgap==res2.maxgap && stable.count_set==res2.count_set {
                                      "stable S"
                                  } else {""},
                                  res2.maxgap as f64 / res2.g1 as f64,

            );
            print!("{}", ausgabe);

            if detail { print!("{}", ausgabe); }
            if detail {
                let resf: Semi = semi(&gens);//&primes[skip..maxindex]);
                let fausgabe = format!("{:6};Semi<{}p;f;{:6};m;{:6};e;{:6};S<f;{:8};f/p;{:.6};wilf;{:.6}",
                                       skip + 1, factor1,
                                       resf.maxgap, resf.g1, resf.e, resf.count_set,
                                       resf.maxgap as f64 / resf.g1 as f64, resf.count_set as f64 / resf.c as f64);
                println!("{}", fausgabe);

                let res: WilfSet = generatewilf(&gens);//&primes[skip..maxindex]);
                println!("{:6};Wilf <{}p;f;{:6};m;{:6};e;{:6};S<f;{:8};f/p;{:.6};wilf;{:.6}\n",
                         skip + 1, factor1,
                         res.maxgap, res.g1, res.e, res.count_set,
                         res.maxgap as f64 / res.g1 as f64, (res.count_set as f64) / (res.c as f64));
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

            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            if stable.maxgap==res2.maxgap && stable.count_set==res2.count_set {
                break;
            }
        }
    }
    println!("Task beendet {}-{}", start, stop);
}


fn mainprimes(cores: usize, modul:usize, residue:usize, start: usize, stop: usize, factor1: usize, factor2: usize, detail: bool, iterate:bool) {
    let interval = (stop - start) / cores;
    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for ti in 0..cores {
        tasks.push((start + ti * interval, start + (ti + 1) * interval))
    }

    let primesvec: Vec<usize> = primal::Primes::all().take(8000000).filter(|x|{residue==x%modul}).collect();

    let primes: &[usize] = &primesvec;
    println!("Primgruppen {:?}", tasks);
    thread::scope(|s| for task in &tasks {
        let sta = task.0;
        let sto = task.1;
        s.spawn(move |_| {
            computation(primes, modul, residue, (sta, sto), factor1, factor2, detail, iterate);
        });
    }).unwrap();
}


fn main() {
    let matches = App::new("semiprog")
        .version("0.0")
        .author("Anton Rechenauer")
        .about("compute frobenius")
        .arg(Arg::with_name("cores")
            .help("number of cores to use")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("modul")
            .help("the modulus, in which arithmetic progression to search")
            .required(true)
            .default_value("2")
        )
        .arg(Arg::with_name("residue")
            .help("the residue, consider only primes congruent this mod modul")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("start")
            .help("where to begin, a n th prime")
            .required(true)
            .default_value("10")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime")
            .required(true)
            .default_value("12")
        )
        .arg(Arg::with_name("factor1")
            .help("take all primes as generators factor1*p_start <= gen  < factor2*p_start")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("factor2")
            .help("take all primes as generators factor1*p_start <= gen  < factor2*p_start")
            .required(true)
            .default_value("6")
        )
        .arg(Arg::with_name("iterate")
            .help("if 1, take all intermediate semigroups p_n....p_n+k")
            .required(true)
            .default_value("0")
        )
        .arg(Arg::with_name("detail")
            .help("if 1, show details and comparisons")
            .required(true)
            .default_value("0")
        )
        .get_matches();

    let cores: usize = matches.value_of("cores").unwrap().parse().unwrap();
    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();
    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();
    let factor1: usize = matches.value_of("factor1").unwrap().parse().unwrap();
    let factor2: usize = matches.value_of("factor2").unwrap().parse().unwrap();
    let iterate: usize = matches.value_of("iterate").unwrap().parse().unwrap();
    let detail: &str = matches.value_of("detail").unwrap();
    if cores > 0 {
        mainprimes(cores, modul, residue, start, stop, factor1, factor2, detail != "0", iterate != 0);
    }
    dotenv::dotenv().expect("Failed to read .env file");
    match env::var("WILFPORTs") {
        Ok(port) => {
            Iron::new(index).http(port).unwrap();
        }
        Err(_) => ()//println!("Couldn't read WILFPORT ({})", e),
    };
}

fn index(request: &mut Request) -> IronResult<Response> {
    let map = request.get_ref::<Params>().unwrap();
    let inputnumbers = match map.find(&["numbers"]) {
        Some(&Value::String(ref numbers)) => numbers,
        _ => "6 9 20",
    };
    let inputsamples = match map.find(&["samples"]) {
        Some(&Value::String(ref samples)) => samples,
        _ => "1",
    };
    let result = from_string_input(inputnumbers, inputsamples);
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let mut page: String = String::new();
    page.push_str(r##"<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="A simple page to work with Wilf's conjecture.">
    <title>Wilf</title>
    <link rel="icon" href="data:,">
    <link rel="stylesheet" href="https://unpkg.com/purecss@1.0.0/build/pure-min.css" integrity="sha384-nn4HPE8lTHyVtfCBi5yW9d20FjT8BJwUXyWZT9InLYax14RDjBj46LmSztkmNP9w" crossorigin="anonymous">
    <link rel="stylesheet" href="https://unpkg.com/purecss@1.0.0/build/grids-responsive-old-ie-min.css">
    <link rel="stylesheet" href="https://unpkg.com/purecss@1.0.0/build/grids-responsive-min.css">
    <link rel="stylesheet" href="https://netdna.bootstrapcdn.com/font-awesome/4.0.3/css/font-awesome.css">
    <style>
* {
    -webkit-box-sizing: border-box;
    -moz-box-sizing: border-box;
    box-sizing: border-box;
}

/*
 * -- BASE STYLES --
 * Most of these are inherited from Base, but I want to change a few.
 */
body {
    line-height: 1.7em;
    color: #7f8c8d;
    font-size: 13px;
}

h1,
h2,
h3,
h4,
h5,
h6,
label {
    color: #34495e;
}

.pure-img-responsive {
    max-width: 100%;
    height: auto;
}

/*
 * -- LAYOUT STYLES --
 * These are some useful classes which I will need
 */
.l-box {
    padding: 1em;
}

.l-box-lrg {
    padding: 2em;
    border-bottom: 1px solid rgba(0,0,0,0.1);
}

.is-center {
    text-align: center;
}



/*
 * -- PURE FORM STYLES --
 * Style the form inputs and labels
 */
.pure-form label {
    margin: 1em 0 0;
    font-weight: bold;
    font-size: 100%;
}

.pure-form input[type] {
    border: 2px solid #ddd;
    box-shadow: none;
    font-size: 100%;
    width: 100%;
    margin-bottom: 1em;
}

/*
 * -- PURE BUTTON STYLES --
 * I want my pure-button elements to look a little different
 */
.pure-button {
    background-color: #1f8dd6;
    color: white;
    padding: 0.5em 2em;
    border-radius: 5px;
}

a.pure-button-primary {
    background: white;
    color: #1f8dd6;
    border-radius: 5px;
    font-size: 120%;
}


/*
 * -- MENU STYLES --
 * I want to customize how my .pure-menu looks at the top of the page
 */

.home-menu {
    padding: 0.5em;
    text-align: center;
    box-shadow: 0 1px 1px rgba(0,0,0, 0.10);
}
.home-menu {
    background: #2d3e50;
}
.pure-menu.pure-menu-fixed {
    /* Fixed menus normally have a border at the bottom. */
    border-bottom: none;
    /* I need a higher z-index here because of the scroll-over effect. */
    z-index: 4;
}

.home-menu .pure-menu-heading {
    color: white;
    font-weight: 400;
    font-size: 120%;
}

.home-menu .pure-menu-selected a {
    color: white;
}

.home-menu a {
    color: #6FBEF3;
}
.home-menu li a:hover,
.home-menu li a:focus {
    background: none;
    border: none;
    color: #AECFE5;
}


/*
 * -- SPLASH STYLES --
 * This is the blue top section that appears on the page.
 */

.splash-container {
    background: #1f8dd6;
    z-index: 1;
    overflow: hidden;
    /* The following styles are required for the "scroll-over" effect */
    width: 100%;
    height: 88%;
    top: 0;
    left: 0;
    position: fixed !important;
}

.splash {
    /* absolute center .splash within .splash-container */
    width: 80%;
    height: 50%;
    margin: auto;
    position: absolute;
    top: 100px; left: 0; bottom: 0; right: 0;
    text-align: center;
    text-transform: uppercase;
}

/* This is the main heading that appears on the blue section */
.splash-head {
    font-size: 20px;
    font-weight: bold;
    color: white;
    border: 3px solid white;
    padding: 1em 1.6em;
    font-weight: 100;
    border-radius: 5px;
    line-height: 1em;
}

/* This is the subheading that appears on the blue section */
.splash-subhead {
    color: white;
    letter-spacing: 0.05em;
    opacity: 0.8;
}

/*
 * -- CONTENT STYLES --
 * This represents the content area (everything below the blue section)
 */
.content-wrapper {
    /* These styles are required for the "scroll-over" effect */
    position: absolute;
    top: 10%;
    width: 100%;
    min-height: 12%;
    z-index: 2;
    background: white;

}

/* We want to give the content area some more padding */
.content {
    padding: 1em 1em 3em;
}

/* This is the class used for the main content headers (<h2>) */
.content-head {
    font-weight: 400;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin: 2em 0 1em;
}

/* This is a modifier class used when the content-head is inside a ribbon */
.content-head-ribbon {
    color: white;
}

/* This is the class used for the content sub-headers (<h3>) */
.content-subhead {
    color: #1f8dd6;
}
    .content-subhead i {
        margin-right: 7px;
    }

/* This is the class used for the dark-background areas. */
.ribbon {
    background: #2d3e50;
    color: #aaa;
}

/* This is the class used for the footer */
.footer {
    background: #111;
    position: fixed;
    bottom: 0;
    width: 100%;
}

.lambda {
    padding : 1px;
    margin : 1px;
    border : thin 1px black;
    text-align : right;
}


.wilf {
    padding : 0;
    margin : 0;
    border : thin 1px grey;
}

.wilf-full {
    padding :0;
    margin :0;
    border :1;
    background:#C00;
    width:10px;
    height:10px;
}

.wilf-apery {
    padding :0;
    margin :0;
    border :1;
    background:#0C0;
    width:10px;
    height:10px;
}

.wilf-generator {
    padding :0;
    margin :0;
    border :1;
    background:#00C;
    width:10px;
    height:10px;
}

.wilf-empty {
    padding :0;
    margin :0;
    border :1;
    background:#eee;
    width:10px;
    height:10px;
}



/*
 * -- TABLET (AND UP) MEDIA QUERIES --
 * On tablets and other medium-sized devices, we want to customize some
 * of the mobile styles.
 */
@media (min-width: 48em) {

    /* We increase the body font size */
    body {
        font-size: 16px;
    }

    /* We can align the menu header to the left, but float the
    menu items to the right. */
    .home-menu {
        text-align: left;
    }
        .home-menu ul {
            float: right;
        }

    /* We increase the height of the splash-container */
/*    .splash-container {
        height: 500px;
    }*/

    /* We decrease the width of the .splash, since we have more width
    to work with */
    .splash {
        width: 50%;
        height: 50%;
    }

    .splash-head {
        font-size: 250%;
    }

    /* We remove the border-separator assigned to .l-box-lrg */
    .l-box-lrg {
        border: none;
    }
}

</style>
</head>
<body>
<div class="header">
    <div class="home-menu pure-menu pure-menu-horizontal pure-menu-fixed">
        <a class="pure-menu-heading" href="">WILF - COMPUTE NUMERICAL SEMIGROUPS</a>
    </div>
</div>
<div class="content-wrapper">
    <div class="content">
        <div class="pure-g">
            <div class="l-box-lrg pure-u-1 pure-u-md-5-5">
                <form method="post" action="/" class="pure-form">
                        <label for="numbers">Numbers</label>
                        <textarea id="numbers" name="numbers" rows="5" cols="50">"##);
    page.push_str(inputnumbers.trim());
    page.push_str(r##"
                        </textarea><br/>
                        <label for="samples">No of attempts to get smaller ratio:</label>
                        <textarea id="samples" name="samples" rows="1" cols="8">"##);
    page.push_str(inputsamples.trim());
    page.push_str(r##"</textarea>
                        <button type="submit" class="pure-button">Compute</button>
                </form>
            </div>
        </div>
        <div class="pure-g">
"##);
    page.push_str(&result);
    page.push_str(r##"
        </div>

    </div>
</div>
<script>
//
</script>
</body>
</html>
"##);
    response.set_mut(page);
    Ok(response)
}
