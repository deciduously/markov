# Build You A Markov Chain In Rust (Or Whatever)

## AKA 65 Line Markov Chain (A Rewrite)

I've found that if you can translate something into Rust, you pretty much understand it.  In that spirit, this post covers a translation of the program in [this post](http://theorangeduck.com/page/17-line-markov-chain) by [orangeduck](http://theorangeduck.com/page/about) in Rust, with a little extra explanation.  In fact, it will probably move a little slow - depending on your comfort level, it may be skimmable!  You should be able to knock one out in whatever language you like after.

A [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) can be used to generate realistic(ish) sounding random text based on a sample input.  The Wikipedia article is somewhat opaque, as Wikipedia can tends to be, but at it's heart it's a very simple concept in which the next word is chosen based entirely on the current two words.  It's surprisingly simple (at least, I was surprised at how easy it was) and yet generates some real-sounding(ish) text with minimal effort.  For a fun example of this in action, check out the subreddit [/r/SubredditSimulator](https://www.reddit.com/r/SubredditSimulator/).  All of the posts and comments found there are generated using markov chains using their respective subreddits as input data.

You shouldn't need to know Rust to follow along.

### On Your Marks

If you're just here for the Markov Chain algorithm and not the Rust, skip down to the **Markov!** section.

This project requires stable [Rust](https://rustup.rs/).  Go there to get it if you need, and then spin up a project:

```
$ cargo new markov
$ cd markov/
```

### Get Set

Before hopping in, a quick 'n' dirty CLI would be nice for playing around with different inputs.  Luckily, Rust has a great option in [structopt](https://github.com/TeXitoi/structopt).  From the project root:

```
$ cargo add structopt
```

As the name implies this crate makes it easy to define an interface by simply defining a struct.  Add the following to the top of `src/main.rs`:

```rust
use std::{error::Error, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "markov")]
struct Opt {
    /// Input text file
    #[structopt(short = "i", long = "input")]
    input: Option<PathBuf>,
    /// Output length
    #[structopt(short = "l", long = "length")]
    length: Option<u32>,
}
```

The doc comments with the three slashes end up in the help string this crate will generate for us.  An example format for this struct would be something like `./markov -i poetry.txt -l 500`.  The long names are used with two dashes, like `--length`.  We type each as `Option<T>`, which means if either is omitted when the program is invoked this struct will just hold a `None`.  A `PathBuf` is a fancy owned `String` with [cross-platform path abstractions](https://doc.rust-lang.org/std/path/index.html) built in.  You can `push` to them and traverse them the same way on whichever platform your code runs.

Now, replace the template `println!` call given in `main()` with:

```rust
fn main() {
    let opt = Opt::from_args();
    let filename = opt
        .input
        .unwrap_or(PathBuf::from_str("poetry.txt").unwrap());
    let length = opt.length.unwrap_or(350);

    if let Err(e) = run(filename, length) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    };
}
```

If you're not new to Rust, that's probably fine and dandy.  If you are, let's unpack it a little.

First, we generate the struct itself from whatever was passed on the command line.  In the line `let opt = Opt::from_args()`, `Opt` is the struct we defined just above.  We can call the `from_args()` method on it because we derived the `StructOpt` *trait* for this struct with the `#[derive(StructOpt, Debug)]` line.  For the `./markov -i poetry.txt -l 500` example from above, we now have stored in the variable `opt`:

```rust
Opt(
    input: Some(PathBuf(inner: "poetry.txt")),
    length: Some(350u32),
)
```

All in-memory data structures will be presented in [RON](https://github.com/ron-rs/ron).

Note that the guts of `PathBuf` are omitted - it's an [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html) if you're curious but we just care it's a `PathBuf`.

The first thing to do is get something more concrete from those options to pass in to the program.  Using `unwrap_or()` is a great way to do this.  If the value is a `Some(thing)` it returns `thing`, and if it's `None` it returns the passed argument, and it's gotta be one of those two.

That `from_str` call we do to get our default `"poetry.txt"` `&str` value into a `PathBuf` is part of the `FromStr` trait and only works when that [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) is in scope.  It's an operation that can fail - for example, with a malformed path - so it returns a `Result<T, E>`.  This type acts like `Either` from Haskell, it either contains an `Ok(something: T)` or an `Err(error: E)` value.  You can get at the `T` of those with `unwrap()` if you're sure you'll have an `Ok`.  We know this one won't fail because we just made the input ourselves and it's not a malformed path, just a filename with an extension.  If you don't have something valid this will panic and crash.  It's almost always better to use something like `unwrap_or()`  or [pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html) to deal with the alternative cleanly!

Next we pass both in to an error-checked function.  It's good practice to take advantage of Rust's error handling for as much of your program as possible - this is a good way to force it!  The `if let` syntax is a way of capturing any error.  Our `run()` function here is going to return a `Result<T, E>`.   When called like this, if it ends up returning an `Ok(_)` nothing will happen, but if anything inside returns an error at any point, we'll execute the code path in this if block.  It uses destructuring - this line is saying that if the return value of `run()` can be destructured into an `Err(e)`, run this code.  The alternative is `Ok(_)` - in which case we know everything went fine, and there's no action to take.  If we wanted to do something else, we could have included an `else {}` block as well.  This error catch will use `eprintln!` to display the specific error information returned on `stderr` and end the program with an error code of 1, indicating it was not successful.

Of course, we need a properly typed `run()` function.  Here's a stub, just to get us to compile:

```rust
fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    Ok(())
}
```

The meat of our program expects concrete values, not `Option<T>`, and like good responsible Rustaceans we return a `Result<T, E>`, specifcally a `Result<(), Box<dyn Error>`.  Our success type, `()` stands for `unit` which is the empty tuple, akin to `void`.  This demo will just be outputting our random text to `stdout`, there's no value to return.  If you wanted to store the generated text and pass it to another part of your program, this might look like `Result<String, Box<dynError>>`.  The `Box<dyn Error>` type we're using for the Error type merits a little more explaining.

A [`Box<T>`](https://doc.rust-lang.org/std/boxed/index.html) is a boxed value - a basic heap-allocated value of type `T`.  Specifically the `Box` is a pointer to it, but a Rust-y smart pointer that knows about ownership and borrowing.  It's got a big name but it's just a pointer, nothing else.  This is useful because the `Box` has a size known at compile time, even if the value it points to may not.  The thing in the box with the `dyn Trait` syntax is a [*trait object*](https://doc.rust-lang.org/book/ch17-02-trait-objects.html).  `Error` from `std::error` is a trait that many types implement.  Using `dyn Error` we cover any type that implements the `Error` trait. This allows us to pass and catch many different types of errors in one function easily.

If you're brand new to Rust and that was a little too breezy, you're in for a real treat outside the scope of this post but don't worry - this part isn't necessary to understand the Markov bits below!  It's just some Rust boilerplate for clean and happy error handling without much setup.

Let's fire it up!  See if the help string is working with:

```
$ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/markov -h`
markov 0.1.0
you <you@you.cool>

USAGE:
    markov [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input text file
    -l, --length <length>    Output length
```

Groovy!  Thanks, structopt.  Don't forget:

```
$ git init
$ git add .
$ git commit -m "Initial commit"
```

### Markov!

The basic idea of this method of text generation is to choose the next word based on the current words.  Makes sense, right?  This algorithm never cares about more than two words at a time - it just knows all the possible options that come after that particular word duo from the source text.  Before we can start spewing out beautiful nonsense, we need to catalog .

We can implement this in Rust with a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as a lookup table.  One word doesn't quite give us enough context for realistic generation, though, so the keys of this hashmap will actually be combinations of two words.  These keys can be any type that implement the [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) traits, and a tuple of two strings works just fine.  We'll then store for the corresponding value every word in our source text that ever follows the combination of those two.  This way we can look up words likely to come next based on the current two words we have in our text.

Here's a concrete example what our hashmap might look like built from the source text "bears are big and bears are furry and bears are strong":

```rust
{
    ("bears", "are"): ["big", "furry", "strong"],
    ("and", "bears"): ["are"],
    ("are", "big"): ["and"],
    ("are", "furry"): ["and"],
    ("big", "and"): ["bears"],
    ("furry", "and"): ["bears"],
}
```

This source text would not provide terribly interesting output, but it demonstrates how this will work on a larger scale.  If our current selection is the words "are big", the only option in our table is the word "and" so we append that to our output.  Now our current selection is "big and", which also only has one option: "bears".  That leads us to look up "and bears", which comes back with "are", and now we get to randomly choose any of our options from the words stored under "bears are".

Our next word of the randomly generated text will always be pulled from this lookup table of words that do follow our current two words in the real text, which will (often) result in real-sounding sentences getting strung together even though each run through the loop is only ever aware of exactly where it is and nothing else.  On each iteration we perform a lookup of the proper tuple and select one of the options stored there at random.  Rinse and repeat for the length of the desired text!  Boom, nonsense.  The bigger the source text, the more interesting the output.

The first step in building this is to read in the source text.  This function will accept a `PathBuf` (which we've collected from the user already) and attempt to return the file's contents as a string:

```rust
fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    let mut bfr = BufReader::new(file);
    bfr.read_to_string(&mut contents)?;
    Ok(contents)
}
```

That return type is familiar from up above - this operation can fail, so we're wrapping it in a `Result<T, E>`.  The `T` here is the successfully read `String`, and our `E` is that trait object sweetness to catch any and all error types that may get thrown along the way.

This is my favorite run so far on the poetry set:

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

Continues, yet, the dream...
