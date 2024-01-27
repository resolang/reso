Publishing:
- resolang.dev
  - custom theme
  - logo in title
  - custom favicon
  - new logo
- Documentation
  - markdownbook
  - Getting started
  - Example circuits
  - Component library spritesheet
- Re-arrange repo
  - `oranda` site is separate
  

Features and implementation:
- Run code through a linter
- Direct-to-gif output
- Custom palettes
- Better ascii support
- Move to `Grid<>` from `Vec<Vec<>>`
- New logo
- Consistency with concepts. (Region/node/element. Element 'class/order', wire 'color'.)
- ResoCircuit serialize/deserialize

Tooling:
- Circuit viewer, simulation controller
- WASM drag-and-drop simulator
- IPC
- ResoCircuit debugger / analyzer / anatomizer tool
- Circuit live editor

New element types:
- GPIO wire,
- Bus (Cyan range, 256 values)
