First assign connections - From open pin range to open pin range on another chip.
Then figure out routing - Djikstra's algo?
Occupancy cache - queries for if a given cell is occupied on the sparse grid

... This is gonna end up with you making an actual circuit designing algorithm, isn't it ...
*sigh*
Let's just make the visualizer first, eh?
,,, well we can't until you have something to feed it. Let's work on data organization


# Data organization
What data do we have? Well we have a few different kinds:

Chips:
* A collection of terminals
* `Vec<Point>` - These are relative

Connections:
* Each connection connects a terminal from one chip to another terminal.
* `Vec<(usize, usize)>` - (chip idx, terminal idx)

Placement:
* Placements of individual chips in 2D space
* `HashMap<usize, Point>` - (chips idx, position)

Routes:
* Positional data associated with a connection
* ```HashMap<usize, Vec<Point>>``` - (connection idx, wire positions)


fn(Chips, Connections -> Placement), Routes
fn(Chips, Placement, Routes) -> Mesh

# Optimization
* We're optimizing for the shortest wiring that connects all of our components
* Isn't there a board size constraint too?

## Rules
* Bounding boxes of chips must not overlap
* Routes must not cross
* Routs and chips must stay inside of the bounds

Oh god, imagine if we factored in timing and shit
that would be
yikes

Let's make this a playground, then. A visualization tool, at best for now. Naive solvers!
