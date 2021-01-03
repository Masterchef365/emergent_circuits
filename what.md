Grid-based circuit design animation
Lay out a grid of non-overlapping and intentionally spaced chips and components with pins
Use a (naive?) path finding algorithm to connect sequentual pins of chips to other chips
* Connect adjacent chips first - clustering algo? No, something simpler...

Components are defined by their pins; the bounding box for their pins must not be occipied by another component

Different chip types:
IC: Pins on all sides, generally either a square or a long rectangle
Simple: 14-pin chips, or just two connected together
