<<<<<<< HEAD
=======
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

>>>>>>> 2df653b
#[derive(Debug, PartialEq)]
pub enum Error {
    Parser(parser::Error),
    Eval(tree::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod parser;
pub mod tokens;
pub mod tree;

impl std::convert::From<parser::Error> for Error {
    fn from(error: parser::Error) -> Error {
        Error::Parser(error)
    }
}

impl std::convert::From<tree::Error> for Error {
    fn from(error: tree::Error) -> Error {
        Error::Eval(error)
    }
}

pub use self::parser::parse;

use std::collections::HashMap;

pub fn eval(expr: &str, map: &HashMap<String, f64>) -> Result<f64> {
    parse(expr)?.eval(map).map_err(|err| err.into())
}
