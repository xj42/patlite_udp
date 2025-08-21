//! Run with: `PATLITE_IP=192.168.0.60 PATLITE_PORT=10000 cargo run --example simple`
use patlite_udp::{Buzzer, Color, Flash, *};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    set_stack_light(
        Color::Green,
        Color::Amber,
        Color::Off,
        Color::White,
        Color::Red,
        Flash::On,
        Buzzer::Off,
    )?;
    let state: PatliteState = PatliteState {
        tier1: Color::Green,
        tier2: Color::Green,
        tier3: Color::Off,
        tier4: Color::White,
        tier5: Color::Red,
        flash: Flash::On,
        buzzer: Buzzer::Off,
    };
    _ = set_stack_light_state(state);
    Ok(())
}
