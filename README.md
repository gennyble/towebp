# towebp

A simple command-line tool to convert common image formats to WebP.

### Installation
```
cargo install towebp
```

### Usage
You use this tool like `cat`; just provide a list of files. Here's how you might
convert `a.png` and `some.jpg` into `a.webp` and `some.webp`, respectively:
```
towebp a.png some.jpg
```

This also means you can convert all the PNGs in a directory like this:
```
towebp *.png
```

`towebp` will print errors to stderr if a file fails to parse or decode as an image,
but will continue on with the rest of the files, like `cat`.

There are two flags available for selecting quality and changing to the lossless
encoder. You can only specify one at a time. Using the same imaginary files as before,
you can encode the png losslessly with `-l` and the jpeg with a lossy quality of 75
with `-q 75`. Like this:
```
towebp -l a.png
towebp -q 75 some.jpg
```

### License
This program is licensed under Creative Commons Zero; it's in the public domain.
