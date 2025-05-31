# Barter_Facilitator
An application that facilitates barter over TOR.

<h2>The instructions are used for Linux Mint so look for differences with your OS and adapt these instructions to your OS</h2>

<h3>Prerequisites for Rust and Application</h3>
<ol>
  <li>Install rust from here: https://www.rust-lang.org/tools/install</li>
  <li>The application has no database_url so you are going to have to type it in and there are instruction and/or helpful comments in the Barter_Facilitator.rs file</li>
</ol>

<h3>Prerequisites for Barter_Facilitator database</h3>
<ol>
  <li>type this in terminal then enter: sudo apt install mysql-server</li>
  <li>if installation screen asks to continue press y then enter</li>
  <li>type this in terminal then enter: sudo mysql_secure_installation</li>
  <li>if installation screen asks to validate with password press y then enter</li>
  <li>choose password validation level then enter</li>
  <li>create password then confirm password, the password must not be empty, then enter</li>
  <li>press y to confirm with password then enter</li>
  <li>when asked to remove anonymous users type n then enter</li>
  <li>when asked to dissallow remote login type n then enter</li>
  <li>when asked to remove test database type y then enter</li>
  <li>when asked to reload privelege tables type y then enter</li>
  <li>The database file provided to you must be in var/lib/mysql</li>
</ol>

<h3>Prerequisites for Barter_Facilitator database connection</h3>
<ol>
  <li>type this in terminal then enter: sudo apt install tor</li>
  <li>type this in terminal then enter: sudo systemctl start tor</li>
  <li>Add this to torrc file at the very bottom of config file: HiddenServiceDir /var/lib/tor/mysql_service/
HiddenServicePort 3306 127.0.0.1:3306
</li>
  <li>type this in terminal then enter: sudo systemctl restart tor</li>
  <li>type this in terminal then enter: sudo cat /var/lib/tor/mysql_service/hostname</li>
  <li>copy the result and paste it into a file called for example my mysql onion address.</li>
</ol>

<h3>How to compile Barter_Facilitator</h3>
<ol>
  <li>Put this in the terminal first then press enter except the stuff that is in parentheses and parentheses: cd directory(directory must be unique to those 2 files and that will be your project folder) with Barter_Facilitator.rs and Cargo.toml file</li>
  <li>Put this in the terminal then press enter except the stuff that is in parentheses and parentheses: cargo build (for fast compilation but not efficent application) or cargo build --release (for efficent application but not fast compilation)</li>
  <li>The executable file will be in your project folder in /target/debug directory</li>
  <li>To open the executable you just drag the executable into your terminal right after the word torsocks and there must be a space between torsocks and directory, and press enter unless you are already in part of the directory in your terminal, For Example: the executable is /etc/executable and you are already in /etc/ then you type in just type in executable, torsocks is still needed.</li>
</ol>

<a href="https://github.com/Daniel-Hanrahan-Tools-and-Games/Barter_Facilitator">Repository Page</a>

<a href="https://daniel-hanrahan-tools-and-games.github.io/">Home Page</a>
