# Rust Explained With Examples

Welcome to **Rust Explained With Examples**, a repository dedicated to providing practical examples of Rust code, covering a wide range of use cases. This repository is designed for both beginners and experienced Rust developers looking to deepen their understanding of the language by exploring code examples that range from the basics to more complex implementations.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Installation Steps](#installation-steps)

## Introduction

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. However, its unique ownership system and type safety can present a steep learning curve for newcomers. 

This repository aims to break down that complexity by providing clear, well-documented examples that cover a variety of use cases. Each example is designed to be self-contained and to progressively build on previous concepts.

Whether you're completely new to Rust or looking to refine your knowledge with more advanced examples, this repo will guide you through the core concepts and beyond.

## Getting Started

To run the examples in this repository, you will need to have Rust installed on your machine. If you don't have Rust installed yet, you can follow the official [installation guide](https://www.rust-lang.org/tools/install).

Run all the tests with the following command :
```bash
RUST_LOG=info RUSTFLAGS="-Awarnings" cargo +nightly test -- --nocapture
```

### Installation Steps

1. Install Rust using `rustup` (recommended):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

