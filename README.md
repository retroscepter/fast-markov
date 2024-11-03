# fast-markov

A high-performance Markov string chain for Node.js written in Rust.

## Installation

```bash
npm install fast-markov
# or
yarn add fast-markov
# or
pnpm add fast-markov
```

## Usage

### Basic Example

```javascript
const { MarkovChain } = require("fast-markov");

// Create a new chain
const chain = new MarkovChain();

// Add some training text
chain.addText("the quick brown fox jumps over the lazy dog");
chain.addText("the lazy cat sleeps all day long");

// Generate new text (10 words)
console.log(chain.generate(null, 10));
```

### With Custom Start Word

```javascript
const { MarkovChain } = require("fast-markov");

const chain = new MarkovChain();
chain.addText("the quick brown fox jumps over the lazy dog");

// Generate text starting with 'the'
console.log(chain.generate("the", 5));
```

### Saving and Loading Chain State

```javascript
const { MarkovChain } = require("fast-markov");

// Create and train chain
const chain = new MarkovChain();
chain.addText("some training text here");

// Export chain state
const exported = chain.exportCorpus();

// Create new chain and import state
const newChain = new MarkovChain();
newChain.importCorpus(exported);
```

### With Custom Initial Capacity

```javascript
const { MarkovChain } = require("fast-markov");

// Create chain with larger initial capacity for better performance
// when training with large datasets
const chain = new MarkovChain({ initialCapacity: 10000 });
```

## API Reference

### `new MarkovChain(options?: MarkovChainOptions)`

Creates a new Markov chain instance.

- `options.initialCapacity?: number` - Initial capacity for the internal hash map (default: 1000)

### `addText(text: string): void`

Adds training text to the chain.

### `generate(startWord: string | null, length: number): string`

Generates new text based on the trained patterns.

- `startWord` - Optional word to start generation with
- `length` - Number of words to generate
- Returns generated text string

### `getWordCount(): number`

Returns the total number of words in the chain.

### `exportCorpus(): string`

Exports the chain state as a JSON string.

### `importCorpus(json: string): void`

Imports a previously exported chain state.

## Supported Platforms

Pre-built binaries are available for the following platforms:

- Windows (x64, arm64)
- macOS (x64, arm64)
- Linux (x64, arm64)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
