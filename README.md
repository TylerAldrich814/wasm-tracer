# A Simplistic Ray Tracer, Written in Rust, Compuled in WebAssembly
## Nothing Serious, just a project I'm working on to help solidify 
## My Rust knowledge. Why Web Assembly? Why not.

###### NOTE: This(currently) does not work in Safari. Everything worked
######       smoothly up until I implemented Smart Pointers(Box<T>) for
######       Boxing a Vector of `hitable` traits. 
``` 
  let objects = Vec<Box<dyn Hittable>>; // <- Caused Sarari to stop loading 
                                        //    my WASM module. Even if I didn't
                                        //    use the objects variable!
```
######       For whatever reason, Safari doesn't like this? But it works
######       just fine in Chrome? I Spent a few minutes looking this issue 
######       up and I couldn't find anything.. If Anyone knows why this 
######       happens, let me know!

