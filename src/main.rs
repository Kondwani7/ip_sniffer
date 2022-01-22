use std::env;
use std::io::{self, Write};
use std::net::{IpAddr,TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}
// implementing the ip call with the flags for errs, the ipaddress and number of threads
impl Arguments{
    fn new(args: &[String]) ->Result<Arguments, &'static str> {
        if args.len() < 2 {
            //Err is used to instruct the response to receiving an error
            //similar to the "catch" in a try-catch statment when testing out APIs
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        //ok in rust is a way of demonstrating how a response is success
        //similar to a "try" in the try-catch statment when testing out APIs
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4})
        } else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() ==2{
                println!("Usage: -j to select how many threads you want
                \n\r -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else{
                return Err("invalid syntax")
            }
        }
    }
}
//scanning our ip address to test the number of open ports
fn scan(tx: Sender<u16>, start_port:u16, addr:IpAddr, num_threads:u16){
    //to ensure we don't start from port 0
    let mut port: u16 = start_port + 1;
    loop {
        //matching to potential tcp ports
        //tcp is used for server client tranmissions, e.g postrgres and mysql have tcp ports 
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                //used to get a standard output on the terminal
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            } Err(_) => {}
        }
        //ensure the we don't overload our threads of reqeuests
        if(MAX - port) <= num_threads{
            break;
        }
        port += num_threads;
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                //terminates the entire process
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move ||{
            scan(tx, i , addr, num_threads);
        });
    }

    let mut out = vec![];
    //if the sender goes out of scop
    drop(tx);
    for p in rx{
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v)
    }
}
