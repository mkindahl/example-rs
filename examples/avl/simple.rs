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

extern crate data_structures;

use data_structures::avl::Tree;

fn main() {
    let mut tree = Tree::new();
    for i in 1..11 {
        tree.insert(i, i * i);
        println!("After inserting {}", i);
        tree.pretty();
        tree.insert(2 * 11 - i, i * i * i);
        println!("After inserting {}", 2 * 11 - i);
        tree.pretty();
    }

    for i in 1..11 {
        println!("Searching for {} gave {:?}", i, tree.find(i));
    }
}
