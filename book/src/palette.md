# Palette

> ![./palettes/reso-12-32x.png](./palettes/reso-12-32x.png)
> 
> *The 12-color default Reso palette*


## Twelve-color palette

There are ten semantically relevant colors which get compiled, the other 16777206 colors are ignored. Black and white are reserved to *never* have semantic meaning. Pixels with these colors define regions in the logic graph. 

![./palettes/reso-12-32x.png](./palettes/reso-12-32x.png)


| Color          | Meaning               | Hex code       | RGB               | ASCII |
| -------------- | --------------------- | ---            | ----------------- | ----- |
| Dark  orange   | Orange wire (off)     | ```#804000```  | `(128,  64,   0)` | `o`   | 
| Bright orange  | Orange wire (on)      | ```#ff8000```  | `(255, 128,   0)` | `O`   |
| Dark sapphire  | Sapphire wire (off)   | ```#004080```  | `(  0,  64, 128)` | `s`   |
| Bright sapphire| Sapphire wire (on)    | ```#0080ff```  | `(  0, 128, 255)` | `S`   |
| Dark lime      | Lime wire (off)       | ```#408000```  | `(64,  128,   0)` | `l`   |
| Bright lime    | Lime wire (on)        | ```#80ff00```  | `(128, 255,   0)` | `L`   |
| Dark teal      | AND logic node        | ```#008040```  | `(  0, 128,  64)` | `&`   |
| Bright teal    | XOR logic node        | ```#00ff80```  | `(  0, 255, 128)` | `^`   |
| Dark purple    | Input (wire to node)  | ```#400080```  | `( 64,   0, 128)` | `+`   |
| Bright purple  | Output (node to wire) | ```#8000ff```  | `(128,   0, 255)` | `=`   |
| White          | Comment/whitespace    | ```#ffffff```  | `(255, 255, 255)` | ` `   |
| Black          | Comment/whitespace    | ```#000000```  | `(  0,   0,   0)` | ` `   |

For backwards compatibility with new functionality, we reserve a total of 48 colors. (This is by convention and is not enforced by the Reso simulator.)


## Reserved 48-color palette

The ten colors above (other than black and white) are semantically relevant, which means all the other valid three-byte RGB values are not.

New features will use new colors. Reso is at `0.0.x`, so no promises, but we'll try to stick within these 48 values for any new features:

| Hue               | Saturated (1) | Dark (2)      | Light (3)     | Unsaturated (4) |
| ---               | ---           | ---           | ---           | ---           |
| **Red (R)**       | ```#ff0000``` | ```#800000``` | ```#ff8080``` | ```#804040``` |
| **Yellow (Y)**    | ```#ffff00``` | ```#808000``` | ```#ffff80``` | ```#808040``` |
| **Green (G)**     | ```#00ff00``` | ```#008000``` | ```#80ff80``` | ```#408040``` |
| **Cyan (C)**      | ```#00ffff``` | ```#008080``` | ```#80ffff``` | ```#408080``` |
| **Blue (B)**      | ```#0000ff``` | ```#000080``` | ```#8080ff``` | ```#404080``` |
| **Magenta (M)**   | ```#ff00ff``` | ```#800080``` | ```#ff80ff``` | ```#804080``` |
| **Orange (O)**    | ```#ff8000``` | ```#804000``` | ```#ffc080``` | ```#806040``` |
| **Lime (L)**      | ```#80ff00``` | ```#408000``` | ```#c0ff80``` | ```#608040``` |
| **Teal (T)**      | ```#00ff80``` | ```#008040``` | ```#80ffc0``` | ```#408060``` |
| **Sapphire (S)**  | ```#0080ff``` | ```#004080``` | ```#80c0ff``` | ```#406080``` |
| **Purple (P)**    | ```#8000ff``` | ```#400080``` | ```#c080ff``` | ```#604080``` |
| **Violet (V)**    | ```#ff0080``` | ```#800040``` | ```#ff80c0``` | ```#804060``` |


## Palette downloads

Grab the palette in a convenient format!


Downloads mirrored from [RESO-12 at lospec.com](https://lospec.com/palette-list/reso-12).

- [.png (1x pixels),](./palettes/reso-12-1x.png)
- [.png (8x pixels),](./palettes/reso-12-8x.png)
- [.png (32x pixels),](./palettes/reso-12-32x.png)
- [.ase (Photoshop)](./palettes/reso-12.ase)
- [.gpl (GIMP)](./palettes/reso-12.)
- [.txt (Paint.NET)](./palettes/reso-12.)
- [.hex (line-separated list of hex)](./palettes/reso-12.)

Also available in raw hex, if copy-and-paste is more your thing:

```palette
804000,ff8000,004080,0080ff,408000,80ff00,008040,00ff80,400080,8000ff,ffffff,000000
```

[Submit an issue](https://github.com/resolang/reso/issues) if there's a convenient palette format you'd like to see. The RESO-12 palette is also available 


## Alternative palettes

Future versions of Reso will have support for 100% ASCII-based workflows, and for palette mappings, allowing you to execute Reso circuits over any language of your choosing.