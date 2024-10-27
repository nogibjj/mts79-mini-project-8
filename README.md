# Python vs. Rust
[![Python CICD](https://github.com/nogibjj/mts79-mini-project-8/actions/workflows/ci_python.yml/badge.svg)](https://github.com/nogibjj/mts79-mini-project-8/actions/workflows/ci_python.yml)
[![Rust CICD](https://github.com/nogibjj/mts79-mini-project-8/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/mts79-mini-project-8/actions/workflows/CI.yml)

## Project Overview

This project demonstrates the performance benefits of rewriting an existing Python script in Rust. The goal is to showcase improvements in both execution speed and memory efficiency when using Rust, which is known for its fine-grained control over system resources. The project includes the original Python script, a Rust rewrite, and a performance comparison report.

## Getting Started

### Prerequisites

- **Python 3.x**: Install from [Python's official website](https://www.python.org/downloads/).
- **Rust**: Install via [rustup](https://rustup.rs/).

### Running the Scripts

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/nogibjj/mts79-mini-project-8
   cd mts79-mini-project-8
   cargo run -- -h
   export PATH=$PATH:/workspaces/mts79-mini-project-8/sqlite/target/release #in codespaces
   sqlite -h

# Performance Report: Python VS. Rust

## Rust Performance

| Metric             | Value            |
|--------------------|------------------|
| **Execution Time** | 1.70 ms          |
| **Memory Usage Before** | 17,029,037 KB |
| **Memory Usage After**  | 17,029,037 KB |
| **Memory Consumed**     | 0 KB          |

## Python Performance

| Metric             | Value            |
|--------------------|------------------|
| **Execution Time** | 0.30 ms          |
| **Memory Usage Before** | 33,984 KB     |
| **Memory Usage After**  | 34,000 KB     |
| **Memory Consumed**     | 16 KB         |

## Comparison Summary

| Language | Execution Time | Memory Usage Before | Memory Usage After | Memory Consumed |
|----------|----------------|---------------------|--------------------|-----------------|
| **Rust** | 1.70 ms        | 17,029,037 KB      | 17,029,037 KB      | 0 KB            |
| **Python** | 0.30 ms      | 33,984 KB          | 34,000 KB          | 16 KB           |

## Improvements in Speed and Resource Usage

- **Speed**: Python was faster for this simple task, with an execution time of 0.30 ms versus Rust's 1.70 ms. Python’s high-level optimizations can sometimes give it an edge in simpler operations.
- **Memory Efficiency**: Rust showed zero memory growth, keeping memory usage constant, while Python consumed an additional 16 KB. Rust’s strict memory management makes it highly efficient, especially for larger applications.

## Best Use Cases

- **Rust**: Ideal for resource-intensive tasks where memory predictability and efficiency are crucial.
- **Python**: Great for rapid prototyping and tasks where speed in development outweighs strict resource control.

## Conclusion

Rust offers advantages in resource management and predictable memory usage, which can lead to better performance in high-demand applications. Python, however, remains a strong choice for rapid development and tasks where execution speed is prioritized over strict memory management.
