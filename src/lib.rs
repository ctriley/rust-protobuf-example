pub mod location {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    include!(concat!(env!("OUT_DIR"), "/location.rs"));
}

fn example_function() {
    let point = location::GpsCoordinate { lat: 50, lon: 39 };
}