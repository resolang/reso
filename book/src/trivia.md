# Trivia and history


"Reso" refers to both the circuit description language and the simulator. 

Reso is not a [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton), because *regions* of cells update neighboring *regions*, allowing them infinite reach, whereas cellular automata operate over a finite neighborhood.

Reso instead defines [a digital logic circuit](https://en.wikipedia.org/wiki/Logic_gate) graph. That is to say, the *visual language* is used to define a *logic graph*.

Reso is called "visual" rather than "graphical", because saying "the visual language is compiled to a graph" is less confusing than "the graphical language is compiled to a graph".

## History

In 2015, took a digital logic design course at UConn and came up with the initial sketch for Reso. This course lifted the veil on the fantastic Minecraft Redstone computers I had seen years prior. I proposed the idea as a senior design project in 2017, but it was rejected.

Early 2018, I implemented a [custom three-species Game of Life](https://github.com/lynnpepin/rgblife) and a [three-species Brian's Brain-esque cellular automata](https://github.com/lynnpepin/brainbow). Implementing these gave me good enough chops at working on a grid to implement Reso.

So I ultimately implemented Reso in Python in the summer of 2018. [You can see the original Python implementation here.](https://github.com/lynnpepin/reso). I tried learning Rust this summer with the intention of reimplementing Reso, but I didn't get too far.

In 2021, I reworked Reso, updating the palette and logo and cleaning up the code for [presentation at the 2021 BangBangCon (!!Con)](https://www.youtube.com/watch?v=2Mst6EWqQJc).

In 2022, `ashirviskas` [made a Rust implementation](https://github.com/ashirviskas/rust_reso/), but I honestly did not know enough Rust to understand it! I went back to doing my own implementation.

Early 2023, I was able to start writing Rust in earnest. I learned a lot while writing [Phantasm, a fantasy assembly language](https://github.com/lynnpepin/phantasm/).

Over this whole time, Reso was stewing in my mind, and I realized a number of improvements that could be made. So, when I reimplemented Reso in Rust in late 2023, I did a complete overhaul, with very little translation from the original Python source.

Honestly, I am not great at Rust. If you're someone who thinks they know better, I'd happily welcome improvements and criticism! 
