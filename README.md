# Small Image format
a small image format for portable images.

# Philosophy
The main idea of Small Image Format(sif) is to be a highly portable image format, being small in size while having losing no data; uses a lossless compresion known as [Run Length Encoding](https://en.wikipedia.org/wiki/Run-length_encoding). It also emphasises on ease of implementation, easy to read/ edit and for eductaion (teaching how image starage and compresion works).

# Things in this repo

There are encoders, decoders, converters and viewers in this repo mainly wrriten in rust and js

# Specification
The Specs:
- [Run Length Encoding(RLE)](#rle)
- [Colour](#colour)
- [Storing of the image](#storing-of-the-image)
  - [Rows](#rows)
  - [Representing Pixels](#representing-pixels)
- [Meta Data](#meta-data)
  - title
  - description
  - width
  - height
  - geo location
  - copyright
  - orientation
  - transperency
  - data
= [Final Representation](#final-representation)
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

THe image is tored row by row in a file, Each row is just a: 
```
[pixels...]
```

A square braket with pixels inside, like an array.

Rows are stored sequntialy by seperating a braket pair; it is infered the row after another row always comes after the one before it.
```
[pixels...][pixels...]
```
Also the number pixels should add up to the width of the image on each row

### Representing Pixels

Each pixel is the number of pixels ther are the same after it and the colour. Represnted like this:
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

As these pixels will be inside a row that is an array, each pixel is seperated by a comma
```
A row(array)->[1-ff0000ff,2-ffff00ff,3-ffffffff,1-00ffffff][Next Row...]
```
## Meta Data

There will be meta data that can travel with the image. This will be stored in a JSON Format, This does have some contraversies like implemeting will be harder in progrming languges that doesn't support JSON, but usally some one on the community has probably made a module/ package to parse JSON, JSON has been specificly choosen for its avaliblity and low learning curve.

```
- Title [string]                       The name of the image
- Desc [string]                        The description
- width [integer]                      The width of the image in pixels
- height [integer]                     The height of the image in pixels
- Geo Location [array]                 The geo coordinates
  - arg1: x coords
  - arg2: y coords
- copyright [string]                   The copyright information
- orientation [string]                 The orientation of the image
  - Option: (vertical | landscape)
- transpernacy [boolen]                Should it be a tranaspernt background
                                       (if false over-rides transperncy of default pixels to 1)
- data [string]                        any extra payload/ data can be transported in this
```

## Final Representation

The final representation will be strored in an efficent way so any image viewer can decode it without interference; here is the order:
```
{"...json meta data stored inside of DOUBLE inverted commas..."}
          |
          |_the meta data are stored in curlly braces
{[...pixels...][...pixels...][...pixels...]...}
          |
          |_the rows with pixeles are stored in curlly braces with no extra anything

*the new line can be replaced with nothing making it a one line file
```

# Contributions

If you feel like you want to help with the specifications by enhancing it, making it efficnet and saving some space or to correct my english please open a pull request.

For the other projects on this repo like encoders, decoders, converters and viewers, feel free to fork and edit, and if you want open a pull request.

# License

This project is under [Apache License 2.0](https://github.com/imagineeeinc/Small-Image-format/blob/main/LICENSE)

