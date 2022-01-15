extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate serde_json;

use gtk::prelude::*;
use gtk::{Builder, Window, Label};
use std::process::Command;
use std::collections::HashMap;
use serde_json::Value;

fn main() {
    let ui_src = include_str!("interfaces/main.ui").to_string();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let builder: Builder = Builder::from_string(ui_src.as_str());
    let main_window: Window = builder.get_object("app").unwrap();
    let ygg_version: Label = builder.get_object("ygg_version").unwrap();
    let ygg_public_key: Label = builder.get_object("ygg_public_key").unwrap();
    let ygg_ipv6_address: Label = builder.get_object("ygg_ipv6_address").unwrap();
    let ygg_ipv6_subnet: Label = builder.get_object("ygg_ipv6_subnet").unwrap();

    let yggdrasil_get_self = Command::new("yggdrasilctl").args(["-json", "getSelf"]).output().unwrap().stdout;
    let yggdrasil_get_self_json = serde_json::from_str::<HashMap<String, HashMap<String, Value>>>(&String::from_utf8(yggdrasil_get_self).unwrap()).unwrap();
    for (_, value) in yggdrasil_get_self_json {
        for (key, value) in value {
            ygg_version.set_text(&value["build_version"].as_str().unwrap());
            ygg_public_key.set_text(&value["key"].as_str().unwrap());
            ygg_ipv6_address.set_text(&key);
            ygg_ipv6_subnet.set_text(&value["subnet"].as_str().unwrap());
        }
    }

    let yggdrasil_get_peers = Command::new("yggdrasilctl").args(["-json", "getPeers"]).output().unwrap().stdout;
    let yggdrasil_get_peers_json = serde_json::from_str::<HashMap<String, HashMap<String, Value>>>(&String::from_utf8(yggdrasil_get_peers).unwrap()).unwrap();
    for (_, value) in yggdrasil_get_peers_json {
        for (key, value) in value {
            // ygg_version.set_text(&value["build_version"].as_str().unwrap());
            // ygg_public_key.set_text(&value["key"].as_str().unwrap());
            // ygg_ipv6_address.set_text(&key);
            // ygg_ipv6_subnet.set_text(&value["subnet"].as_str().unwrap());
        }
    }
    // ygg_public_key.set_text("1234");
    
    main_window.resize(400, 450);
    main_window.show();
    gtk::main();
}
