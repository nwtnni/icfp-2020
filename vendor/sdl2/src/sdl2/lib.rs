//! # Getting started
//!
//! ```rust,no_run
//! extern crate sdl2; 
//!
//! use sdl2::pixels::Color;
//! use sdl2::event::Event;
//! use sdl2::keyboard::Keycode;
//! use std::time::Duration;
//! 
//! pub fn main() {
//!     let sdl_context = sdl2::init().unwrap();
//!     let video_subsystem = sdl_context.video().unwrap();
//! 
//!     let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
//!         .position_centered()
//!         .build()
//!         .unwrap();
//! 
//!     let mut canvas = window.into_canvas().build().unwrap();
//! 
//!     canvas.set_draw_color(Color::RGB(0, 255, 255));
//!     canvas.clear();
//!     canvas.present();
//!     let mut event_pump = sdl_context.event_pump().unwrap();
//!     let mut i = 0;
//!     'running: loop {
//!         i = (i + 1) % 255;
//!         canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//!         canvas.clear();
//!         for event in event_pump.poll_iter() {
//!             match event {
//!                 Event::Quit {..} |
//!                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//!                     break 'running
//!                 },
//!                 _ => {}
//!             }
//!         }
//!         // The rest of the game loop goes here...
//!
//!         canvas.present();
//!         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//!     }
//! }
//! ```

#![crate_name = "sdl2"]
#![crate_type = "lib"]

#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless, transmute_ptr_to_ref))]

extern crate num;
pub extern crate libc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;
pub extern crate sdl2_sys as sys;

#[cfg(feature = "gfx")]
extern crate c_vec;

pub use crate::sdl::*;

pub mod clipboard;
pub mod cpuinfo;
#[macro_use] pub mod macros;
pub mod event;
pub mod filesystem;
pub mod gesture;
pub mod touch;
pub mod joystick;
pub mod controller;
pub mod haptic;
pub mod keyboard;
pub mod mouse;
pub mod rect;
pub mod surface;
pub mod pixels;
pub mod video;
pub mod timer;
pub mod render;
pub mod rwops;
pub mod log;
mod sdl;
pub mod audio;
pub mod version;
pub mod messagebox;
pub mod hint;

// modules
#[cfg(feature = "ttf")]
pub mod ttf;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "mixer")]
pub mod mixer;
#[cfg(feature = "gfx")]
pub mod gfx;

mod common;
// Export return types and such from the common module.
pub use crate::common::IntegerOrSdlError;
