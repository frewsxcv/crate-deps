use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

extern crate git2;
extern crate glob;
extern crate rustc_serialize;
extern crate tiny_http;


static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";
static INDEX_LOCAL_PATH: &'static str = "crates.io-index";


#[derive(RustcDecodable)]
#[allow(dead_code)]
struct CrateInfo {
    name: String,
    vers: String,
    deps: Vec<DepInfo>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct DepInfo {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>
}

fn build_dependency_map() -> HashMap<String, Vec<String>> {
    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_leading_dot = true;

    let index_paths1 = glob::glob_with("crates.io-index/*/*/*", &match_options).unwrap();
    let index_paths2 = glob::glob_with("crates.io-index/[12]/*", &match_options).unwrap();

    let index_paths = index_paths1.chain(index_paths2);

    let mut map = HashMap::new();

    for glob_result in index_paths {
        let index_path = glob_result.unwrap();
        let file = fs::File::open(&index_path).unwrap();
        let last_line = BufReader::new(file).lines().last().unwrap().unwrap();
        let crate_info: CrateInfo = rustc_serialize::json::decode(&last_line).unwrap();
        let deps_names = crate_info.deps.iter().map(|d| d.name.clone()).collect();
        map.insert(crate_info.name, deps_names);
    }

    map
}

fn build_dot(crate_name: &str, dep_map: &HashMap<String, Vec<String>>) -> String {
    let mut crate_names = vec![crate_name];

    let mut dot = String::new();
    dot.push_str("digraph graphname {");

    while let Some(crate_name) = crate_names.pop() {
        for crate_dep in dep_map.get(crate_name).unwrap() {
            dot.push_str(&format!("{} -> {};", crate_name.replace("-", "_"), crate_dep.replace("-", "_")))
        }
    }
    dot.push_str("}");

    dot
}

fn main() {
    if fs::metadata(INDEX_LOCAL_PATH).is_err() {
        println!("Cloning crates.io-index");
        git2::Repository::clone(INDEX_GIT_URL, INDEX_LOCAL_PATH).unwrap();
    }

    let dep_map = build_dependency_map();

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let server = tiny_http::ServerBuilder::new().with_port(port).build().unwrap();

    println!("Server listening on port {}", port);
    for req in server.incoming_requests() {
        let response = if dep_map.get(req.get_url().trim_left_matches("/")).is_some() {
            build_dot(req.get_url().trim_left_matches("/"), &dep_map)
        } else {
            String::from("could not find crate")
        };

        let response = tiny_http::Response::from_string(response);
        req.respond(response);
    }
}
