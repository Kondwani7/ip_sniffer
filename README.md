# ip_sniffer
This program assesses how many open ports are running on your current ipaddress

to download rust on wsl ubuntu/linux and macos:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then run:
```bash
rustcup --version and rustc --version
```
to initalize the rust project in  your terminal before adding files run:
```bash
cargo new "folder name" --bin
```
#Run the program
This program portrays how many ports are running on your ipaddress. To run this code concurrently, it uses multithreading

use the following command in terminal to test out the code
```bash
cargo run -- -j "number_of_threads, e.g '1000' " "ipaddress"
```
