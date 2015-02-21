I realise that in terms of learning Rust, I had jumped straight to the 'how' and
skipped the 'why'. I guess I am in enough of a Rust bubble that I can't imagine
why you wouldn't want to learn it. So, I will make a bit more of an effort to
explain why things are how they are. Here I will try to give a bit of an
overview/motivation.

If you are using C or C++, it is probably because you have to - either you need
low-level access to the system, or need every last drop of performance, or both.
Rust aims to do offer the same level of abstraction around memory, the same
performance, but be safer and make you more productive.

Concretely, there are many languages out there that you might prefer to use to
C++: Java, Scala, Haskell, Python, and so forth, but you can't because either
the level of abstraction is too high - you don't get direct access to memory,
you are forced to use garbage collection, etc. - or there are performance issues
- either performance is unpredictable or its simply not fast enough. Rust does
not force you to use garbage collection, and as in C++, you get raw pointers to
memory to play with. Rust subscribes to the 'pay for what you use' philosophy of
C++. If you don't use a feature, then you don't pay any performance overhead for
its existence. Furthermore, all language features in Rust have predictable (and
usually small) cost.

Whilst these constraints make Rust a (rare) viable alternative to C++, Rust also
has benefits: it is memory safe - Rust's type system ensures that you don't get
the kind of memory errors which are common in C++ - memory leaks, accessing un-
initialised memory, dangling pointers - all are impossible in Rust. Furthermore,
whenever other constraints allow, Rust strives to prevent other safety issues
too - for example, all array indexing is bounds checked (of course, if you want
to avoid the cost, you can (at the expense of safety) - Rust allows you to do
this in unsafe blocks, along with many other unsafe things. Crucially, Rust
ensures that unsafety in unsafe blocks stays in unsafe blocks and can't affect
the rest of your program). Finally, Rust takes many concepts from modern
programming languages and introduces them to the systems language space.
Hopefully, that makes programming in Rust more productive, efficient, and
enjoyable.

I would like to motivate some of the language features from part 1. Local type
inference is convenient and useful without sacrificing safety or performance
(it's even in modern versions of C++ now). A minor convenience is that language
items are consistently denoted by keyword (`fn`, `let`, etc.), this makes
scanning by eye or by tools easier, in general the syntax of Rust is simpler and
more consistent than C++. The `println!` macro is safer than printf - the number
of arguments is statically checked against the number of 'holes' in the string
and the arguments are type checked. This means you can't make the printf
mistakes of printing memory as if it had a different type or addressing memory
further down the stack by mistake. These are fairly minor things, but I hope
they illustrate the philosophy behind the design of Rust.
