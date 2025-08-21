//! Run with: `PATLITE_IP=192.168.0.60 PATLITE_PORT=10000 cargo run --example simple`
use patlite_udp::{Buzzer, Flash, La6Colour, *};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    set_stack_light(
        La6Colour::Green,
        La6Colour::Amber,
        La6Colour::Off,
        La6Colour::White,
        La6Colour::Red,
        Flash::On,
        Buzzer::Off,
    )?;
    let state: PatliteState = PatliteState {
        tier1: La6Colour::Green,
        tier2: La6Colour::Green,
        tier3: La6Colour::Off,
        tier4: La6Colour::White,
        tier5: La6Colour::Red,
        flash: Flash::On,
        buzzer: Buzzer::Off,
    };
    _ = set_stack_light_state(state);
    Ok(())
}
