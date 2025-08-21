use patlite_udp::{send_pns, Buzzer, La6Colour, Flash, PatliteConfig, PatliteState};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state: PatliteState = PatliteState {
        tier1: La6Colour::Green,
        tier2: La6Colour::Amber,
        tier3: La6Colour::Off,
        tier4: La6Colour::White,
        tier5: La6Colour::Red,
        flash: Flash::On,
        buzzer: Buzzer::Off,
    };
    let cfg: PatliteConfig = PatliteConfig {
        ip: "192.168.1.2".to_string(),
        port: 10000,
    };
    _ = send_pns(&cfg, state);
    Ok(())
}
