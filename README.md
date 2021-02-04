# ticker
A command-line stock ticker that allows you to quickly check selected stocks.

## Running

First, fill out `.ticker.yml` with the list of stocks you want to track. Then
run the following command:

```
$ cargo run
```

You should see output like this:

```
STOCK             | SYMBOL | COST           | CHANGE    | % CHANGE
-------------------+--------+----------------+-----------+----------
 AMC Entertainment | AMC    | $ 9.59 (USD)   |  $ 1.77   | 22.63%
 GameStop          | GME    | $ 104.00 (USD) |  $ 14.00  | 15.56%
 Nokia             | NOK    | $ 4.64 (EUR)   |  $ 0.11   | 2.32%
 BlackBerry        | BB     | $ 12.15 (USD)  |  $ 0.60   | 5.19%
 Koss              | KOSS   | $ 28.13 (USD)  |  $ 8.13   | 40.65%
 ```
 
 This requires [Rust](https://www.rust-lang.org/) to be installed to compile.
