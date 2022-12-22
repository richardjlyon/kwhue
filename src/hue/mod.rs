use crate::error::AppError;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

const SERVICE_NAME: &'static str = "_hue._tcp.local";

/// Get the ip address of the Hue bridge
pub async fn bridge_ipaddr() -> Result<IpAddr, AppError> {
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
        .map_err(|_| AppError::Other)
        .unwrap()
        .listen();
    pin_mut!(stream);

    match stream.next().await {
        Some(Ok(response)) => {
            let addr = response.records().filter_map(self::to_ip_addr).next();
            if let Some(addr) = addr {
                return Ok(addr);
            } else {
                Err(AppError::HueBridgeAddressNotFoundError)
            }
        }
        Some(Err(_)) => Err(AppError::Other),
        None => Err(AppError::HueBridgeNotFoundError),
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
