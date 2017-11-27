---
layout: default
title: What is WebAssembly?
description: What is WebAssembly?
---

At its core, WebAssembly is a *programming language*. Its specification,
which is managed by a WC3 Community Group, defines the semantics of the
laugage, but also a portable [binary format], as well as two different
textual representations of that binary format.

[binary format]: ../terms/binary-format.html

## Wasm is born from the web, but not tied to it

As the name sort of implies, WebAssembly is primarily being used on the
web. However, it is defined entirely agnostically from the web, and can
be used in any context: it's a general-purpose language.

This is accomplished by having the core spec having no reference to the
web. It also specifies an API for the embedding environment to provide
functionality available to running programs. The APIs that you can use
inside of browsers are therefore specified this way, "on top of" the core
Wasm spec, in a sense.

This also means that other embedding environments may specify different
functionality for things you can't do in a browser.

## Wasm is a virtual ISA

The name also implies the second important aspect of what WebAssembly is:
something low-level. More specifically, Wasm is a virtual instruction set
architecture (virtual ISA). In other words, it's a programming language
in the same way that assembly language is a programming language; it's
very low-level.

As a 'virtual' ISA, Wasm is defined in terms of a virtual machine, rather
than of physical hardware. This is what allows it to be portable.

Additionally, its low-level nature means that most people who use webassembly
don't write webassembly code directly, they instead use a programming language
that can compile *to* WebAssembly, like [Rust](https://www.rust-lang.org/).

## JavaScript does not compile to Wasm

To get the question out of the way, no, there are no implementations of
JavaScript for WebAssembly today. WebAssembly code often runs in the context
of existing JavaScript VMs, rather than a single Wasm VM that runs all
languages.

## Wasm is available everywhere

As ["Can I Use"](https://caniuse.com/#feat=wasm) shows, Wasm is supported
today in all evergreen browsers. The big holdback is non-Edge IE; however,
that will only dwindle over time. As more people use Wasm in their
applications, there are still some options for supporting these older
browsers; a proof-of-concept shim was created, but isn't ready for prime-time
yet.

## Wasm is sandboxed

Security is a big concern for the Wasm folks. The web platform already allows
for users to run arbitrarily code fairly securely: JavaScript. So Wasm
follows a similar model; Wasm programs can't escape outside of their sandbox.

Bugs happen, of course. But WebAssembly doesn't fundamentally change the
attack surface of the web platform.

## Wasm is still new, and is missing many features

The current WebAssembly spec is refered to as "the MVP." It's the minimum amount
of stuff needed to ship useful software. As such, there are some big limitations
of the Wasm platform at the time of writing. These are expected to be resolved
in the future, over time, like the rest of the web platform.

Specifically, here are some features that aren't in Wasm today, but will be
tomorrow (metaphorically):

* Garbage collection
* Exceptions
* Threads & Shared Memory
* [SIMD](https://en.wikipedia.org/wiki/SIMD)
* [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model) access

## Wasm is for every language, but 'no runtime' ones have an advantage

In theory, WebAssembly is language-agnostic: you can compile any language to it.
This is true in practice as well, but this question has some complexity to it.

The primary one is your language's runtime: the stuff that's in every binary, or
its virtual machine. Consider a language like Ruby: it runs in a virtual machine
of its own, and contains a garbage collector. Becuase Wasm doesn't have built-in
GC support, not only would your Ruby program need to be compiled to WebAssembly,
but so would the Ruby interpreter itself. Your program would then be running
inside the Ruby VM, which is running inside the Wasm VM. This also means that
the extra binary size of the whole VM would need to be included with your program.

This means, in my mind, languages without significant runtime requirements have
a big advantage when working with Wasm. To give you an idea of scale, a Rust
program that looks like this:

```rust
#[no_mangle]
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

compiles to a 116 *byte* Wasm file at the time of writing. That's much smaller
than an entire VM + the function itself.

Users care a lot about the speed of web applications, and smaller file sizes
mean that they're faster to download, faster to parse, and faster (in some
senses) to run.

> Once Wasm gains support for garbage collection, this could slim down languages
> that require it, as they'd be able to not bundle their own GC, and instead use
> the built-in one. We'll see!

## Wasm is a JavaScript alternative, not killer

One of the first things that people think when hearing about WebAssembly is
that it will somehow "kill" JavaScript. I think that's *incredibly*
misguided. In my experience, people that say this assume that the **only**
reason that people write JavaScript is because they have to, since it's the
only widely supported language on the web. In their minds, as soon as choice
is possible, JavaScript is done for.

First, this is wrong becuase the premise isn't true: while it's true that
JavaScript has some warts, it's also a language that's beloved by many
people. JavaScript programmers love it so much they took it to the
server-side with Node.js, where the choice of being able to use other
languages already exists. That Node is so popular on the server, to me, says
that many people will still choose JavaScript on its merits as a language,
even with the existence of WebAssembly.

Beyond preference, there's another good reason to choose JavaScript: it's
the default language of the platform, and therefore enjoys advantages that
are unique to it. For example, JavaScript doesn't suffer the binary size
problem described above; you already have its VM on your machine. As such,
if you don't want a no-runtime language, JavaScript has an advantage over
other languages that would be compiled to WebAssembly. Beyond binary size,
it has gold-standard bindings to the web platform, as it's the language that
those bindings are defined in! Other languages not borne of the web may
not be as well-integrated as JavaScript is. We'll see as libraries develop.