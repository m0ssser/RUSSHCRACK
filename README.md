
# Description 💻

## _RUSSHCRACK 🐱‍👤_

**RUSSHCRACK** is a multi-threaded SSH brute-force tool written in Rust. It attempts to brute-force SSH login credentials using usernames and passwords from provided wordlists. The tool leverages the `ssh2` crate to handle SSH connections and supports custom configurations for threads, timeouts, and target hosts.

> **Features**

• Multi-threaded brute-forcing for fast credential testing.

• Customizable SSH target IP, port, username, and password wordlists.

• Colored output for better visibility of successful, failed, or connection errors.

• User-friendly command-line interface.

## Prerequisites 📋

> Before using **RUSSHCRACK**, ensure you have the following:

• Rust installed on your system.

• Valid wordlists for usernames and passwords with read and write permission (use `chmod 644 file_name.txt` to set file permissions).

• Install necessary dependencies by adding them to your `Cargo.toml`:

```toml
[dependencies]
ssh2 = "0.9.1"
colored = "2.0"
```

## Installation 💾

Clone the repository from GitHub:

```bash
git clone https://github.com/m0ssser/RUSSHCRACK.git
```

Navigate to the project directory:

```bash
cd RUSSHCRACK
```

Build the project:

```bash
cargo build --release
```

The compiled binary will be available in the `target/release/` directory.

## Usage 🚀

Run the binary or use `cargo run` to execute:

```bash
cargo run
```

You'll be prompted for:

- **Target IP address**: Enter the SSH server's IP address.
- **Target Port**: Specify the port (default is `22`).
- **Username Wordlist**: Path to the username wordlist file.
- **Password Wordlist**: Path to the password wordlist file.

> **Example**:

```bash
Enter target IP address: 192.168.1.100
Enter target port (default 22): 22
Enter path to username wordlist: ./usernames.txt
Enter path to password wordlist: ./passwords.txt
[+] Starting Bruteforce...
```

Successful attempts will be shown in **green**, and failures in **red**.

## Legal Disclaimer ⚠️

This tool is for educational purposes only. It should only be used for testing networks or systems with explicit permission. Unauthorized access is illegal and punishable by law. The author is not responsible for any misuse.

## License 📄

This project is licensed under the MIT License.
```

