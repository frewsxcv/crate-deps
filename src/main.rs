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
static ROOT_REDIRECT_URL: &'static str = "https://github.com/frewsxcv/crate-deps";


struct DotBuilder {
    buf: String,
}

impl DotBuilder {
    fn new_digraph(name: &str) -> Self {
        DotBuilder{buf: format!("digraph \"{}\" {}", name, "{")}
    }

    fn set_ratio(&mut self, ratio: &str) {
        self.buf.push_str(&format!("ratio={};", ratio))
    }

    fn set_node_attrs(&mut self, node: &str, attrs: &str) {
        self.buf.push_str(&format!("\"{}\" [{}];", node, attrs));
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.buf.push_str(&format!("\"{}\" -> \"{}\";", from, to));
    }

    fn finish(&mut self) {
        self.buf.push_str("}");
    }

    fn png_bytes(&self) -> Vec<u8> {
        let child = Command::new("dot").arg("-Tpng")
                                       .stdin(Stdio::piped()).stdout(Stdio::piped())
                                       .spawn().unwrap();
        child.stdin.unwrap().write_all(self.buf.as_bytes()).unwrap();
        let mut ret = vec![];
        child.stdout.unwrap().read_to_end(&mut ret).unwrap();
        ret
    }
}


fn build_dot_png(crate_name: &str, dep_map: &HashMap<String, Vec<String>>) -> Vec<u8> {
    let mut crate_names = vec![crate_name];

    let mut dot = DotBuilder::new_digraph(crate_name);
    dot.set_ratio("0.75");
    dot.set_node_attrs(crate_name, "root=true,style=filled,fillcolor=grey");

    // Which dependencies we've already seen
    let mut seen_set = HashSet::new();

    while let Some(crate_name) = crate_names.pop() {
        if seen_set.contains(crate_name as &str) {
            continue;
        }
        seen_set.insert(crate_name);
        for crate_dep in dep_map.get(crate_name).unwrap() {
            dot.add_edge(crate_name, crate_dep);
            if !seen_set.contains(crate_dep as &str) {
                crate_names.push(crate_dep);
            }
        }
    }
    dot.finish();

    dot.png_bytes()
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
            if crate_name.is_empty() {
                let location_string = format!("Location: {}", ROOT_REDIRECT_URL);
                let location_header = location_string.parse::<Header>().unwrap();
                // FIXME: use Response::empty() instead of Response::from_string()
                Response::from_string("").with_status_code(302)
                                         .with_header(location_header)
            } else if dep_map.get(crate_name).is_some() {
                let data = build_dot_png(crate_name, &dep_map);
                let content_type_header = "Content-Type: image/png".parse::<Header>().unwrap();
                let cache_control_header = "cache-control: no-cache".parse::<Header>().unwrap();
                Response::from_data(data).with_header(content_type_header)
                                         .with_header(cache_control_header)
            } else {
                let error = format!("could not find crate (that has dependencies) titled: '{}'", crate_name);
                Response::from_string(error).with_status_code(400)
            }
        };
        req.respond(response);
    }
}
