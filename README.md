# Barter_Facilitator
An application that facilitates barter over TOR and it figures out the value of products.

the mysql version I use is: 8.0.42-0

I have no intention of adding addon mod support to this application because I want every possible thing for this app to be in the base app.

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
  <li>Add this to torrc file at the very bottom of config file, the HiddenServiceDir and HiddenServicePort parts are 2 different lines: HiddenServiceDir /var/lib/tor/mysql_service/
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




GNU GPL v3.0 Conditional Exceptions to use MPL 2.0

If the following condition is met, the licensing rules for content covered by GNU GPL v3.0 are modified as described below:

Condition:

The developer is distributing, porting, or integrating the software with platforms or environments that impose requirements incompatible with GPL-3.0, including but not limited to:
- proprietary or non-redistributable SDKs
- confidential hardware or platform documentation
- legally required confidentiality obligations preventing full GPL redistribution
- safety-regulated or certified systems where full GPL redistribution cannot be satisfied

Effect on licensing:

- Content covered by GNU GPL v3.0: May instead be used under the Mozilla Public License 2.0.

These exceptions apply **only when the condition above is met**.





CC BY-SA 4.0 and GNU GPL v3.0 Conditional Exceptions to use PolyForm Noncommercial and CC BY-NC 4.0

The PolyForm Noncommercial License (and Creative Commons
Attribution-NonCommercial 4.0 International for non-code
content) may be used as an alternative only when the combined
work is subject to binding legal, contractual, or platform-
imposed restrictions that prohibit commercial use.

Such restrictions may arise from third-party licenses,
distribution platforms, or other enforceable legal terms that
make commercial use of the combined work not legally permitted.

Content covered by the primary license (e.g., source code or
other covered material) remains governed by that license.

Content not covered by the primary license (e.g., assets,
documentation, or other non-code materials) is governed by
CC BY-NC 4.0, unless otherwise stated.

This alternative applies only to the extent necessary to
comply with such restrictions.




CC BY-SA 4.0 and GNU GPL v3.0 Conditional Exceptions to use PolyForm Strict and CC BY-NC-ND 4.0

The PolyForm Strict License may be used as an alternative
license only when the combined work is subject to binding
legal, contractual, or platform-imposed restrictions that
require both non-commercial use and prohibit the creation of
derivative works as part of the distribution terms.

Such restrictions may arise from third-party licenses,
distribution platforms, or other enforceable legal terms that
impose both non-commercial and no-derivatives requirements on
the combined work.

Content covered by the primary license (e.g., source code or
other covered material) remains governed by that license.

Content not covered by the primary license (e.g., assets,
documentation, or other non-code materials) is governed by
Creative Commons Attribution-NonCommercial-NoDerivatives
4.0 International (CC BY-NC-ND 4.0), unless otherwise stated.

This alternative applies only to the extent necessary to
comply with such restrictions.





Contributors Needed
