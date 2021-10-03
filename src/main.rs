mod tplink;
use text_io::*;
use crate::tplink::{TpLinkCliService};

fn main(){
    let mut service = TpLinkCliService::new();
    service.get_device_selection();

    run_with_repeat(&mut service)
}

fn run_with_repeat(service: &mut TpLinkCliService){
    service.get_action_selection()
        .execute_action();

    println!("Run again for device? (y/n):");
    let mut repeat: String = read!();

    while repeat.is_empty() {
        println!("Please select y or n:");
        repeat = read!();
    }

    if repeat == "y"{
        run_with_repeat(service);
    }
}

