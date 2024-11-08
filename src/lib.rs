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

#[napi(object)]
pub struct MarkovChainOptions {
  pub initial_capacity: Option<u32>,
}

#[napi]
impl MarkovChain {
  #[napi(constructor)]
  pub fn new(options: Option<MarkovChainOptions>) -> Self {
    let capacity = options
      .and_then(|opts| opts.initial_capacity)
      .unwrap_or(1000) as usize;

    MarkovChain {
      chain: HashMap::with_capacity(capacity),
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
  pub fn generate(&self, start_word: Option<String>, length: u32) -> String {
    if self.chain.is_empty() || length == 0 {
      return String::new();
    }

    let mut rng = rand::thread_rng();
    let mut result = Vec::with_capacity(length as usize);

    // Get start word - either provided or random from words that have transitions
    let first_word = match start_word {
      Some(word) => {
        if self.chain.contains_key(&word as &str) {
          Arc::from(word)
        } else {
          // Only select from words that have transitions
          let valid_keys: Vec<_> = self
            .chain
            .keys()
            .filter(|k| !self.chain[*k].is_empty())
            .collect();
          if valid_keys.is_empty() {
            return String::new(); // No valid transitions available
          }
          Arc::clone(valid_keys[rng.gen_range(0..valid_keys.len())])
        }
      }
      None => {
        let valid_keys: Vec<_> = self
          .chain
          .keys()
          .filter(|k| !self.chain[*k].is_empty())
          .collect();
        if valid_keys.is_empty() {
          return String::new(); // No valid transitions available
        }
        Arc::clone(valid_keys[rng.gen_range(0..valid_keys.len())])
      }
    };

    result.push(first_word.to_string());
    let mut current_word = first_word;

    // Generate subsequent words
    while result.len() < length as usize {
      match self.chain.get(&current_word as &str) {
        Some(next_words) if !next_words.is_empty() => {
          let next_word = Arc::clone(&next_words[rng.gen_range(0..next_words.len())]);
          result.push(next_word.to_string());
          current_word = next_word;
        }
        _ => {
          // If we hit a dead end, pick a random word that has transitions
          let valid_keys: Vec<_> = self
            .chain
            .keys()
            .filter(|k| !self.chain[*k].is_empty())
            .collect();
          if valid_keys.is_empty() {
            break; // No more valid transitions possible
          }
          current_word = Arc::clone(valid_keys[rng.gen_range(0..valid_keys.len())]);
          result.push(current_word.to_string());
        }
      }
    }

    result.join(" ")
  }

  #[napi]
  pub fn get_word_count(&self) -> u32 {
    self.word_count as u32
  }

  #[napi]
  pub fn export_corpus(&self) -> String {
    let mut entries: Vec<(String, Vec<String>)> = self
      .chain
      .iter()
      .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
      .collect();

    // Sort for consistent output
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    serde_json::to_string(&entries).unwrap_or_default()
  }

  #[napi]
  pub fn import_corpus(&mut self, json: String) {
    if let Ok(entries) = serde_json::from_str::<Vec<(String, Vec<String>)>>(&json) {
      self.chain.clear();

      // Count unique words across all entries
      let mut unique_words = std::collections::HashSet::new();

      for (key, values) in entries {
        unique_words.insert(key.clone());
        unique_words.extend(values.iter().cloned());

        let key = Arc::from(key.as_str());
        let values: Vec<Arc<str>> = values.iter().map(|s| Arc::from(s.as_str())).collect();

        self.chain.insert(key, values);
      }

      self.word_count = unique_words.len();
    }
  }
}
