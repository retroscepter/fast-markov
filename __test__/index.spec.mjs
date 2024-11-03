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
