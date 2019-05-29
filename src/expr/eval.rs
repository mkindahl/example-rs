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

use {Error, Machine, Parser, Result};

#[derive(Debug)]
struct Evaluator {
    stack: Vec<f64>,
}

impl Machine for Evaluator {
    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }
    fn add(&mut self) -> Result<()> {
        self.binary(|a, b| a + b)
    }
    fn sub(&mut self) -> Result<()> {
        self.binary(|a, b| a - b)
    }
    fn mul(&mut self) -> Result<()> {
        self.binary(|a, b| a * b)
    }
    fn div(&mut self) -> Result<()> {
        self.binary(|a, b| a / b)
    }
    fn exp(&mut self) -> Result<()> {
        self.binary(|a, b| a.powf(b))
    }
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { stack: Vec::new() }
    }

    pub fn eval(&mut self, expr: &str) -> Result<f64> {
        let result = {
            let mut parser = Parser::new(self);
            parser.parse(expr)
        };

        match result {
            Ok(()) if self.stack.len() != 1 => Err(Error::MissingOperand),
            Ok(()) => self.stack.pop().ok_or(Error::MissingOperand),
            Err(e) => Err(e),
        }
    }

    fn pop(&mut self) -> Result<f64> {
        self.stack.pop().ok_or(Error::MissingOperand)
    }

    fn binary<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let a = self.pop()?;
        let b = self.pop()?;
        let r = f(b, a);
        self.stack.push(r);
        Ok(())
    }
}

pub fn eval(expr: &str) -> Result<f64> {
    Evaluator::new().eval(expr)
}
