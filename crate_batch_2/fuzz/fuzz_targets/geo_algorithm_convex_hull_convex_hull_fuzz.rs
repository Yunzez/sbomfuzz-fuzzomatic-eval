#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use geo::{algorithm::convex_hull::ConvexHull, LineString, coord};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    points: Vec<(f64, f64)>,
}

fuzz_target!(|data: FuzzInput| {
    if !data.points.is_empty() {
        let points: Vec<_> = data.points.iter().map(|&(x, y)| coord! { x: x, y: y }).collect();
        let line_string = LineString(points);
        let _ = line_string.convex_hull();
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: geo 
//  - Crate Link: None 
//  - Crate Version: 0.29.3 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: geo::algorithm::convex_hull 
//  - Use Statement: None 
//  - Function Name: convex_hull 
//  - Function Usage: fn run_3() {
//     let line_string = LineString(
//         vec![
//             coord! { x: -10.0, y: 0.0 },
//             coord! { x: 5.0, y: -10.0 },
//             coord! { x: 10.0, y: 10.0 },
//             coord! { x: -5.0, y: 10.0 }
//         ]
//     );
// 
//     let res = line_string.convex_hull();
//     println!("Convex hull: {:?}", res);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a Self) -> Polygon<<Self as ConvexHull<'a, T>>::Scalar>,
//   type_fields: [Self, geo_types::Polygon] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a Self) -> Polygon<<Self as ConvexHull<'a, T>>::Scalar>,
//   type_fields: [Self, geo_types::Polygon] 
// }
// Struct construction metadata: {
//   type_name: geo_types::Polygon,
//   type_fields: [geo_types::LineString, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: geo_types::LineString,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [flif::Metadata, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: flif::Metadata,
//   type_fields: [flif::ChunkType, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: flif::ChunkType,
//   type_fields: [Iccp, Exif, Exmp, Unknown([u8; {const}])] 
// }

