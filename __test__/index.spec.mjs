import test from "ava";

import { MarkovChain } from "../index.js";

test("new MarkovChain() creates an empty chain", (t) => {
  const markov = new MarkovChain();
  t.is(markov.generate(null, 10), "");
});

test("addText() with single word creates empty chain", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello");
  t.is(markov.generate(null, 1), "");
});

test("addText() with two words creates expected chain", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello world");
  t.is(markov.generate("hello", 2), "hello world");
});

test("generate() respects length parameter", (t) => {
  const markov = new MarkovChain();
  markov.addText("the quick brown fox jumps over the lazy dog");
  const result = markov.generate(null, 3);
  t.is(result.split(" ").length, 3);
});

test("generate() with invalid start word falls back to random start", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello world");
  const result = markov.generate("invalid", 2);
  t.truthy(result.length > 0);
});

test("generate() with multiple possible next words returns valid chain", (t) => {
  const markov = new MarkovChain();
  markov.addText("the dog runs");
  markov.addText("the cat sleeps");
  markov.addText("the bird flies");
  const result = markov.generate("the", 2);
  t.truthy(
    ["the dog", "the cat", "the bird"].includes(result),
    "Should generate one of the valid chains"
  );
});

test("addText() handles empty string", (t) => {
  const markov = new MarkovChain();
  markov.addText("");
  t.is(markov.generate(null, 1), "");
});

test("addText() handles multiple spaces", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello    world");
  t.is(markov.generate("hello", 2), "hello world");
});

test("exportCorpus() returns empty JSON array for empty chain", (t) => {
  const markov = new MarkovChain();
  t.is(markov.exportCorpus(), "[]");
});

test("exportCorpus() returns correct JSON for simple chain", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello world");
  const exported = JSON.parse(markov.exportCorpus());
  t.deepEqual(exported, [["hello", ["world"]]]);
});

test("importCorpus() with empty JSON array creates empty chain", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello world");
  markov.importCorpus("[]");
  t.is(markov.getWordCount(), 0);
  t.is(markov.generate(null, 1), "");
});

test("importCorpus() correctly restores chain state", (t) => {
  const markov1 = new MarkovChain();
  markov1.addText("the quick brown fox");
  const exported = markov1.exportCorpus();

  const markov2 = new MarkovChain();
  markov2.importCorpus(exported);

  t.is(markov1.getWordCount(), markov2.getWordCount());
  t.is(markov2.generate("the", 4), "the quick brown fox");
});

test("importCorpus() handles invalid JSON gracefully", (t) => {
  const markov = new MarkovChain();
  markov.addText("hello world");
  markov.importCorpus("invalid json");
  t.is(markov.getWordCount(), 2);
  t.is(markov.generate("hello", 2), "hello world");
});

test("MarkovChain constructor accepts options with initialCapacity", (t) => {
  const markov = new MarkovChain({ initialCapacity: 5000 });
  t.truthy(markov);
});

test("MarkovChain constructor works without options", (t) => {
  const markov = new MarkovChain();
  t.truthy(markov);
});
