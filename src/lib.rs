//! `ugly_graphics` is a library dedicated to performing drawing operations on CPU.
//! It aims to provide simple extendable API focusing on either pixel overwriting or modification via an apply function.
//!
//! # Abstractions
//!
//! There are several abstractions:
//!
//! - [`Image`](crate::image::Image) trait - something to store pixels in some
//!   manner.
//! - [`Painter`](crate::painter::Painter) struct - specific adapter to store
//!   an image and to use `Operation`s on.
//! - [`Strategy`](crate::strategy::Strategy) enum - an action to be performed
//!   on the pixel set.
//!   It is either an overwrite action or compute and overwrite action.
//! - [`Operation`](crate::operation::Operation) trait - something to be
//!   applied to the `Painter`.
//!
//! # Usage
//!
//! To draw something one is to pass an `Image` instance to the `Painter` and
//! to call draw methods of specific `Operations`:
//!
//! ```rust
//! # use ugly_graphics::image::sprite::Sprite;
//! # use ugly_graphics::operation::scanline::line::Line;
//! # use ugly_graphics::operation::scanline::rectangle::filled::FilledRectangle;
//! # use ugly_graphics::operation::scanline::rectangle::outline::OutlineRectangle;
//! # use ugly_graphics::painter::Painter;
//! # use ugly_graphics::strategy::Strategy;
//! #
//! # fn main() {
//!     // Create a sprite to store our data.
//!     let mut sprite = Sprite::<u8, 16, 11>::from_copies(b' ');
//!
//!     // Pass the sprite to the painter.
//!     let mut painter = Painter::new(&mut sprite);
//!
//!     // Draw an outline rectangle using the `Apply` strategy.
//!     {
//!         let delta = b'#' - b' ';
//!         let apply = |v| v + delta;
//!         let rectangle = OutlineRectangle::new((14, 1), (1, 9), Strategy::Apply(&apply));
//!         painter.draw(rectangle);
//!     }
//!
//!     // Draw a filled rectangle using the `Overwrite` strategy.
//!     {
//!         let rectangle = FilledRectangle::new((3, 3), (12, 7), Strategy::Overwrite(b'+'));
//!         painter.draw(rectangle);
//!     }
//!
//!     // Draw a line using the `Overwrite` strategy.
//!     {
//!         let line = Line::new((3, 3), (12, 7), Strategy::Overwrite(b'o'));
//!         painter.draw(line);
//!     }
//!
//!     // The expected result is the following set of bytes.
//!     let raw = [
//!         b"                ",
//!         b" ############## ",
//!         b" #            # ",
//!         b" # oo++++++++ # ",
//!         b" # ++oo++++++ # ",
//!         b" # ++++oo++++ # ",
//!         b" # ++++++oo++ # ",
//!         b" # ++++++++oo # ",
//!         b" #            # ",
//!         b" ############## ",
//!         b"                ",
//!     ];
//!     let expected = Sprite::from_raw(raw.map(Clone::clone));
//!     assert_eq!(sprite, expected);
//! # }
//! ```

#![no_std]
#![deny(missing_docs)]

pub mod image;
pub mod operation;
pub mod painter;
pub mod strategy;
pub mod view;

mod utility;
