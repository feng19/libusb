use rusb::{ConfigDescriptor, Context, DeviceHandle, Direction, TransferType, UsbContext};
use rustler::{Atom, Binary, Env, Error, NifResult, NifStruct, ResourceArc, Term};
use std::collections::HashMap;

mod atoms {
    rustler::atoms! {
        ok,
        has_capability,
        has_hid_access,
        has_hotplug,
        supports_detach_kernel_driver,
        rusb_error,
        not_found,
        no_bulk_endpoint
    }
}

pub struct DeviceResource {
    pub device_handle: DeviceHandle<Context>,
    pub in_endpoint: u8,
    pub out_endpoint: u8,
}

pub type Resource = ResourceArc<DeviceResource>;

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(DeviceResource, env);
    true
}

#[derive(NifStruct)]
#[module = "LibUSB.Native.Device"]
struct Device {
    pub bus_number: u8,
    pub address: u8,
    pub port_numbers: Vec<u8>,
    pub device_descriptor: DeviceDescriptor,
}

#[derive(NifStruct)]
#[module = "LibUSB.Native.DeviceDescriptor"]
struct DeviceDescriptor {
    pub vendor_id: u16,
    pub product_id: u16,
    pub class_code: u8,
    pub sub_class_code: u8,
    pub usb_version: (u8, u8, u8),
    pub device_version: (u8, u8, u8),
    pub protocol_code: u8,
    pub max_packet_size: u8,
    pub manufacturer_index: u8,
    pub product_index: u8,
    pub serial_number_index: u8,
    pub num_configurations: u8,
}

#[rustler::nif]
fn info(env: Env) -> HashMap<Term, bool> {
    let mut info = HashMap::new();

    info.insert(atoms::has_capability().to_term(env), rusb::has_capability());
    info.insert(atoms::has_hid_access().to_term(env), rusb::has_hid_access());
    info.insert(atoms::has_hotplug().to_term(env), rusb::has_hotplug());
    info.insert(
        atoms::supports_detach_kernel_driver().to_term(env),
        rusb::supports_detach_kernel_driver(),
    );
    info
}

#[rustler::nif]
fn list_devices() -> Vec<Device> {
    let mut vec = vec![];
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        let device_version = device_desc.device_version();
        let usb_version = device_desc.usb_version();

        let device_descriptor = DeviceDescriptor {
            vendor_id: device_desc.vendor_id(),
            product_id: device_desc.product_id(),
            class_code: device_desc.class_code(),
            sub_class_code: device_desc.sub_class_code(),
            usb_version: (
                usb_version.major(),
                usb_version.minor(),
                usb_version.sub_minor(),
            ),
            device_version: (
                device_version.major(),
                device_version.minor(),
                device_version.sub_minor(),
            ),
            protocol_code: device_desc.protocol_code(),
            max_packet_size: device_desc.max_packet_size(),
            manufacturer_index: device_desc.manufacturer_string_index().unwrap_or(0),
            product_index: device_desc.product_string_index().unwrap_or(0),
            serial_number_index: device_desc.serial_number_string_index().unwrap_or(0),
            num_configurations: device_desc.num_configurations(),
        };

        let device = Device {
            bus_number: device.bus_number(),
            address: device.address(),
            port_numbers: device.port_numbers().unwrap(),
            device_descriptor,
        };

        vec.push(device);
    }
    vec
}

#[rustler::nif]
fn write(
    vendor_id: u16,
    product_id: u16,
    binary: Binary,
    timeout: u64,
) -> Result<(Atom, usize), rustler::Error> {
    match Context::new() {
        Ok(mut context) => match open_device(&mut context, vendor_id, product_id) {
            Some((device, _device_desc, mut device_handle)) => {
                let config_descriptor = to_term_error(device.active_config_descriptor())?;
                let (_in_endpoint, out_endpoint) = detected_endpoint(config_descriptor)?;

                if let Ok(active) = device_handle.kernel_driver_active(0) {
                    if active {
                        // The kernel is active, we have to detach it
                        match device_handle.detach_kernel_driver(0) {
                            Ok(_) => (),
                            Err(e) => return Err(Error::Term(Box::new(e.to_string()))),
                        };
                    }
                };
                // Now we claim the interface
                match device_handle.claim_interface(0) {
                    Ok(_) => (),
                    Err(e) => return Err(Error::Term(Box::new(e.to_string()))),
                }

                match device_handle.write_bulk(
                    out_endpoint,
                    binary.as_slice(),
                    std::time::Duration::from_secs(timeout),
                ) {
                    Ok(s) => Ok((atoms::ok(), s)),
                    Err(e) => Err(Error::Term(Box::new(e.to_string()))),
                }
            }
            None => Err(Error::Term(Box::new(atoms::not_found()))),
        },
        Err(e) => Err(Error::Term(Box::new(e.to_string()))),
    }
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(rusb::Device<T>, rusb::DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    None
}

fn detected_endpoint(config_descriptor: ConfigDescriptor) -> NifResult<(u8, u8)> {
    let mut in_endpoint: Option<u8> = None;
    let mut out_endpoint: Option<u8> = None;

    for interface in config_descriptor.interfaces() {
        for descriptor in interface.descriptors() {
            for endpoint in descriptor.endpoint_descriptors() {
                match (endpoint.transfer_type(), endpoint.direction()) {
                    (TransferType::Bulk, Direction::Out) => out_endpoint = Some(endpoint.number()),
                    (TransferType::Bulk, Direction::In) => in_endpoint = Some(endpoint.number()),
                    _ => {}
                }
            }
        }
    }

    match (in_endpoint, out_endpoint) {
        (None, None) => {
            let reason = atoms::no_bulk_endpoint();
            Err(Error::Term(Box::new(reason)))
        }
        _ => Ok((in_endpoint.unwrap(), out_endpoint.unwrap())),
    }
}

fn to_term_error<T>(res: Result<T, impl ToString>) -> NifResult<T> {
    res.map_err(|e| Error::Term(Box::new(e.to_string())))
}

rustler::init!(
    "Elixir.LibUSB.Native",
    [info, list_devices, write],
    load = load
);
