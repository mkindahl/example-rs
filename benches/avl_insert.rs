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

#[macro_use]
extern crate criterion;
extern crate examples;

use examples::avl;
use criterion::Criterion;

fn inserts(n: u32) {
    use avl::tree::Tree;
    let mut tree = Tree::new();
    for i in 1..n {
        tree.insert(i, i * i);
    }
}

fn bench_inserts(c: &mut Criterion) {
    c.bench_function("avl_inserts 1000", |b| b.iter(|| inserts(1000)));
}

criterion_group!(benches, bench_inserts);
criterion_main!(benches);
