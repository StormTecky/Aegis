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

    let mailbox = imap_session
        .select("INBOX")
        .expect("Failed to select INBOX");
    println!("📥 INBOX selected and ready.");

    let mut next_uid = mailbox.uid_next.unwrap_or(1);

    println!("👀 Entering Sentinel mode. Press Ctrl+C to stop.");

    loop {
        println!("⏳ Waiting for new emails (IMAP IDLE)...");

        let idle_session = imap_session.idle().expect("Failed to initialize IDLE mode");

        idle_session
            .wait_keepalive()
            .expect("Error while waiting for server events");

        println!("🔔 Wake up! Activity detected in the INBOX.");
        println!("🔍 Searching for newly received messages...");
        let search_query = format!("UID {}:*", next_uid);
        let new_messages = imap_session
            .uid_search(&search_query)
            .expect("Failed to search for new messages");

        let new_messages: Vec<u32> = new_messages
            .into_iter()
            .filter(|&uid| uid >= next_uid)
            .collect();

        if new_messages.is_empty() {
            println!("❌ No new messages found.");
        } else {
            println!("✅ Found {} new messages.", new_messages.len());
            for msg_uid in new_messages {
                if msg_uid >= next_uid {
                    next_uid = msg_uid + 1;
                }
                let fetch_result = imap_session
                    .uid_fetch(msg_uid.to_string(), "(ENVELOPE)")
                    .expect("Failed to fetch message");
                for msg in &fetch_result {
                    if let Some(envelope) = msg.envelope() {
                        let from = envelope
                            .from
                            .as_ref()
                            .and_then(|f| f.first())
                            .map(|addr| {
                                let mailbox = addr
                                    .mailbox
                                    .as_ref()
                                    .map(|m| String::from_utf8_lossy(m))
                                    .unwrap_or_default();
                                let host = addr
                                    .host
                                    .as_ref()
                                    .map(|h| String::from_utf8_lossy(h))
                                    .unwrap_or_default();
                                format!("{}@{}", mailbox, host)
                            })
                            .unwrap_or_else(|| "Unknown Sender".to_string());
                        let subject = envelope
                            .subject
                            .as_ref()
                            .map(|s| String::from_utf8_lossy(s).into_owned())
                            .unwrap_or_else(|| "No Subject".to_string());
                        println!("🚨 [NEW THREAT SCAN] From: {} | Subject: {}", from, subject);
                    }
                }
            }
        }
        println!("🛡️ Aegis is going back to sleep...");
        println!("--------------------------------------------------");
    }
}
