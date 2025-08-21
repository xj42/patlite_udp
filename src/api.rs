use std::net::UdpSocket;
use std::{env, error::Error, io};

use crate::models::{Buzzer, Color, Flash, PatliteConfig, PatliteState};

/// Build the 13-byte command frame from discrete parts.
/// Header: 0x41 0x42 ('AB'), 0x44 ('D'), 0x00 0x00, 0x07 (data size)
/// Data:   [Tier1, Tier2, Tier3, Tier4, Tier5, Flash, Buzzer]
pub fn build_command(
    tier1: Color,
    tier2: Color,
    tier3: Color,
    tier4: Color,
    tier5: Color,
    flash: Flash,
    buzzer: Buzzer,
) -> [u8; 13] {
    [
        0x41,
        0x42, // 'AB'
        0x44, // 'D' = Detailed Motion Control
        0x00,
        0x00, // reserved
        0x07, // data size
        tier1 as u8,
        tier2 as u8,
        tier3 as u8,
        tier4 as u8,
        tier5 as u8,
        flash as u8,
        buzzer as u8,
    ]
}

/// Build the 13-byte command frame from a PatliteState.
pub fn build_command_from_state(state: PatliteState) -> [u8; 13] {
    [
        0x41,
        0x42, // 'AB'
        0x44, // 'D' = Detailed Motion Control
        0x00,
        0x00, // reserved
        0x07, // data size
        state.tier1 as u8,
        state.tier2 as u8,
        state.tier3 as u8,
        state.tier4 as u8,
        state.tier5 as u8,
        state.flash as u8,
        state.buzzer as u8,
    ]
}

/// Send raw command bytes over UDP to HOST:PORT.
pub fn send_pns_command(host: &str, port: u16, cmd: &[u8]) -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    let addr = format!("{}:{}", host, port);
    sock.send_to(cmd, addr)?;
    Ok(())
}

/// Send a PatliteState over UDP to HOST:PORT (builds the bytes for you).
pub fn send_pns_state(host: &str, port: u16, state: PatliteState) -> io::Result<()> {
    let cmd = build_command_from_state(state);
    send_pns_command(host, port, &cmd)
}

/// Send a PatliteState using a PatliteConfig (no env needed).
pub fn send_pns(cfg: &PatliteConfig, state: PatliteState) -> io::Result<()> {
    send_pns_state(&cfg.ip, cfg.port, state)
}

/// Convenience helper: build and send using env-configured target.
///
/// Requires feature `env` (enabled by default). Reads:
/// - `PATLITE_IP`
/// - `PATLITE_PORT` (u16)
#[cfg(feature = "env")]
pub fn set_stack_light(
    tier1: Color,
    tier2: Color,
    tier3: Color,
    tier4: Color,
    tier5: Color,
    flash: Flash,
    buzzer: Buzzer,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cfg = load_patlite_config()?;
    let cmd = build_command(tier1, tier2, tier3, tier4, tier5, flash, buzzer);
    send_pns_command(&cfg.ip, cfg.port, &cmd)?;
    Ok(())
}

/// Alternate convenience: pass a full PatliteState with env-configured target.
#[cfg(feature = "env")]
pub fn set_stack_light_state(state: PatliteState) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cfg: PatliteConfig = load_patlite_config()?;
    send_pns(&cfg, state)?;
    Ok(())
}

/// When the `env` feature is disabled, error out clearly.
#[cfg(not(feature = "env"))]
pub fn set_stack_light(
    _tier1: Color,
    _tier2: Color,
    _tier3: Color,
    _tier4: Color,
    _tier5: Color,
    _flash: Flash,
    _buzzer: Buzzer,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "set_stack_light requires the `env` feature",
    )))
}

/// Load `PATLITE_IP` and `PATLITE_PORT` from the environment.
#[cfg(feature = "env")]
pub fn load_patlite_config() -> Result<PatliteConfig, Box<dyn Error + Send + Sync>> {
    let ip = env::var("PATLITE_IP").map_err(|e| {
        let msg = format!("Missing env var PATLITE_IP: {}", e);
        Box::new(io::Error::new(io::ErrorKind::Other, msg)) as Box<dyn Error + Send + Sync>
    })?;

    let port_str = env::var("PATLITE_PORT").map_err(|e| {
        let msg = format!("Missing env var PATLITE_PORT: {}", e);
        Box::new(io::Error::new(io::ErrorKind::Other, msg)) as Box<dyn Error + Send + Sync>
    })?;

    let port = port_str.parse::<u16>().map_err(|e| {
        let msg = format!("Invalid PATLITE_PORT '{}': {}", port_str, e);
        Box::new(io::Error::new(io::ErrorKind::Other, msg)) as Box<dyn Error + Send + Sync>
    })?;

    Ok(PatliteConfig { ip, port })
}
