use dotenv::dotenv;
use native_tls::TlsConnector;
use std::env;

fn main() {
    println!("🛡️ Starting Aegis sentinel...");

    // 1. Load environment variables
    dotenv().ok();

    // Retrieve and clean credentials
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

    // 2. Configure the secure connector (TLS)
    let tls = TlsConnector::builder()
        .build()
        .expect("Failed to create TLS connector");

    // 3. Establish connection to the IMAP server
    println!("🔄 Connecting to server {}...", domain);
    let client = imap::connect((domain.as_str(), 993), domain.as_str(), &tls)
        .expect("Failed to connect to the IMAP server");

    // 4. Authenticate with email and password
    let mut imap_session = client
        .login(&email, &password)
        .map_err(|e| e.0)
        .expect("Authentication failed: check your email or password");

    println!("✅ Authentication successful!");

    // 5. Select the INBOX
    imap_session
        .select("INBOX")
        .expect("Failed to select INBOX");
    println!("📥 INBOX selected and ready.");

    // 6. Clean logout
    imap_session.logout().expect("Error during logout");
    println!("🛑 Logout successful. End of test.");
}
