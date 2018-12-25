extern crate iron;
extern crate params;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use params::{Params, Value};

fn main() {
    println!("Starting server at 8800...");
    Iron::new(index).http("localhost:8800").unwrap();
}

fn index(request:&mut Request) -> IronResult<Response>{
    println!("request..");
    let map = request.get_ref::<Params>().unwrap();
    let inputnumbers = match map.find(&["numbers"]) {
        Some(&Value::String(ref numbers)) => numbers,
        _ => "6 9 20",
    };
    let inputsamples = match map.find(&["samples"]) {
        Some(&Value::String(ref samples)) => samples,
        _ => "1",
    };

    println!("numbers: {} samples: {}",inputnumbers,inputsamples);

    let result = &rust_iron::from_string_input(inputnumbers, inputsamples);

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let mut page:String = String::new();
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
}

.wilf-apery {
    padding :0;
    margin :0;
    border :1;
    background:#0C0;
}

.wilf-empty {
    padding :0;
    margin :0;
    border :1;
    background:#eee;
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
        <a class="pure-menu-heading" href="">WILF</a>
        <ul class="pure-menu-list">
            <li class="pure-menu-item pure-menu-selected"><a href="#" class="pure-menu-link">Link</a></li>
            <li class="pure-menu-item"><a href="#" class="pure-menu-link">Link</a></li>
            <li class="pure-menu-item"><a href="#" class="pure-menu-link">Link</a></li>
        </ul>
    </div>
</div>
<div class="content-wrapper">
    <div class="content">
        <h2 class="content-head is-center">Compute numerical Semigroups</h2>
        <div class="pure-g">
            <div class="l-box-lrg pure-u-1 pure-u-md-2-5">
                <form method="post" action="/" class="pure-form pure-form-stacked">
                    <fieldset>
                        <label for="numbers">Numbers</label>
                        <input id="numbers" name="numbers" type="text" value=""##);
page.push_str(inputnumbers);
page.push_str(r##"">
                        <label for="samples">Samples</label>
                        <input id="samples" name="samples" type="text" value=""##);
page.push_str(inputsamples);
page.push_str(r##"">
                        <button type="submit" class="pure-button">Compute</button>
                    </fieldset>
                </form>
            </div>
            <div class="l-box-lrg pure-u-1 pure-u-md-3-5">
"##);
page.push_str(result);
page.push_str(r##"
            </div>
        </div>

    </div>
    <div class="footer l-box is-center">
         No disclaimer, nothing to do here.
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