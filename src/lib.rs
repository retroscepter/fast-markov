#![deny(clippy::all)]

use napi_derive::napi;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;

type WordMap = HashMap<Arc<str>, Vec<Arc<str>>>;

#[napi(js_name = "MarkovChain")]
pub struct MarkovChain {
  chain: WordMap,
  word_count: usize,
}

#[napi]
impl MarkovChain {
  #[napi(constructor)]
  pub fn new() -> Self {
    MarkovChain {
      chain: HashMap::with_capacity(1000),
      word_count: 0,
    }
  }

  #[napi]
  pub fn add_text(&mut self, text: String) {
    if text.is_empty() {
      return;
    }

    let words: Vec<Arc<str>> = text.split_whitespace().map(|s| Arc::from(s)).collect();

    if words.len() < 2 {
      return;
    }

    self.word_count += words.len();

    let avg_transitions = 5;

    for window in words.windows(2) {
      self
        .chain
        .entry(Arc::clone(&window[0]))
        .or_insert_with(|| Vec::with_capacity(avg_transitions))
        .push(Arc::clone(&window[1]));
    }
  }

  #[napi]
  pub fn generate(&self, start_word: Option<String>, length: i32) -> String {
    if self.chain.is_empty() || length <= 0 {
      return String::new();
    }

    let mut rng = rand::thread_rng();
    let mut result = Vec::with_capacity(length as usize);

    let first_word = match start_word {
      Some(word) => {
        let word = Arc::from(word.as_str());
        if self.chain.contains_key(&word) {
          word
        } else {
          Arc::clone(self.chain.keys().next().unwrap())
        }
      }
      None => {
        let idx = rng.gen_range(0..self.chain.len());
        Arc::clone(self.chain.keys().nth(idx).unwrap())
      }
    };

    result.push(first_word.to_string());
    let mut current = first_word;

    let target_len = length.min(100) as usize;

    while result.len() < target_len {
      match self.chain.get(&current) {
        Some(next_words) if !next_words.is_empty() => {
          let next = Arc::clone(&next_words[rng.gen_range(0..next_words.len())]);
          result.push(next.to_string());
          current = next;
        }
        _ => break,
      }
    }

    result.join(" ")
  }

  #[napi]
  pub fn get_word_count(&self) -> u32 {
    self.word_count as u32
  }
}
