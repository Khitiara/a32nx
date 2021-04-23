#[msfs::sim_connect::data_definition]
#[derive(Debug)]
pub(crate) struct TcasAircraftData {
    /*
    #[name = "TITLE"]
    #[unit = ""]
    pub(crate) title: [::std::os::raw::c_char; 256usize],
     */
    #[name = "PLANE LATITUDE"]
    #[unit = "DEGREES"]
    pub(crate) latitude: f64,
    #[name = "PLANE LONGITUDE"]
    #[unit = "DEGREES"]
    pub(crate) longitude: f64,
    #[name = "PLANE ALTITUDE"]
    #[unit = "FEET"]
    pub(crate) altitude: f64,
}