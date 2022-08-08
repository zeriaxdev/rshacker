use std::fmt::Write;
use std::thread::{self, sleep};
use std::time::Duration;

use colored::Colorize;
use dns_lookup::lookup_host;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::Rng;
use regex::Regex;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use spinach::Spinach;

struct CmdRes {}

impl CmdRes {
    fn success(msg: &str) {
        let sign = "✔ ".green();
        println!("{sign} {}", msg)
    }

    fn error(msg: &str) {
        let sign = "✘ ".red();
        println!("{sign} {}", msg)
    }
}

fn main() -> Result<()> {
    println!("{}", ":: WELCOME ::".blue().bold());

    let mut rl = Editor::<()>::new()?;

    let prompt = format!("({}@{}) >> ", whoami::username(), whoami::hostname());
    loop {
        let readline = rl.readline(&prompt.red().bold().to_string().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if line == "help" {
                    println!("Try \"{}\"ing something!", "hack".blue().bold());
                } else if line == "clear" {
                    println!("{}", "Use ctrl-l to clear".green().bold());
                } else if line == "whoami" {
                    println!("{}", whoami::username());
                } else if line == "exit" {
                    break;
                } else if line == "hack" {
                    CmdRes::success("Enter the domain name you want to hack.");
                    let domain = rl.readline("domain >> ");
                    match domain {
                        Ok(domain) => {
                            let re = Regex::new(
                                r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9](?:\.[a-zA-Z]{2,})+$",
                            )
                            .unwrap();
                            if re.is_match(domain.as_str()) {
                                let tick = format!("[{}]", "+".green());

                                fn hack(msg: &str, dur: u64, ans: &str) {
                                    let tick = format!("[{}]", "+".green());

                                    let s = Spinach::new(msg);
                                    sleep(Duration::from_secs(dur));
                                    s.succeed("Success!");

                                    println!("  {tick} {ans}");
                                }

                                // 1
                                let host = domain.as_str();
                                let lookup = lookup_host(host);

                                if lookup.is_err() {
                                    CmdRes::error("Name or service not known");
                                } else {
                                    let ascii = "          .                                                      .\n        .n                   .                 .                  n.\n  .   .dP                  dP                   9b                 9b.    .\n 4    qXb         .       dX                     Xb       .        dXp     t\ndX.    9Xb      .dXb    __                         __    dXb.     dXP     .Xb\n9XXb._       _.dXXXXb dXXXXbo.                 .odXXXXb dXXXXb._       _.dXXP\n 9XXXXXXXXXXXXXXXXXXXVXXXXXXXXOo.           .oOXXXXXXXXVXXXXXXXXXXXXXXXXXXXP\n  `9XXXXXXXXXXXXXXXXXXXXX'~   ~`OOO8b   d8OOO'~   ~`XXXXXXXXXXXXXXXXXXXXXP'\n    `9XXXXXXXXXXXP' `9XX'   DIE    `98v8P'  HUMAN   `XXP' `9XXXXXXXXXXXP'\n        ~~~~~~~       9X.          .db|db.          .XP       ~~~~~~~\n                        )b.  .dbo.dP'`v'`9b.odb.  .dX(\n                      ,dXXXXXXXXXXXb     dXXXXXXXXXXXb.\n                     dXXXXXXXXXXXP'   .   `9XXXXXXXXXXXb\n                    dXXXXXXXXXXXXb   d|b   dXXXXXXXXXXXXb\n                    9XXb'   `XXXXXb.dX|Xb.dXXXXX'   `dXXP\n                     `'      9XXXXXX(   )XXXXXXP      `'\n                              XXXX X.`v'.X XXXX\n                              XP^X'`b   d'`X^XX\n                              X. 9  `   '  P )X\n                              `b  `       '  d'\n                               `             '\n";

                                    let dur = Duration::from_millis(10);
                                    for s in ascii.split("\n") {
                                        println!("{}", s.red().bold());
                                        sleep(dur)
                                    }

                                    let ips: Vec<std::net::IpAddr> = lookup_host(host).unwrap();

                                    hack(
                                        "Enumerating target...",
                                        1,
                                        &format!("Host: {domain}\n  {tick} IPv4: {}", ips[1]),
                                    );

                                    // 2
                                    hack(
                                        "Opening SOCKS5 ports on infected hosts",
                                        1,
                                        "SSL entry point on 127.0.0.1:1337",
                                    );

                                    // 3
                                    hack(
                                        "Chaining proxies",
                                        1,
                                        "7/7 proxies chained {{BEL>AUS>JAP>CHI>NOR>FIN>UKR}}",
                                    );

                                    // 4
                                    hack(
                                        "Launching port knocking sequence",
                                        1,
                                        "Knock on TCP<143,993,587,456,25,587,993,80>",
                                    );

                                    // 5
                                    hack(
                                        "Sending PCAP datagrams for fragmentation overlap",
                                        1,
                                        "Stack override ***** w00t w00t g0t r00t!",
                                    );

                                    // progress bar
                                    let mut rng = rand::thread_rng();
                                    let num: u64 = rng.gen_range(1000..1800);

                                    let pb = ProgressBar::new(num);
                                    pb.set_style(
                                    ProgressStyle::with_template(
                                        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] ({eta})"
                                )
                                .unwrap()
                                .with_key("eta", |state: &ProgressState, w: &mut dyn Write|
                                            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                                .progress_chars("=>-"));

                                    for _ in 0..1024 {
                                        pb.inc(1);
                                        thread::sleep(Duration::from_millis(1));
                                    }
                                    pb.finish_with_message("Done!");

                                    CmdRes::success("Hack completed!");
                                }
                            } else {
                                CmdRes::error("Hack failed!");
                            }
                        }
                        Err(ReadlineError::Interrupted) => {
                            println!("{}", "\n:: HACK INTERRUPTED ::\n".blue().bold());
                        }
                        Err(ReadlineError::Eof) => {
                            println!("{}", "\n:: HACK INTERRUPTED ::\n".blue().bold());
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                            break;
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => {
                CmdRes::error("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
