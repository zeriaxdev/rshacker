use std::env;
use std::fmt::Write;
use std::thread::{self, sleep};
use std::time::Duration;

use colored::Colorize;
use dns_lookup::lookup_host;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::Rng;
use regex::Regex;
use spinach::Spinach;

struct CmdRes {}

impl CmdRes {
    fn success(msg: &str) {
        let sign = "✔".green();
        println!("{sign} {}", msg)
    }

    fn error(msg: &str) {
        let sign = "✘".red();
        println!("{sign} {}", msg)
    }
}

fn main() {
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9](?:\.[a-zA-Z]{2,})+$").unwrap();

    let args: Vec<String> = env::args().collect();
    let domain: &str;

    if args.len() == 1 {
        CmdRes::error("Please use the script along with a valid website as an argument.");
        CmdRes::success(&format!("{} google.com", "rshacker".green().bold()));

        return;
    } else {
        domain = &args[1].as_str();
    }

    if re.is_match(domain) {
        let tick = format!("[{}]", "+".green());

        fn hack(msg: &str, dur: u64, ans: &str) {
            let tick = format!("[{}]", "+".green());

            let s = Spinach::new(msg);
            sleep(Duration::from_secs(dur));
            s.succeed(msg);

            println!("  {tick} {ans}");
        }

        let host = domain;
        let lookup = lookup_host(host);

        if lookup.is_err() {
            CmdRes::error("Name or service not known");
        } else {
            let ascii = "          .                                                      .\n        .n                   .                 .                  n.\n  .   .dP                  dP                   9b                 9b.    .\n 4    qXb         .       dX                     Xb       .        dXp     t\ndX.    9Xb      .dXb    __                         __    dXb.     dXP     .Xb\n9XXb._       _.dXXXXb dXXXXbo.                 .odXXXXb dXXXXb._       _.dXXP\n 9XXXXXXXXXXXXXXXXXXXVXXXXXXXXOo.           .oOXXXXXXXXVXXXXXXXXXXXXXXXXXXXP\n  `9XXXXXXXXXXXXXXXXXXXXX'~   ~`OOO8b   d8OOO'~   ~`XXXXXXXXXXXXXXXXXXXXXP'\n    `9XXXXXXXXXXXP' `9XX'   DIE    `98v8P'  HUMAN   `XXP' `9XXXXXXXXXXXP'\n        ~~~~~~~       9X.          .db|db.          .XP       ~~~~~~~\n                        )b.  .dbo.dP'`v'`9b.odb.  .dX(\n                      ,dXXXXXXXXXXXb     dXXXXXXXXXXXb.\n                     dXXXXXXXXXXXP'   .   `9XXXXXXXXXXXb\n                    dXXXXXXXXXXXXb   d|b   dXXXXXXXXXXXXb\n                    9XXb'   `XXXXXb.dX|Xb.dXXXXX'   `dXXP\n                     `'      9XXXXXX(   )XXXXXXP      `'\n                              XXXX X.`v'.X XXXX\n                              XP^X'`b   d'`X^XX\n                              X. 9  `   '  P )X\n                              `b  `       '  d'\n                               `             '\n";

            let dur = Duration::from_millis(50);
            for s in ascii.split("\n") {
                println!("{}", s.red().bold());
                sleep(dur)
            }

            let ips: Vec<std::net::IpAddr> = lookup_host(host).unwrap();
            let ipv4 = ips[ips.len() - 1];

            hack(
                "Enumerating target...",
                1,
                &format!("Host: {domain}\n  {tick} IPv4: {ipv4}"),
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
                r#"7/7 proxies chained {BEL>AUS>JAP>CHI>NOR>FIN>UKR}"#,
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

            // 6
            hack("Installing the virus...", 5, "Successfully installed.");

            // 7
            CmdRes::success("Injecting the virus...");

            // progress bar
            let mut rng = rand::thread_rng();
            let num: u64 = rng.gen_range(1024..2048);

            let pb = ProgressBar::new(num);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] ({eta})",
                )
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                })
                .progress_chars("=> "),
            );

            for _ in 0..1024 {
                pb.inc(1);
                thread::sleep(Duration::from_millis(5));
            }
            pb.finish_with_message("Done!");

            CmdRes::success("Hack completed!");
        }
    } else {
        CmdRes::error("Hack failed!");
    }
}
