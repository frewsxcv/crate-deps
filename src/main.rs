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

use std::collections::HashSet;
use std::env;

use dotty::DotBuilder;
use tiny_http::{Header, Response};

extern crate crates_index;
extern crate dotty;
extern crate tiny_http;

static INDEX_LOCAL_PATH: &'static str = "crates.io-index";
static ROOT_REDIRECT_URL: &'static str = "https://github.com/frewsxcv/crate-deps";


fn build_dot_png(crate_: crates_index::Crate, index: &crates_index::Index) -> Vec<u8> {
    let crate_name = {
        let version = crate_.latest_version();
        version.name().to_owned()
    };

    let mut crates = vec![crate_];

    let mut dot = DotBuilder::new_digraph(&crate_name);
    dot.set_ratio("0.75");
    dot.set_node_attrs(&crate_name, "root=true,style=filled,fillcolor=grey");

    // Which dependencies we've already seen
    let mut seen_set = HashSet::new();

    while let Some(crate_) = crates.pop() {
        // TODO: this shouldn't always look up the latest version for the dependencies
        let version = crate_.latest_version().to_owned();

        if seen_set.contains(version.name()) {
            continue;
        }
        seen_set.insert(version.name().to_owned());
        for dep in version.dependencies() {
            dot.add_edge(&version.name(), &dep.name());
            if !seen_set.contains(dep.name()) {
                crates.push(index.crate_(dep.name()).unwrap());
            }
        }
    }
    dot.finish();

    dot.png_bytes()
}

fn main() {
    let index = crates_index::Index::new(INDEX_LOCAL_PATH.into());
    if !index.exists() {
        println!("Cloning crates.io-index");
        index.clone().unwrap();
    }

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
            } else if let Some(crate_) = index.crate_(crate_name) {
                let data = build_dot_png(crate_, &index);
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
