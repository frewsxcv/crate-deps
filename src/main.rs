// Copyright 2015 Corey Farwell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/* TODO:
 *  * redirect root url
 *  * add watermark? url to repo?
 *  * update git repo?
 *  * serverside caching? redis?
 */

use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{Write, Read};
use std::process::{Command, Stdio};

use tiny_http::{Header, Response};

extern crate crates_index;
extern crate tiny_http;

static INDEX_LOCAL_PATH: &'static str = "crates.io-index";


fn build_dot(crate_name: &str, dep_map: &HashMap<String, Vec<String>>) -> Vec<u8> {
    let mut crate_names = vec![crate_name];

    let mut dot = String::new();
    dot.push_str("digraph graphname {");
    dot.push_str("ratio=0.75;");

    dot.push_str(&format!("\"{}\" [root=true]", crate_name));

    // Which dependencies we've already seen
    let mut seen_set = HashSet::new();

    while let Some(crate_name) = crate_names.pop() {
        if seen_set.contains(crate_name as &str) {
            continue;
        }
        seen_set.insert(crate_name);
        for crate_dep in dep_map.get(crate_name).unwrap() {
            dot.push_str(&format!("\"{}\" -> \"{}\";", crate_name, crate_dep));
            if !seen_set.contains(crate_dep as &str) {
                crate_names.push(crate_dep);
            }
        }
    }
    dot.push_str("}");

    let child = Command::new("dot").arg("-Tpng").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    child.stdin.unwrap().write_all(dot.as_bytes()).unwrap();

    let mut ret = vec![];
    child.stdout.unwrap().read_to_end(&mut ret).unwrap();
    ret
}

fn main() {
    let index = crates_index::CratesIndex::new(INDEX_LOCAL_PATH.into());
    if !index.exists() {
        println!("Cloning crates.io-index");
        index.clone_index();
    }

    let dep_map = index.dependency_map();

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let server = tiny_http::ServerBuilder::new().with_port(port).build().unwrap();

    println!("Server listening on port {}", port);
    for req in server.incoming_requests() {
        let response = {
            let crate_name = req.url().trim_left_matches("/");
            if dep_map.get(crate_name).is_some() {
                let data = build_dot(crate_name, &dep_map);
                let content_type_header = "Content-Type: image/png".parse::<Header>().unwrap();
                let cache_control_header = "cache-control: no-cache".parse::<Header>().unwrap();
                Response::from_data(data).with_header(content_type_header)
                                         .with_header(cache_control_header)
            } else {
                Response::from_string("could not find crate").with_status_code(400)
            }
        };
        req.respond(response);
    }
}
