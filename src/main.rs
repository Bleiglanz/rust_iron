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


fn computation(primes: &[usize], task: (usize, usize), factor:usize, detail: bool) {
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

    let mut out = std::fs::File::create(format!("./outinterv{}_{}_{}.csv", factor, start, stop)).expect("Unable to create file");

    for skip in start..stop {
        let maxindex :usize = findmaxindex(&primes, skip, factor);
        let res2: Fast = fast(&primes[skip..maxindex]);
        let ausgabe = format!("{:6};Fast <{}p;f;{:6};m;{:6};e;{:6};S<f;{:8};f/p;{:.6};wilf;{:.6}\n",
                     skip + 1,factor,
                     res2.maxgap, res2.g1, res2.e, res2.count_set,
                     res2.maxgap as f64 / res2.g1 as f64, res2.count_set as f64 / res2.c as f64);

        if detail { print!("{}", ausgabe); }
        if detail {
            let resf: Semi = semi(&primes[skip..maxindex]);
            let fausgabe = format!("{:6};Semi<{}p;f;{:6};m;{:6};e;{:6};S<f;{:8};f/p;{:.6};wilf;{:.6}",
                                  skip + 1,factor,
                                  resf.maxgap, resf.g1, resf.e, resf.count_set,
                                  resf.maxgap as f64 / resf.g1 as f64, resf.count_set as f64 / resf.c as f64);
            println!("{}",fausgabe);

            let res: WilfSet = generatewilf(&primes[skip..maxindex]);
            println!("{:6};Wilf <{}p;f;{:6};m;{:6};e;{:6};S<f;{:8};f/p;{:.6};wilf;{:.6}\n",
                     skip + 1, factor,
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
            //let l = res.lambda_matrix;
            //for i in 0..res.g1 {
            //    print!("{:2}",l[1][i]);
            //}
            //println!("\n a1={} höhe {}",res.apery[1],(res.apery[1]-1)/res.g1);
        }
        use std::io::Write;
        out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
    }
    println!("Task beendet {}-{}", start, stop);
}


fn mainprimes(cores: usize, start: usize, stop: usize, factor:usize, detail:bool) {
    let interval = (stop - start) / cores;
    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for ti in 0..cores {
        tasks.push((start + ti * interval, start + (ti + 1) * interval))
    }

    let primesvec: Vec<usize> = primal::Primes::all().take(8000000).collect();
    let primes:&[usize] = &primesvec;
    println!("Primgruppen {:?}", tasks);
    thread::scope(|s| for task in &tasks {
        let sta = task.0;
        let sto = task.1;
        s.spawn(move |_| {
            computation(primes, (sta, sto),  factor, detail);
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
        .arg(Arg::with_name("start")
            .help("where to begin, a n th prime")
            .required(true)
            .default_value("10")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime")
            .required(true)
            .default_value("100")
        )
        .arg(Arg::with_name("factor")
            .help("take all primes as generators p_start <= gen  < factor*p_start")
            .required(true)
            .default_value("2")
        )
        .arg(Arg::with_name("detail")
            .help("if 1, show details")
            .required(true)
            .default_value("1")
        )
        .get_matches();

    let cores: usize = matches.value_of("cores").unwrap().parse().unwrap();
    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();
    let factor: usize = matches.value_of("factor").unwrap().parse().unwrap();
    let detail:&str = matches.value_of("detail").unwrap();
    if cores > 0 {
        mainprimes(cores, start, stop, factor, detail != "0");
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
