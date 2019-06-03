// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you
// may not use this file except in compliance with the License.  You
// may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.  See the License for the specific language governing
// permissions and limitations under the License.

extern crate examples;

use examples::expr::eval;
use std::collections::HashMap;
use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: expr <expression> [ <variable>=<value> ... ]");
    } else {
        let expr = &args[1];
        let mut map = HashMap::new();
        for assign in &args[2..] {
            let parts: Vec<&str> = assign.splitn(2, '=').collect();
            map.insert(
                parts[0].to_string(),
                parts[1].parse::<f64>().expect("expected number"),
            );
        }
        println!("'{}' => {:?}", expr, eval(expr.as_str(), &map));
    }
}
