# Syntax Example

This is a list of every syntax feature in this language. Keep in mind that this language is still in development and things are subject to change. Suggestions are always welcome and appreciated :)

## Comments

```julia
# This is a comment
```

## Variable Declaration

```julia
# Implicit type declaration

var assumed_integer = 2 # i32 is the default integer type
var assumed_float = 3.0 # f32 is the default float type

# Explicit type declaration

var explicit_integer: u8 = 5
var explicit_float: f16 = 6.0
```

## Strings

```julia
# Single-line strings

var message = "Hello, world!"

# Multi-line strings
# Behaves similar to Python's triple quotes

var letter =
    """
    Dear world,
        Hello!
    """

# Output from the line of code above:
# Using ^ to mark the beginning of a line and · to mark leading spaces
#
# ^Dear world,
# ^····Hello!
#
```

## Functions

```julia
func square(x: f32) -> f32:
    return x * x
```

## Structs

```julia
struct Point:
    x: f32
    y: f32

    func __init__(x: f32, y: f32):
        this.x = x
        this.y = y

    func add(other: Point) -> Point:
        return Point(this.x + other.x, this.y + other.y)
```
