
use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use message::CliMessage;
use cli;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") { cfg.verbose = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let id = m.value_of("id").unwrap();
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("actions", Some(m))      => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).actions().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ImageActions(id).display();
            match domgr.image(id).actions().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    for act in s.iter() {
                        CliMessage::Action.display();
                        println!("\t{}\n", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if s.is_empty() { println!("\tNo actions to dipslay"); }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("rename", Some(m))            => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() { return }
            }
            let name = m.value_of("name").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                          .update(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).update(name).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::UpdateImage(id, name).display();
            match domgr.image(id).update(name).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("delete", Some(m))            => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() { return }
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                          .delete()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).delete().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::DeleteImage(id).display();
            match domgr.image(id).delete().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("transfer", Some(m))          => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() { return }
            }
            let reg = m.value_of("region").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                         .transfer(reg)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).transfer(reg).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::TransferImage(id, reg).display();
            match domgr.image(id).transfer(reg).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("convert", Some(m))           => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                          .convert()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).convert().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ConvertImage(id).display();
            match domgr.image(id).convert().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("action", Some(m))       => {
            let a_id = m.value_of("action_id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.image(id)
                         .action(a_id)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.image(id).action(a_id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ImageAction(id, a_id).display();
            match domgr.image(id).action(a_id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        _                              => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.image(id)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.verbose {
                CliMessage::JsonResponse.display();
                match domgr.image(id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::Image(id).display();
            match domgr.image(id).retrieve() {
                Ok(img) => {
                    CliMessage::Success.display();
                    println!("\t{}\n", &img.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
    }
}
