# `ugly-graphics`

Ugly library for ugly graphics operations on CPU.

## Abstractions

There are several abstractions:

- `Image` trait - something to store pixels in some manner.
- `Painter` struct - specific adapter to store an image and to use `Operation`s on.
- `Strategy` enum - an action to be performed on the pixel set.
  It is either an overwrite action or compute and overwrite action.
- `Operation` trait - something to be applied to the `Painter`.

## Usage

1. Obtain an image (`1` and `2`);
2. Create a painter (`3`);
3. Construct a primitive (`4`);
4. Draw it (`5`);

```rust
fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);   // 1
    let mut adapter = Adapter::new(&mut image);   // 2
    let mut painter = Painter::new(&mut adapter); // 3

    let triangle = OverlappingTriangle::new(      // 4
        [(10, 210), (160, 300), (310, 20)],
        Rgb([0xff, 0x80, 0x00]).overwrite(),
    );
    painter.draw(triangle);                       // 5

    let mut path = env::current_exe()?;
    path.set_extension("png");
    let file = File::create(&path)?;

    let encoder = PngEncoder::new(file);
    encoder
        .write_image(
            &image,
            image.width(),
            image.height(),
            ExtendedColorType::Rgb8,
        )
        .map_err(io::Error::other)?;

    eprintln!("Generated image at {}", path.display());
    eprintln!("You can pass the output of this example as a path to some viewer...");
    eprintln!();
    println!("{}", path.display());

    Ok(())
}
```

## Optional features

- `bitvec-adapter` - an adapter to the `BitSlice` of the [`bitvec`](https://crates.io/crates/bitvec) crate;
- `image-adapter` - an adapter to the `ImageBuffer` of the [`image`](https://crates.io/crates/image) crate;

## License

The `ugly_graphics` crate is distributed under the `MIT` license.
