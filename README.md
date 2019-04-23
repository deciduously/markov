# markov

A mini markov chain, translated from the [example by orangeduck](http://theorangeduck.com/page/17-line-markov-chain)

## Requirements

[Rust](https://rustup.rs)

## Usage

```
$ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/markov -h`
markov 0.1.0
deciduously <ben@deciduously.com>

USAGE:
    markov [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input text file
    -l, --length <length>    Output length
```

## Example Output

```
$ cargo run --release
   Compiling markov v0.1.0 (/home/ben/code/markov)
    Finished release [optimized] target(s) in 0.58s
     Running `target/release/markov`
An actor experiences
Other peoples lives
Through a metamorphosis of mind

Words sifted through a Forest, beneath the blowing gale,
The waves have now the year of 1897, and on like that.
I can't abear it. I killed last night.

I wonder, 'struth, I wonder if the listener please,
A most important thing;
But Fortune to a thousand times, but I
 Would have him with his prophetic bill.
The great Colosse, erect to Memory;
And what the royal feast!
See here the blue night, railway stations.

The water and fire his courage on despair
And utter dissolution, as the love of slaughter;
Many indeed are the men
With spears gathering at his feet: and my evening hours.

Last evening when it rests,
Leaves to be 
Of work may be shared by not crossing the line,
Though that same morning officers and men.

Continues yet the dream
```