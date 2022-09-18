use chrono::Local;
use core::time;
use log::{debug, info};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use winput::message_loop;
use winput::{Action, Vk};

pub mod data;

pub fn run(args: &data::Args) {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = std::sync::mpsc::channel();

    let key = unsafe { Vk::from_u8(args.autowalk_trigger_key) };
    
    debug!("Trigger key: {:?}",key);

    let millis = args.trigger_interval_millis.clone();

    thread::spawn(move || {
        debug!("Key press detector enabled");

        let receiver = message_loop::start().unwrap();

        let mut timestamp2 = Local::now();
        let mut enabled = false;

        loop {
            match receiver.next_event() {
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Press,
                    ..
                } => {
                    if vk == key {
                        debug!("{:?} pressed", key);

                        let timestamp1 = timestamp2;
                        timestamp2 = Local::now();

                        if !enabled
                            && (timestamp2.timestamp_millis() - timestamp1.timestamp_millis()) < millis
                        {
                            enabled = !enabled;
                            info!("Setting autowalk to {}", enabled);
                            tx.send(enabled).unwrap();
                        } else if enabled {
                            enabled = !enabled;
                            info!("Setting autowalk to {}", enabled);
                            tx.send(enabled).unwrap();
                        }
                    }
                }
                _ => (),
            }
        }
    });

    debug!("Key presser enabled");
    loop {
        match rx.try_recv() {
            Ok(mut enabled) => {
                if enabled {
                    while enabled {
                        debug!("autowalking...");
                        winput::press(Vk::W);
                        match rx.try_recv() {
                            Ok(second_enabled) => {
                                enabled = second_enabled;
                                winput::release(Vk::W);
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
                        }

                        thread::sleep(time::Duration::from_millis(10));
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        thread::sleep(time::Duration::from_millis(10));
    }
}
