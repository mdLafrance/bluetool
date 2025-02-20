//! Discover Bluetooth devices and list them.

use anyhow::Result;
use bluer::{
    AdapterEvent, Address, Device, DeviceEvent, DiscoveryFilter, DiscoveryTransport, Uuid,
};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use std::{
    cmp::Ordering,
    collections::HashSet,
    env,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use super::bluetool::AppEvent;

#[derive(Debug, Clone)]
pub struct BTDevice {
    pub inner: Device,
    pub name: String,
    pub icon_name: String,
    pub paired: bool,
    pub connected: bool,
    pub address: String,
    pub battery: Option<u8>,
    pub rssi: Option<i16>,
}

impl BTDevice {
    pub async fn new(device: &Device) -> Self {
        BTDevice {
            inner: device.clone(),
            name: device
                .name()
                .await
                .unwrap_or(None)
                .unwrap_or("???".to_string()),
            icon_name: device
                .icon()
                .await
                .unwrap_or(None)
                .unwrap_or("".to_string()),
            address: device
                .address()
                .0
                .to_vec()
                .iter()
                .map(|u| format!("{:02x}", u))
                .collect::<Vec<String>>()
                .join(":"),

            paired: device.is_paired().await.unwrap_or(false),
            connected: device.is_connected().await.unwrap_or(false),
            battery: check_battery_service(device).await,
            rssi: device.rssi().await.unwrap_or(None),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        Ok(self.inner.connect().await?)
    }

    pub async fn pair(&self) -> Result<()> {
        Ok(self.inner.pair().await?)
    }

    pub async fn disconnect(&self) -> Result<()> {
        Ok(self.inner.disconnect().await?)
    }

    pub async fn remove(&self) -> Result<()> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        Ok(adapter.remove_device(self.inner.address()).await?)
    }

    fn sort_value(&self) -> (i32, &str) {
        (
            self.connected as i32 * 2000 + self.paired as i32 * 1000,
            &self.name,
        )
    }
}

async fn check_battery_service(device: &Device) -> Option<u8> {
    // Fuck this is nasty though
    if let Ok(services) = device.services().await {
        for service in services {
            if service.uuid().await.unwrap() == Uuid::from_u128(0x180F) {
                if let Ok(characteristics) = service.characteristics().await {
                    for characteristic in characteristics {
                        if characteristic.uuid().await.unwrap() == Uuid::from_u128(0x2A19) {
                            if let Ok(value) = characteristic.read().await {
                                return Some(value[0]);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub async fn launch_bluetooth_listener(
    event_send_chan: Arc<Sender<AppEvent>>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        // let with_changes = env::args().any(|arg| arg == "--changes");
        // let all_properties = env::args().any(|arg| arg == "--all-properties");
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

        let mut all_change_events = SelectAll::new();

        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                                continue;
                            }

                            let device = adapter.device(addr).unwrap();

                            event_send_chan.send(AppEvent::DeviceAdded(BTDevice::new(&device).await)).await.unwrap();

                            let change_events = device.events().await?.map(move |evt| (addr, evt));
                            all_change_events.push(change_events);
                        },
                        AdapterEvent::DeviceRemoved(addr) => {
                            let device = adapter.device(addr).unwrap();
                            event_send_chan.send(AppEvent::DeviceRemoved(BTDevice::new(&device).await)).await.unwrap();
                        }
                        _ => {},
                    }
                }
                Some((addr, DeviceEvent::PropertyChanged(_))) = all_change_events.next() => {
                    let device = adapter.device(addr).unwrap();
                    event_send_chan.send(AppEvent::DeviceModified(BTDevice::new(&device).await)).await.unwrap();

                }
                else => break
            }
        }

        Ok(())
    })
}

impl Eq for BTDevice {}

impl Hash for BTDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state)
    }
}

impl Ord for BTDevice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.sort_value();
        let b = other.sort_value();

        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for BTDevice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BTDevice {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
