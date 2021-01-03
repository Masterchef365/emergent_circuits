pub type Point = (i32, i32);
pub type Component = Vec<Point>;
pub type Destination = (usize, usize);
pub type Connection = (Destination, Destination);
pub type Placement = Point;
pub type Route = Vec<Point>;
pub type AABB = (Point, Point);
pub type Circuit = (Vec<Component>, Vec<Connection>, AABB);
pub type Layout = (Vec<Placement>, Vec<Route>);

/// Contract:
/// Returns (Placements, Routes) where:
/// Placements are the locations of the `components`
/// Routes are the actual connections between components with their placements
/// Returns None if no solution can be found 
pub fn layout(circuit: &Circuit) -> Option<Layout> {
    todo!()
}
