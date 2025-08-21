use patlite_udp::{send_pns, Buzzer, Color, Flash, PatliteConfig, PatliteState};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state: PatliteState = PatliteState {
        tier1: Color::Green,
        tier2: Color::Amber,
        tier3: Color::Off,
        tier4: Color::White,
        tier5: Color::Red,
        flash: Flash::On,
        buzzer: Buzzer::Off,
    };
    let cfg: PatliteConfig = PatliteConfig {
        ip: "192.168.0.60".to_string(),
        port: 10000,
    };
    _ = send_pns(&cfg, state);
    Ok(())
}
