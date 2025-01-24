//! Discover Bluetooth devices and list them.

use anyhow::Result;
use bluer::{
    Adapter, AdapterEvent, Address, Device, DeviceEvent, DeviceProperty, DiscoveryFilter,
    DiscoveryTransport,
};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use std::{
    collections::HashSet,
    env,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use super::blueman::BMEvent;

#[derive(Debug)]
pub struct BTDevice(Device);

impl BTDevice {
    pub fn new(device: Device) -> Self {
        BTDevice(device)
    }
}

// async fn query_device(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
//     let device = adapter.device(addr)?;
//     println!("    Address type:       {}", device.address_type().await?);
//     println!("    Name:               {:?}", device.name().await?);
//     println!("    Icon:               {:?}", device.icon().await?);
//     println!("    Class:              {:?}", device.class().await?);
//     println!(
//         "    UUIDs:              {:?}",
//         device.uuids().await?.unwrap_or_default()
//     );
//     println!("    Paired:             {:?}", device.is_paired().await?);
//     println!("    Connected:          {:?}", device.is_connected().await?);
//     println!("    Trusted:            {:?}", device.is_trusted().await?);
//     println!("    Modalias:           {:?}", device.modalias().await?);
//     println!("    RSSI:               {:?}", device.rssi().await?);
//     println!("    TX power:           {:?}", device.tx_power().await?);
//     println!(
//         "    Manufacturer data:  {:?}",
//         device.manufacturer_data().await?
//     );
//     println!("    Service data:       {:?}", device.service_data().await?);
//     Ok(())
// }
//
// async fn query_all_device_properties(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
//     let device = adapter.device(addr)?;
//     let props = device.all_properties().await?;
//     for prop in props {
//         println!("    {:?}", &prop);
//     }
//     Ok(())
// }

pub async fn launch_bluetooth_listener(
    event_send_chan: Arc<Sender<BMEvent>>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        let with_changes = env::args().any(|arg| arg == "--changes");
        let all_properties = env::args().any(|arg| arg == "--all-properties");
        let le_only = env::args().any(|arg| arg == "--le");
        let br_edr_only = env::args().any(|arg| arg == "--bredr");
        let filter_addr: HashSet<_> = env::args()
            .filter_map(|arg| arg.parse::<Address>().ok())
            .collect();
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;

        adapter.set_powered(true).await?;

        let filter = DiscoveryFilter {
            transport: if le_only {
                DiscoveryTransport::Le
            } else if br_edr_only {
                DiscoveryTransport::BrEdr
            } else {
                DiscoveryTransport::Auto
            },
            ..Default::default()
        };

        adapter.set_discovery_filter(filter).await?;

        let device_events = adapter.discover_devices().await?;

        pin_mut!(device_events);

        // let mut all_change_events = SelectAll::new();

        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                                continue;
                            }

                            let device = adapter.device(addr).unwrap();

                            event_send_chan.send(BMEvent::DeviceAdded(BTDevice::new(device))).await.unwrap();
                        },
                        AdapterEvent::DeviceRemoved(addr) => {
                            let device = adapter.device(addr).unwrap();
                            event_send_chan.send(BMEvent::DeviceRemoved(BTDevice::new(device))).await.unwrap();
                        }
                        _ => {},
                    }
                }
                // Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                //     println!("Device changed: {addr}");
                //     println!("    {property:?}");
                // }
                else => break
            }
        }

        Ok(())
    })
}

impl PartialEq for BTDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0.address() == other.0.address()
    }
}

impl Eq for BTDevice {}

impl Hash for BTDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.address().hash(state)
    }
}
