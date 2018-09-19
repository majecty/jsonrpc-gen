extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use clap::{Arg, App};

fn main() {
    let matches = App::new("jsonrpc-gen")
        .version("0.1.0")
        .arg(Arg::with_name("method")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("method"))
        .arg(Arg::with_name("args")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("args"))
        .get_matches();
    let method = matches.value_of("method").unwrap();
    let args = matches.value_of("args").unwrap();

//    println!("{}, {}", method, args);
//    println!("{}", serde_json::to_string(&Req::new(method, args)).unwrap());
    println!("{}", Req::new(method, args).print_json());
}

#[derive(Serialize, Deserialize, Debug)]
struct Req {
    jsonrpc: String,
    id: String,
    method: String,
    params: String,
}

impl Req {
    fn new(method: &str, params: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params: params.to_string(),
        }
    }

    fn print_json(&self) -> String {
        format!("{{\"jsonrpc\": \"{}\", \"id\":\"{}\", \"method\": \"{}\", \"params\": {} }}",
            self.jsonrpc, self.id, self.method, self.params)
    }
}
