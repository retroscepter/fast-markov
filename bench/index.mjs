import Benchmark from "benchmark";
import { MarkovChain } from "../index.js";

const suite = new Benchmark.Suite();

// Sample texts of different sizes
const smallText = "the quick brown fox jumps over the lazy dog";
const mediumText = Array(100).fill(smallText).join(" ");
const largeText = Array(1000).fill(smallText).join(" ");

// Initialize and train chains of different sizes
const smallChain = new MarkovChain({ initialCapacity: 100 });
smallChain.addText(smallText); // ~9 words

const mediumChain = new MarkovChain({ initialCapacity: 1000 });
mediumChain.addText(mediumText); // ~900 words

const largeChain = new MarkovChain({ initialCapacity: 10000 });
largeChain.addText(largeText); // ~9000 words

// Pre-export corpus strings for import benchmarks
const smallCorpus = smallChain.exportCorpus();
const mediumCorpus = mediumChain.exportCorpus();
const largeCorpus = largeChain.exportCorpus();

// Training benchmarks
suite
  .add("addText (small - 9 words)", () => {
    const chain = new MarkovChain({ initialCapacity: 100 });
    chain.addText(smallText);
  })
  .add("addText (medium - 900 words)", () => {
    const chain = new MarkovChain({ initialCapacity: 1000 });
    chain.addText(mediumText);
  })
  .add("addText (large - 9000 words)", () => {
    const chain = new MarkovChain({ initialCapacity: 10000 });
    chain.addText(largeText);
  })
  // Generation benchmarks - small chain
  .add("generate 10 words (from 9 word chain)", () => {
    smallChain.generate(null, 10);
  })
  .add("generate 50 words (from 9 word chain)", () => {
    smallChain.generate(null, 50);
  })
  .add("generate 100 words (from 9 word chain)", () => {
    smallChain.generate(null, 100);
  })
  // Generation benchmarks - medium chain
  .add("generate 10 words (from 900 word chain)", () => {
    mediumChain.generate(null, 10);
  })
  .add("generate 50 words (from 900 word chain)", () => {
    mediumChain.generate(null, 50);
  })
  .add("generate 100 words (from 900 word chain)", () => {
    mediumChain.generate(null, 100);
  })
  // Generation benchmarks - large chain
  .add("generate 10 words (from 9000 word chain)", () => {
    largeChain.generate(null, 10);
  })
  .add("generate 50 words (from 9000 word chain)", () => {
    largeChain.generate(null, 50);
  })
  .add("generate 100 words (from 9000 word chain)", () => {
    largeChain.generate(null, 100);
  })
  // Export benchmarks
  .add("exportCorpus (9 word chain)", () => {
    smallChain.exportCorpus();
  })
  .add("exportCorpus (900 word chain)", () => {
    mediumChain.exportCorpus();
  })
  .add("exportCorpus (9000 word chain)", () => {
    largeChain.exportCorpus();
  })
  // Import benchmarks with pre-exported corpus
  .add("importCorpus (9 word chain)", () => {
    const chain = new MarkovChain({ initialCapacity: 100 });
    chain.importCorpus(smallCorpus);
  })
  .add("importCorpus (900 word chain)", () => {
    const chain = new MarkovChain({ initialCapacity: 1000 });
    chain.importCorpus(mediumCorpus);
  })
  .add("importCorpus (9000 word chain)", () => {
    const chain = new MarkovChain({ initialCapacity: 10000 });
    chain.importCorpus(largeCorpus);
  })
  .on("cycle", function (event) {
    console.log(String(event.target));
  })
  .on("complete", function () {
    console.log("done");
  })
  .run({ async: true });
