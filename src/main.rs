mod tplink;
use text_io::*;
use crate::tplink::{TpLinkCliService};

fn main(){
    let mut service = TpLinkCliService::new();
    service
        .get_device_selection()
        .get_action_selection()
        .execute_action();

    println!("Run again? (y/n):");
    let mut repeat: String = read!();

    while repeat.is_empty() {
        println!("Please select y or n:");
        repeat = read!();
    }

    if repeat == "y"{
        main();
    }
}

