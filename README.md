# KaaDivisors
Simple CLI program for finding divisors of huge numbers

Very FAST. Written in Rust.

[![License](https://img.shields.io/badge/License-%20Apache%202.0-green.svg)](#)
[![License](https://img.shields.io/badge/License-%20MIT%20-green.svg)](#)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)

## Getting Started
`cargo install kaadivisors`  
`kaadivs`

## Output Example
```
This program returns divisors of a number
----------
2 ^1
3 ^2
5 ^1
3607 ^1
3803 ^1
----------
Finished: 28.299µs
```
All divisors are PRIME NUMBERS (2, 3, 5, 3607, 3803).

`3 ^2` - here `3` is a <ins>prime divisor</ins>, `^2` is a <ins>power</ins> (3<sup>2</sup> = 3 × 3).

Indeed, 2 × 3<sup>2</sup> × 5 × 3607 × 3803 = 1234567890

## Time Complexity

![scatter](scatter.png)
**xaxis** - values 1 to 1 000 000  
**yaxis** - execution time, microsecs

The measurements based on the algorithm in the **Rust** language. The scatter was made by Plotly (Python).

You can take a closer look in the [scatter.html](scatter.html). Download it, because it's too big to be shown here.

## All Commands
* `kaadivs` or `kaadivisors`
* `kaadivs <your_number>` (example: kaadivs 123)
