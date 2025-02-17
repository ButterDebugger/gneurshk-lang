# Syntax Example

```go
// Automatic type inference

var assumed_integer = 2 // i32 is the default integer type
var assumed_float = 3.0 // f32 is the default float type

// Explicit type declaration

var explicit_integer: u8 = 5
var explicit_float: f16 = 6.0

// Functions

func square(x: f32) -> f32:
    return x * x

// Structs

struct Point:
    x: f32
    y: f32

    func __init__(x: f32, y: f32):
        this.x = x
        this.y = y

    func add(other: Point) -> Point:
        return Point(this.x + other.x, this.y + other.y)

```
