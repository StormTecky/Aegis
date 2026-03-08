use dotenv::dotenv;
use native_tls::TlsConnector;
use std::env;

fn main() {
    println!("🛡️ Starting Aegis sentinel...");

    dotenv().ok();

    let domain = env::var("IMAP_DOMAIN")
        .expect("Error: IMAP_DOMAIN missing in .env")
        .trim()
        .to_string();

    let email = env::var("IMAP_EMAIL")
        .expect("Error: IMAP_EMAIL missing in .env")
        .trim()
        .to_string();

    let password = env::var("IMAP_PASSWORD")
        .expect("Error: IMAP_PASSWORD missing in .env")
        .trim()
        .to_string();

    let tls = TlsConnector::builder()
        .build()
        .expect("Failed to create TLS connector");

    println!("🔄 Connecting to server {}...", domain);
    let client = imap::connect((domain.as_str(), 993), domain.as_str(), &tls)
        .expect("Failed to connect to the IMAP server");

    let mut imap_session = client
        .login(&email, &password)
        .map_err(|e| e.0)
        .expect("Authentication failed: check your email or password");

    println!("✅ Authentication successful!");

    imap_session
        .select("INBOX")
        .expect("Failed to select INBOX");
    println!("📥 INBOX selected and ready.");

    println!("👀 Entering Sentinel mode. Press Ctrl+C to stop.");

    loop {
        println!("⏳ Waiting for new emails (IMAP IDLE)...");
        let mut idle_session = imap_session.idle().expect("Failed to initialize IDLE mode");
        idle_session.wait_keepalive().expect("Failed to wait for new emails");
        imap_session = idle_session.done().expect("Failed to resume normal session");
        println!("🔔 Wake up! Activity detected in the INBOX.");
    }
}
