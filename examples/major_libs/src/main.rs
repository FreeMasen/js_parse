//! This example is primarily for illustrating the
//! project's abilities. It is a good example of how
//! the `tokenize` function works.
extern crate ress;
extern crate reqwest;
use std::time::{SystemTime, Duration};

fn main() {
    println!("trying jquery");
    jquery();
    println!("trying angular1");
    angular1();
    println!("trying react");
    react();
    println!("trying react_dom");
    react_dom();
    println!("trying vue");
    vue();
}

fn jquery() {
    match get_js("https://code.jquery.com/jquery-3.3.1.js") {
        Ok(ref js) => {
            let size = js.len();
            let now = SystemTime::now();
            let _ = ress::tokenize(js);
            if let Ok(elapsed) = now.elapsed() {
                report(size, elapsed);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}

fn angular1() {
    match get_js("https://ajax.googleapis.com/ajax/libs/angularjs/1.5.6/angular.js") {
        Ok(ref js) => {
            let size = js.len();
            let now = SystemTime::now();
            let _ = ress::tokenize(js);
            if let Ok(elapsed) = now.elapsed() {
                report(size, elapsed);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}

fn react() {
    match get_js("https://unpkg.com/react@16/umd/react.development.js") {
        Ok(ref js) => {
            let size = js.len();
            let now = SystemTime::now();
            let _ = ress::tokenize(js);
            if let Ok(elapsed) = now.elapsed() {
                report(size, elapsed);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}

fn react_dom() {
    match get_js("https://unpkg.com/react-dom@16/umd/react-dom.development.js") {
        Ok(ref js) => {
            let size = js.len();
            let now = SystemTime::now();
            let _ = ress::tokenize(js);
            if let Ok(elapsed) = now.elapsed() {
                report(size, elapsed);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}

fn vue() {
    match get_js("https://cdn.jsdelivr.net/npm/vue@2.5.16/dist/vue.js") {
        Ok(ref js) => {
            let size = js.len();
            let now = SystemTime::now();
            let _ = ress::tokenize(js);
            if let Ok(elapsed) = now.elapsed() {
                report(size, elapsed);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}

fn get_js(url: &str) -> Result<String, String> {
    let c = reqwest::Client::new();
    match c.get(url.clone()).send() {
        Ok(mut res) => match res.text() {
            Ok(js) => Ok(js.to_string()),
            Err(e) => Err(format!("Error getting js: {:?}", e))
        },
        Err(e) => Err(format!("Error getting js: {:?}", e)),
    }
}

fn report(bytes: usize, elapsed: Duration) {
    let size = get_size(bytes);
    println!("tokenized {} in {}s {:.2}ms", size, elapsed.as_secs(), elapsed.subsec_millis())
}

fn get_size(b: usize) -> String {
    let mut size = b as f32;
    let mut i = 0;
    while size > 1000 as f32 {
        if i > 4 {
            break;
        }
        size = size / 1000.0;
        i += 1;
    }
    let bytes = match i {
        0 => "b",
        1 => "kb",
        2 => "mb",
        3 => "gb",
        _ => "tb",
    };
    format!("{:.2}{}", size, bytes)
}