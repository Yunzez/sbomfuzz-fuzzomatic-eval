#![no_main]
extern crate libfuzzer_sys;
extern crate geo;

use libfuzzer_sys::fuzz_target;
use geo::algorithm::convex_hull::ConvexHull;
use geo::{Coordinate, LineString};
use arbitrary::{Arbitrary, Unstructured};
use geo::coord;

// Define a struct that represents a random LineString for fuzzing.
#[derive(Debug)]
struct RandomLineString(Vec<Coordinate<f64>>);

// Define a new type that wraps Coordinate<f64>
#[derive(Debug)]
struct FuzzCoordinate(Coordinate<f64>);

impl<'a> Arbitrary<'a> for FuzzCoordinate {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let x = u.arbitrary()?;
        let y = u.arbitrary()?;
        Ok(FuzzCoordinate(Coordinate { x, y }))
    }
}

impl<'a> Arbitrary<'a> for RandomLineString {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // Create an arbitrary length vector of FuzzCoordinate
        let coords: Vec<FuzzCoordinate> = u.arbitrary()?;
        // Convert FuzzCoordinate to Coordinate<f64>
        let coords: Vec<Coordinate<f64>> = coords.into_iter().map(|fc| fc.0).collect();
        Ok(RandomLineString(coords))
    }
}

fuzz_target!(|data: RandomLineString| {
    // Create a LineString from the random coordinates
    let line_string = LineString(data.0);
    // Execute the function that we want to test
    let res = line_string.convex_hull();
    println!("Convex hull: {:?}", res);
});