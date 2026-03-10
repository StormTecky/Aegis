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

        let idle_session = imap_session.idle().expect("Failed to initialize IDLE mode");

        idle_session
            .wait_keepalive()
            .expect("Error while waiting for server events");

        println!("🔔 Wake up! Activity detected in the INBOX.");
        println!("🔍 Searching for unread messages...");
        let unread_messages = imap_session.search("UNSEEN").expect("Failed to search for unread messages");
        if unread_messages.is_empty() {
            println!("❌ No unread messages found.");
        } else {
            println!("✅ Found {} unread messages.", unread_messages.len());
            for msg_id in unread_messages {
                let fetch_result = imap_session.fetch(msg_id.to_string(), "(ENVELOPE)").expect("Failed to fetch message");
                for msg in fetch_result {
                    if let Some(envelope) = msg.envelope() {
                        let subject = envelope.subject().as_ref().and_then(|f| f.first()).map(|addr| {
                            let mailbox = addr.mailbox.as_ref().map(|m| String::from_utf8_lossy(m)).unwrap_or_default();
                            let host = addr.host.as_ref().map(|h| String::from_utf8_lossy(h)).unwrap_or_default();
                            format!("{}@{}", mailbox, host)
                        }).unwrap_or_else(|| "Unknown Sender".to_string());
                        println!("🚨 [NEW THREAT SCAN] From: {} | Subject: {}", from, subject);
                    }
                }
            }
        }
        println!("🛡️ Aegis is going back to sleep...");
        println!("--------------------------------------------------");
    }
}
