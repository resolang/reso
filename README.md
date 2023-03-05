# Rust Reso

WIP! 

See https://gitlab.com/resolang/reso (originally at https://gitlab.com/lynnpepin/reso).


--- 

Scrap notes!

```
each iteration:

1. for input in input_nodes:
  1. get adjacent wires, logic, outputs
  2. calc and, or, xor from wires
  3. push 'or' to outputs, push 'xor' 'and' to logic nodes
2. for each logic node:
  1. push state to outputs
3. for each output node:
  1. push state to wires
4. for each wire:
  1. record state by updating reselboard
  2. update pixels (if outputting image)
5. then:
  1. if outputting image, update image state, then output
  2. reset all temporary state
```