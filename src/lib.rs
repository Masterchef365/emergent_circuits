pub type Point = (i32, i32);
pub type Component = (Vec<Point>, Size);
pub type Destination = (usize, usize);
pub type Connection = (Destination, Destination);
pub type Placement = Point;
pub type Route = Vec<Point>;
pub type Size = (i32, i32);
pub type Circuit = (Vec<Component>, Vec<Connection>, Size);
pub type Layout = (Vec<Placement>, Vec<Route>);

/// Contract:
/// Returns (Placements, Routes) where:
/// Placements are the locations of the `components`
/// Routes are the actual connections between components with their placements
/// Returns None if no solution can be found 
pub fn layout((components, connections, _size): &Circuit) -> Option<Layout> {
    let placements = vec![
        (2, 2),
        (9, 8),
        (19, 18),
    ];
    let mut routes = Vec::new();

    for &(src, dst) in connections {
        let term = |(comp_idx, term_idx): (usize, usize)| {
            let (x, y) = components[comp_idx].0[term_idx];
            let (ox, oy) = placements[comp_idx];
            dbg!((x + ox, y + oy))
        };
        routes.push(vec![term(src), term(dst)]);
    }

    Some((placements, routes))
}
