# oxidized-metal

Yeah, that's right. Rust. Let's talk about it.

## What is Rust?

Have you ever encountered a segmentation fault in C++? I'm sure you have. If you've taken any of the main programming courses at Kent State, you definitely have (I'm looking at you CS2). I could give you the true high-level breakdown of Rust but at it's core, it aims to solve that one problem: **Pointers are evil**

Now, I don't literally mean *pointers are evil*. Pointers are great. We love to use them and they have a lot of uses for us in programming. In reality, when I say pointers are evil, I mean that *pointers to dynamic memory that we allocated ourselves are evil*. That's a mouthful though, I mean try saying that 3x fast. Rust aims to fix this. The term we use for this, is called **memory safety**.

So, now let me answer that question: what is Rust? It's a tool like every other programming language. It has it's uses and there are times where you should and shouldn't use it. It has it's pros and cons, just like Python, C++ and any other language you can name off the top of your head (yes, even Lisp, even though I hope nobody is thinking about Lisp).

## Let's talk applications of Rust

**The dynamic array**: one of the first (and most common) applications of dynamic memory.

We run into a common problem with C++ that we need to allocate some array but we don't actually know what size we need this array to be. So, we throw it on the heap and stop worrying about it. But what if we forget to deallocate that memory.. or what if we forget to resize the array when we get more elements.. and the list goes on and on of problems with just straight up allocating some memory on the heap and then running with it.

Lets model this problem in Rust and see how easy it is to solve it without ripping our hair out (and segfaulting). We're going to build a simple website that:

- Sends raw bytes of [PPM images](https://en.wikipedia.org/wiki/Netpbm) to Rust
  - I wrote a tool (in Rust) that can do this, it's called [image-to-ppm](https://github.com/Struck713/image-to-ppm)
- Modifies that image data
- Send the modified data back to the browser

## Woah woah.. Rust is a native language?

With [WebAssembly](https://webassembly.org/) (or WASM), we can now write native code in Rust and then compile it to run in the browser. The meat of this tutorial is going to focus on using tools such as `wasm-pack` to compile our Rust to WASM and then run it on the web. 