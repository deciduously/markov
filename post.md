# Build You A Markov Chain (In Rust)

A [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) can be used to generate realistic-ish sounding random text based on a sample input.  The Wikipedia article somewhat opaque, as Wikipedia can tends to be, but at it's heart it's a very simple concept in which the next word is chosen pased entirely on the current two words.  It's surprisingly simple (at least, I was surprised) and yet generates some real-sounding text with minimal effort.



```
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

This post is largely a translation of [this post](http://theorangeduck.com/page/17-line-markov-chain) by [oragneduck](http://theorangeduck.com/page/about).  He's got lots of great stuff, go check him out!