mod zz_build_cfg;
use lightning_bolt11::wallet::create_vault;

fn main() {
    zz_build_cfg::ensure();
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        eprintln!("lightning-bolt11 — BTC wallet");
        eprintln!("usage: lightning-bolt11 [name]");
        return;
    }
    let name = args.get(1).map(String::as_str).unwrap_or("default");
    let vault = create_vault(name, "demo");
    println!("{} {}", vault.id, vault.accounts[0].address);
}
