#![cfg(any(target_arch = "wasm32", doc))]

mod interface;
mod sim_connect;

use msfs::MSFSEvent;
use crate::sim_connect::TcasAircraftData;

use msfs::sim_connect::SIMCONNECT_OBJECT_ID_USER;

#[msfs::gauge(name = TCAS)]
async fn tcas(mut gauge: msfs::Gauge) -> Result<(), Box<dyn std::error::Error>> {
    let request_id: msfs::sys::SIMCONNECT_DATA_REQUEST_ID = 1;
    let mut iface = interface::Interface::new(guage.open_simconnect("FBW-TCAS")?);
    // request all aircraft in a range of 444481 meters (about 240 nm)
    iface.request_detects(request_id, 444481).unwrap();
    while let Some(event) = gauge.next_event().await {
        if let msfs::MSFSEvent::SimConnect(event) = event {
            match event {
                msfs::sim_connect::SimConnectRecv::SimObjectData(event) => {
                    if event.dwRequestID == request_id {
                        let detect = event.into::<TcasAircraftData>(&sim).unwrap();
                        // ignore ghost aircraft at 0,0 with 0 altitude and ignore user aircraft for tcas purposes
                        if (detect.latitude != 0 || detect.longitude != 0 || detect.altitude != 0) && event.dwObjectId != SIMCONNECT_OBJECT_ID_USER {
                            iface.handle_detect(event.dwObjectID, detect)
                        }
                        if event.dwentrynumber == event.dwoutof {
                            // got all aircraft for this request. TODO: Do something with that here
                            // for now, debug print the number of detects and clear for another request
                            println!("TCAS Detects: {}", iface.detects.len());
                            iface.detects.clear();
                            iface.request_detects(request_id, 444481).unwrap();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}