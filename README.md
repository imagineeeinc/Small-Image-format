# Small Image format
a small image format for portable images.

## Current Edition is 0.01.0-re

# Philosophy
The main idea of Small Image Format(sif) is to be a highly portable image format, being small in size while having losing no data; uses a lossless compression known as [Run Length Encoding](https://en.wikipedia.org/wiki/Run-length_encoding). It also emphasises on ease of implementation, easy to read/ edit and for eductaion (teaching how image starage and compresion works).

# Things in this repo

There are encoders, decoders, converters and viewers in this repo mainly written in rust and js. Check it out in [The Projects Section](#the-projects)

# Specification
The Specs:
- [Run Length Encoding(RLE)](#rle)
- [Colour](#colour)
- [Storing of the image](#storing-of-the-image)
  - [Rows](#rows)
  - [Representing Pixels](#representing-pixels)
- [Meta Data](#meta-data)
  - width
  - height
  - edition
  - data
- [Final Representation](#final-representation)
- [Edition](#editions)
## RLE

Run length encoding takes multiple pixels that are same together and groups them into the space of one when being stored.

Example:
```
Original Image:
[re][re][wh][wh]
[re][re][wh][gr]
[gr][gr][gr][gr]

Turned into:
[2re][2wh]
[2re][1wh][1br]
[4gr]
```

## Colour

The colours are stored in Hex colour codes and compiling them into one 8-digit string(RGBA: RED,GREEN,BLUE,ALPAH(TRANSPERENCY)).
```
FF FF FF FF
|  |  |  |_Alpha
|  |  |____Blue
|  |_______Green
|__________Red
*There won't be spaces

This is the colour white
```
<br>
Experiments:
- Storing colours in rgb(a) colour
- storing the colours in actual bytes rather than text

## Storing of the image

The Format of storing the image

### Rows

THe image is stored row by row in a file, Each row is just a: 
```
[pixels...]
```

A square bracket with pixels inside, like an array.

Rows are stored sequentially by separating a bracket pair; it is inferred the row after another row always comes after the one before it.
```
[pixels...][pixels...]
```
Also the number pixels should add up to the width of the image on each row

### Representing Pixels

Each pixel is the number of pixels there are the same after it and the colour. Represented like this:
```
1-ff00ffff
|| |
|| |_the colour
||
||___Seperator seperating the colour from the number
|
|____is the number of pixels this is repeated for
```

Example of 5 pixels next to each other being yellow
```
5-ffff00ff
```

As these pixels will be inside a row that is an array, each pixel is separated by a comma
```
A row(array)->[1-ff0000ff,2-ffff00ff,3-ffffffff,1-00ffffff][Next Row...]
```
## Meta Data

There will be meta data that can travel with the image. This will be stored in a JSON Format, This does have some controversies like implementing will be harder in programming languages that doesn't support JSON, but usually some one on the community has probably made a module/ package to parse JSON, JSON has been specifically chosen for its availability and low learning curve.

```
- width [integer]-required             The width of the image in pixels
- height [integer]-required            The height of the image in pixels
- edition [string]-optional            The image format edition
- data [string]-optional               any extra payload/ data can be transported in this, for apps and decoders to interpret
                                       eg: Title, Description, Geo Location, copyright, orientation
```

## Final Representation

The final representation will be stored in an efficient way so any image viewer can decode it without interference; here is the order:
```
{...json meta data...}
          |
          |_the meta data are stored in curlly braces
{[...pixels...][...pixels...][...pixels...]...}
          |
          |_the rows with pixeles are stored in curlly braces with no extra anything

*the new line can be replaced with nothing making it a one line file
```

## Editions

There will be editions which add minor changes, its basically version of the format that allow the decoder to change its strategy for decoding. For example there is 1.2.0-rgb edition, its basically the 1.2.0 version with rgb colour codes over hex; so, when the encoder sees this it automatically changes to support rgb colours.

# Contributions

If you feel like you want to help with the specifications by enhancing it, making it efficient and saving some space or to correct my English please open a pull request.

For the other projects on this repo like encoders, decoders, converters and viewers, feel free to fork and edit, and if you want open a pull request.

# The Projects
There are a few folders with implementations on the spec in languages.
## Encoder
As of now there is a encoder tool that converts images ([formats that are supported](https://github.com/image-rs/image#supported-image-formats)). Written in rust is a simple tool to test the specs; the plan is in the future to have a glorified single sif conversion tool that will have an encoder and decoder module inside it and a main cli tool.

**Usage**

```shell
# example file
cargo run -- --input examples/8tile.png --output examples/8tile.sif
# your own file
cargo run -- --input path/to/you/file.png --output path/to/output/file.sif

Usage: sif-encoder [options] <input file> <output file>

        Options:
        --input,-f:   Input file name
        --output,-o:  Output file name
```
# License

This project is under [Apache License 2.0](https://github.com/imagineeeinc/Small-Image-format/blob/main/LICENSE)

