mod zz_build_cfg;
fn main() {
    zz_build_cfg::ensure();
    if std::env::args().any(|a| a == "--help") {
        eprintln!("grid-levels — paper bot");
        return;
    }
    let bars: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let r = grid_levels::engine::backtest(bars);
    println!("equity={:.2} fills={} bars={}", r.equity, r.fills, r.bars);
}
