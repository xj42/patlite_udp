//! patlite-udp
//!
//! A tiny, std-only UDP client for PATLITE-like stack lights that accept
//! the `AB D` (Detailed Motion Control) frame shown below.
//!
//! ```text
//! Header: 0x41 0x42 ('AB'), 0x44 ('D'), 0x00 0x00, 0x07
//! Data:   [Tier1, Tier2, Tier3, Tier4, Tier5, Flash, Buzzer]
//! ```
//!
//! ## Quick start
//! ```no_run
//! use patlite_udp::{La6Colour, Flash, Buzzer, build_command, send_pns_command};
//!
//! # fn main() -> std::io::Result<()> {
//! let cmd = build_command(
//!     La6Colour::Green, La6Colour::Amber, La6Colour::Off, La6Colour::White, La6Colour::Red,
//!     Flash::On, Buzzer::Off
//! );
//! send_pns_command("192.168.1.2", 10000, &cmd)?;
//! # Ok(()) }
//! ```
//!
//! ## With env (feature `env`, enabled by default)
//! ```no_run
//! # #[cfg(feature = "env")]
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! use patlite_udp::{La6Colour, Flash, Buzzer, set_stack_light};
//! set_stack_light(La6Colour::Red, La6Colour::Off, La6Colour::Off, La6Colour::Off, La6Colour::Off, Flash::Off, Buzzer::Off)?;
//! # Ok(()) }
//! # #[cfg(not(feature = "env"))]
//! # fn main() {}
//! ```

mod api;
mod models;

pub use crate::api::{
    build_command, send_pns, send_pns_command, send_pns_state, set_stack_light,
    set_stack_light_state,
};
pub use crate::models::{Buzzer, La6Colour, Flash, PatliteConfig, PatliteState};

#[cfg(feature = "env")]
pub use crate::api::load_patlite_config;
