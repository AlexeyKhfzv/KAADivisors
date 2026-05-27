# Divisors Very Fast
Simple CLI program for finding divisors of huge numbers.

Very FAST. Written in Rust.

[![License](https://img.shields.io/badge/License-%20Apache%202.0-green.svg)](#)
[![License](https://img.shields.io/badge/License-%20MIT%20-green.svg)](#)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)

## Getting Started
`cargo install divisors_very_fast`  
`divisors`

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

## Special Mode
Yes, my program has special "Expedited" mode!  
Run it - `EXPEDITED=1 divisors`

In this mode my program gets already found divisors from file "found_divisors.bin".  
You always can write the count of them by self - `EXPEDITED=1 divisors write_divisors <count>`

## Time Complexity

![scatter](scatter.png)
**xaxis** - values 1 to 1 000 000  
**yaxis** - execution time, microsecs

The measurements based on the algorithm in the **Rust** language. The scatter was made by Plotly (Python).

You can take a closer look in the [scatter.html](scatter.html). Download it, because it's too big to be shown here.

## All Commands
* `divisors` or `divisors_very_fast`
* `divisors <your_number>` (example: divisors 123)
* `EXPEDITED=1 divisors`
* `EXPEDITED=1 divisors <your_number>`
* `EXPEDITED=1 divisors write_divisors <count>`
