use tplinker::devices::Device;
use tplinker::discover;
use tplinker::capabilities::{DeviceActions, Switch, Dimmer};
use text_io::*;
use tplinker::datatypes::{GetLightStateResult, SetLightState};
use serde_json::json;
use tplinker::discovery::with_timeout;
use std::time::Duration;

pub struct TpLinkCliService{
    devices: Vec<Device>,
    selected_device: Option<Device>,
    selected_action: Option<Actions>
}

impl TpLinkCliService{
    pub fn new() -> TpLinkCliService {
        TpLinkCliService{
            devices: Vec::new(),
            selected_device: None,
            selected_action: None
        }
    }

    pub fn get_device_selection(&mut self) -> &mut TpLinkCliService{
        self.get_devices();

        println!("Select a device...");
        for i in 0..self.devices.len() {
            let device = self.devices.get(i).unwrap();
            let alias: String = device.alias().unwrap();
            println!("{}) \t{}", i + 1, alias);
        }
        println!();

        let mut selection: usize = read!();
        while selection == 0 {
            println!("You entered an empty selection, please try again...");
            println!();
            selection = read!();
        };

        self.selected_device = Option::from(self.devices.get(selection - 1).unwrap().clone());

        println!("You selected {}", self.selected_device.as_ref().unwrap().alias().unwrap());

        return self;
    }

    pub fn get_action_selection(&mut self) -> &mut TpLinkCliService{
        let actions = [Actions::Toggle, Actions::Brightness, Actions::Colour];

        println!("What would you like to do:");
        for action_index in 0..actions.len(){
            let action = actions.get(action_index).unwrap();
            match action{
                Actions::Toggle =>  println!("{}) {}", action_index+1, "Toggle on / off."),
                Actions::Brightness =>  println!("{}) {}", action_index+1, "Set the brightness %."),
                Actions::Colour =>  println!("{}) {}", action_index+1, "Set the colour.")
            }
        }

        let mut action_selection: usize = read!();
        while action_selection == 0 {
            println!("You entered an empty selection, please try again...");
            println!();
            action_selection = read!();
        };

        self.selected_action = Option::from(actions.get(action_selection - 1).unwrap().clone());

        println!("Selected action is {}.", match self.selected_action.unwrap() {
            Actions::Toggle => "toggle on / off",
            Actions::Brightness => "set brightness",
            Actions::Colour => "set colour"
        });

        return self;
    }

    pub fn execute_action(&mut self){
        match self.selected_action.unwrap() {
            Actions::Toggle => DeviceExtensions::toggle_power(self.selected_device.as_ref().unwrap()),
            Actions::Brightness => {
                println!("Desired brightness %:");
                let mut brightness: u16 = read!();
                while brightness == 0{
                    println!("You entered an empty brightness, please try again...");
                    brightness = read!();
                }
                DeviceExtensions::set_brightness(self.selected_device.as_ref().unwrap(), brightness)
            },
            Actions::Colour => {
                println!("Desired colour (hue,saturation,temperature):");
                let mut colour: String = read!();
                while colour.is_empty() {
                    println!("You entered an empty colour, please try again...");
                    colour = read!();
                }
                DeviceExtensions::set_colour(self.selected_device.as_ref().unwrap(), colour)
            }
        }
    }

    fn get_devices(&mut self){
        let discovery = match with_timeout(Some(Duration::from_secs(3))){
            Ok(v) => v,
            Err(e) => panic!("Error")
        };

        for (addr, data) in discovery { &self.devices.push(Device::from_data(addr, &data)); }
    }
}

#[derive(Copy, Clone)]
enum Actions{
    Toggle,
    Brightness,
    Colour
}

pub struct DeviceExtensions {}
impl DeviceExtensions {
    pub fn toggle_power(device: &Device) {
        match device {
            Device::LB110(device) => {
                if device.is_on().unwrap() {
                    device.switch_off();
                    return;
                }
                if device.is_off().unwrap() {
                    device.switch_on();
                    return;
                }
            },
            Device::HS110(device) => {
                if device.is_on().unwrap() {
                    device.switch_off();
                    return;
                }
                if device.is_off().unwrap() {
                    device.switch_on();
                    return;
                }
            },
            Device::HS105(device) => {
                if device.is_on().unwrap() {
                    device.switch_off();
                    return;
                }
                if device.is_off().unwrap() {
                    device.switch_on();
                    return;
                }
            },
            Device::HS100(device) => {
                if device.is_on().unwrap() {
                    device.switch_off();
                    return;
                }
                if device.is_off().unwrap() {
                    device.switch_on();
                    return;
                }
            },
            Device::Unknown(device) => {
                let command = json!({
                "smartlife.iot.smartbulb.lightingservice": {
                    "get_light_state": null
                }
            }).to_string();
                let data: GetLightStateResult = device.send(&command).unwrap();

                if data.light_state().unwrap().on_off == 1 {
                    device.send::<serde_json::Value>(&r#"{"smartlife.iot.smartbulb.lightingservice":{"transition_light_state":{"on_off":0}}}"#);
                } else {
                    device.send::<serde_json::Value>(&r#"{"smartlife.iot.smartbulb.lightingservice":{"transition_light_state":{"on_off":1}}}"#);
                }
            }
            _ => ()
        }
    }
    pub fn set_brightness(device:& Device, brightness: u16){
        match device {
            Device::LB110(device) => { device.set_brightness(brightness).unwrap() },
            Device::Unknown(device) => {
                let light_state = SetLightState {
                    on_off: None,
                    hue: None,
                    saturation: None,
                    brightness: Some(brightness),
                    color_temp: None,
                };

                let command = json!({
                "smartlife.iot.smartbulb.lightingservice": {
                    "transition_light_state": light_state,
                }
            }).to_string();
                device.send::<serde_json::Value>(&command).unwrap();
            }
            _ => ()
        }
    }
    pub fn set_colour(device: &Device, colour: String){
        let values: Vec<&str> = colour.split(",").collect();
        match device {
            Device::Unknown(device) => {
                let light_state = SetLightState {
                    on_off: None,
                    hue: Some(values.get(0).unwrap().to_string().parse::<u16>().unwrap()),
                    saturation: Some(values.get(1).unwrap().to_string().parse::<u16>().unwrap()),
                    brightness: None,
                    color_temp: Some(values.get(2).unwrap().to_string().parse::<u16>().unwrap()),
                };

                let command = json!({
                "smartlife.iot.smartbulb.lightingservice": {
                    "transition_light_state": light_state,
                }
            }).to_string();
                device.send::<serde_json::Value>(&command).unwrap();
            }
            _ => ()
        }
    }
}