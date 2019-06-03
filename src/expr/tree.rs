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

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoValue(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ExprTree {
    Var(String),
    Float(f64),
    Add(Box<ExprTree>, Box<ExprTree>),
    Sub(Box<ExprTree>, Box<ExprTree>),
    Mul(Box<ExprTree>, Box<ExprTree>),
    Div(Box<ExprTree>, Box<ExprTree>),
}

impl ExprTree {
    pub fn eval(self, map: &HashMap<String, f64>) -> Result<f64> {
        match self {
            ExprTree::Float(num) => Ok(num),
            ExprTree::Var(name) => map
                .get(&name)
                .ok_or(Error::NoValue(name.clone()))
                .map(Clone::clone),
            ExprTree::Add(lhs, rhs) => Ok(lhs.eval(map)? + rhs.eval(map)?),
            ExprTree::Sub(lhs, rhs) => Ok(lhs.eval(map)? - rhs.eval(map)?),
            ExprTree::Mul(lhs, rhs) => Ok(lhs.eval(map)? * rhs.eval(map)?),
            ExprTree::Div(lhs, rhs) => Ok(lhs.eval(map)? / rhs.eval(map)?),
        }
    }
}
