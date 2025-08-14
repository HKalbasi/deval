# JSON Schema Test Runner

A test runner for evaluating the deval-schema-from-json-schema converter and validator against the official JSON Schema Test Suite.

## Features

- Run comprehensive analysis on JSON Schema test suite files
- Debug specific test cases with detailed output
- Uses temporary directories for isolated testing
- Builds deval-cli binary once for efficient testing
- Hides CLI output during analysis for cleaner results
- Reuses test execution logic between analysis and debug modes
- Shows clear pass/fail results in debug output
- Displays stdout and stderr output in debug mode

## Usage

### Analyze Mode

Run analysis on JSON Schema test files:

```bash
# Run analysis on default files (type.json, properties.json, required.json)
cargo run -- analyze

# Run analysis with verbose output
cargo run -- analyze -v

# Run analysis on specific files
cargo run -- analyze -f type.json,properties.json

# Run analysis with verbose output on specific files
cargo run -- analyze -v -f type.json,properties.json
```

### Debug Mode

Debug specific test cases:

```bash
# Debug a specific test case and test
cargo run -- debug -f type.json -c 0 -t 1

# Debug with different file
cargo run -- debug -f properties.json -c 1 -t 2
```

## Options

- `-f, --files <FILES>`: Comma-separated list of test files to analyze
- `-v, --verbose`: Show detailed output for each test
- `-f, --file <FILE>`: Test file to debug [default: type.json]
- `-c, --case <CASE>`: Test case index [default: 0]
- `-t, --test <TEST>`: Test index within the test case [default: 0]

## Test Infrastructure

The test runner:

1. Builds the deval-cli binary once at startup
2. Creates temporary directories for isolated testing
3. Runs JSON Schema tests from the official test suite
4. Measures coverage percentage for each test file
5. Cleans up temporary files after execution
6. Reuses test execution logic between modes for consistency

## Coverage Results

The test runner reports coverage as:

- Percentage of passing tests
- Number of passing tests vs total tests
- Detailed failure information when run in verbose mode

## Debug Output

The debug mode shows:

- Test case and test information
- Schema and data being tested
- Expected vs actual validation results
- Clear PASS/FAIL indication
- Stdout output from the validation process
- Stderr output from the validation process (including error messages)
