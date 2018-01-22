# Rust For Systems Programmers

A Rust tutorial for experienced C and C++ programmers.

Jump to [contents](#contents).
Jump to [contributing](#contributing).

This tutorial is intended for programmers who already know how pointers and
references work and are used to systems programming concepts such as integer
widths and memory management. We intend to cover, primarily, the differences
between Rust and C++ to get you writing Rust programs quickly without lots of
fluff you probably already know.

Hopefully, Rust is a pretty intuitive language for C++ programmers. Most of the
syntax is pretty similar. The big difference (in my experience) is that the
sometimes vague concepts of good systems programming are strictly enforced by
the compiler. This can be infuriating at first - there are things you want to
do, but the compiler won't let you (at least in safe code), and sometimes these
things *are* safe, but you can't convince the compiler of that. However, you'll
quickly develop a good intuition for what is allowed. Communicating your own
notions of memory safety to the compiler requires some new and sometimes
complicated type annotations. But if you have a strong idea of lifetimes for
your objects and experience with generic programming, they shouldn't be too
tough to learn.

This tutorial started as a [series of blog posts](http://featherweightmusings.blogspot.co.nz/search/label/rust-for-c).
Partly as an aid for me (@nrc) learning Rust (there is no better way to
check that you have learnt something than to try and explain it to somebody
else) and partly because I found the existing resources for learning Rust
unsatisfactory - they spent too much time on the basics that I already knew and
used higher level intuitions to describe concepts that could better be explained
to me using lower level intuitions. Since then, the documentation for Rust has
got *much* better, but I still think that existing C++ programmers are an
audience who are a natural target for Rust, but are not particularly well
catered for.


## Contents

1. [Introduction - Hello world!](hello%20world.md)
1. [Control flow](control%20flow.md)
1. [Primitive types and operators](primitives.md)
1. [Unique pointers](unique.md)
1. [Borrowed pointers](borrowed.md)
1. [Rc and raw pointers](rc%20raw.md)
1. [Data types](data%20types.md)
1. [Destructuring pt 1](destructuring.md)
1. [Destructuring pt 2](destructuring%202.md)
1. [Arrays and vecs](arrays.md)
1. [Graphs and arena allocation](graphs/README.md)
1. [Closures and first-class functions](closures.md)


## Other resources

* [The Rust book/guide](http://doc.rust-lang.org/book/) - the best place for
  learning Rust in general and probably the best place to go for a second opinion
  on stuff here or for stuff not covered.
* [Rust API documentation](http://doc.rust-lang.org/std/index.html) - detailed
  documentation for the Rust libraries.
* [The Rust reference manual](https://doc.rust-lang.org/reference/) - a little
  out of date in places, but thorough; good for looking up details.
* [Discuss forum](http://users.rust-lang.org/) - general forum for discussion or
  questions about using and learning Rust.
* [#rust irc channel](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) - probably
  the best place to get quick answers to your Rust questions if you like irc.
* [StackOverflow Rust questions](https://stackoverflow.com/questions/tagged/rust) - answers
  to many beginner and advanced questions about Rust, but be careful though - Rust
  has changed *a lot* over the years and some of the answers might be very out of date.


## Contributing

Yes please!

If you spot a typo or mistake, please submit a PR, don't be shy! Please feel
free to file [an issue](https://github.com/nrc/r4cppp/issues/new) for
larger changes or for new chapters you'd like to see. I'd also be happy to see
re-organisation of existing work or expanded examples, if you feel the tutorial
could be improved in those ways.

If you'd like to contribute a paragraph, section, or chapter please do! If you
want ideas for things to cover, see the [list of issues](https://github.com/nrc/r4cppp/issues),
in particular those tagged [new material](https://github.com/nrc/r4cppp/labels/new%20material).
If you're not sure of something, please get in touch by pinging me here
(@nrc) or on irc (nrc, on #rust or #rust-internals).


### Style

Obviously, the intended audience is C++ programmers. The tutorial should
concentrate on things that will be new to experienced C++ programmers, rather
than a general audience (although, I don't assume the audience is familiar with
the most recent versions of C++). I'd like to avoid too much basic material and
definitely avoid too much overlap with other resources, in particular the Rust
guide/book.

Work on edge case use cases (e.g., using a different build system from Cargo, or
writing syntax extensions, using unstable APIs) is definitely welcome, as is
in-depth work on topics already covered at a high level.

I'd like to avoid recipe-style examples for converting C++ code to Rust code,
but small examples of this kind are OK.

Use of different formats (e.g., question and answer/FAQs, or larger worked
examples) are welcome.

I don't plan on adding exercises or suggestions for mini-projects, but if you're
interested in that, let me know.

I'm aiming for a fairly academic tone, but not too dry. All writing should be in
English (British English, not American English; although I would be very happy
to have localisations/translations into any language, including American
English) and be valid GitHub markdown. For advice on writing style, grammar,
punctuation, etc. see the Oxford Style Manual
or [The Economist Style Guide](http://www.economist.com/styleguide/introduction).
Please limit width to 80 columns. I am a fan of the Oxford comma.

Don't feel like work has to be perfect to be submitted, I'm happy to edit and
I'm sure other people will be in the future.
