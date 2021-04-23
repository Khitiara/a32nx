use msfs::sim_connect::SimConnect;
use std::pin::Pin;
use crate::sim_connect::TcasAircraftData;
use msfs::sys::DWORD;
use msfs::sim_connect::Result;
use uom::si::{f64::*, length::meter};

#[derive(Debug)]
pub(crate) struct Interface<'a> {
    pub(crate) sim: Pin<Box<SimConnect<'a>>>,
    pub(crate) detects: Hashmap<DWORD, TcasAircraftData>,
}

impl<'a> Interface<'a> {
    pub(crate) fn new(sim: Pin<Box<SimConnect<'a>>>) -> Self {
        Interface { sim, detects: vec![] }
    }
    pub(crate) fn handle_detect(&mut self, id: DWORD, detect: &TcasAircraftData) {
        detects.insert(id, detect);
    }
    pub(crate) fn request_detects(&mut self, request: msfs::sys::SIMCONNECT_DATA_REQUEST_ID, radius: Length) -> Result<()> {
        self.sim.request_data_on_sim_object_type(request, radius.get::<meter>().ceil() as DWORD, msfs::sys::SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_AIRCRAFT)
    }
}