# 🛡️ AEGIS - Anti-Phishing Sentinel

Aegis is a lightweight, real-time email security shield written in Rust. It runs stealthily in the background (using IMAP IDLE) to intercept and analyze incoming emails, neutralizing phishing attempts and malicious attachments before the user can interact with them.

## 🚀 Key Features

* **Real-Time Monitoring (IMAP IDLE):** Passively listens to the inbox with near-zero system resource consumption. Wakes up instantly upon receiving a new email.
* **URL Analysis (Anti-Typosquatting):** Extracts hidden links from raw MIME/HTML content and compares domains against a curated list of legitimate targets using the Levenshtein distance algorithm.
* **Attachment Analysis (Malware Detection):** Calculates the SHA-256 footprint of suspicious executables and queries the VirusTotal API to detect known malware without executing the file locally.
* **Automated Action & Alerting:**
    * Triggers native OS desktop notifications (Safe/Danger).
    * Automatically quarantines threats by moving malicious emails to the Trash or Spam folder.

## 🛠️ Technical Architecture

* **Language:** Rust (Chosen for memory safety and blazing-fast performance).
* **Network:** IMAPS Protocol (Port 993, TLS encrypted).
* **Core Modules:**
    * `main.rs`: Orchestrator handling IMAP connection and the IDLE event loop.
    * `parser.rs`: Extracts MIME/HTML content, URLs, and file attachments.
    * `analyzer.rs`: Core logic engine (Levenshtein algorithm and VirusTotal API calls).
    * `notifier.rs`: OS integration for desktop alerts and IMAP commands for email deletion.

## ⚙️ Prerequisites & Setup

*(This section will be expanded as development progresses)*
* Rust and Cargo installed.
* A test email account with an "App Password" generated.
* A free VirusTotal API key.

### Configuration

Create a `.env` file in the root directory and configure it with your credentials (see `.env-template`):

```env
IMAP_DOMAIN=[your imap domain]
IMAP_EMAIL=[your email address]
IMAP_PASSWORD=[follow the instructions]
```

---
*Developed as a personal cybersecurity project.*
