# towebp

A simple command-line tool to convert common image formats to WebP.

### Installation
```
cargo intall towebp
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
but will continue on with the rest of the files, like cat.

### License
This program is licensed under Creative Commons Zero; It's in the public domain.