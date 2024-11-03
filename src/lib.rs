#![deny(clippy::all)]

use napi_derive::napi;
use rand::Rng;
use std::collections::HashMap;

#[napi(js_name = "MarkovChain")]
pub struct MarkovChain {
  chain: HashMap<String, Vec<String>>,
}

#[napi]
impl MarkovChain {
  #[napi(constructor)]
  pub fn new() -> Self {
    MarkovChain {
      chain: HashMap::new(),
    }
  }

  #[napi]
  pub fn add_text(&mut self, text: String) {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

    for i in 0..words.len().saturating_sub(1) {
      let current = &words[i];
      let next = &words[i + 1];

      self
        .chain
        .entry(current.clone())
        .or_insert_with(Vec::new)
        .push(next.clone());
    }
  }

  #[napi]
  pub fn generate(&self, start_word: Option<String>, length: i32) -> String {
    if self.chain.is_empty() {
      return String::new();
    }

    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    let mut current = start_word
      .and_then(|word| {
        if self.chain.contains_key(&word) {
          Some(word)
        } else {
          self.chain.keys().next().map(|s| s.to_string())
        }
      })
      .unwrap_or_else(|| {
        self
          .chain
          .keys()
          .nth(rng.gen_range(0..self.chain.len()))
          .unwrap()
          .clone()
      });

    result.push(current.clone());

    for _ in 0..length.saturating_sub(1) {
      if let Some(next_words) = self.chain.get(&current) {
        if next_words.is_empty() {
          break;
        }
        current = next_words[rng.gen_range(0..next_words.len())].clone();
        result.push(current.clone());
      } else {
        break;
      }
    }

    result.join(" ")
  }
}
