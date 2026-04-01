# Syntax Example

This is a list of every syntax feature in this language. Keep in mind that this language is still in development and things are subject to change. Suggestions are always welcome and appreciated :)

## Comments

```julia
# This is a comment
```

## Variable Declaration

```julia
# Uninitialized declaration

var uninitialized_integer: Int32
var uninitialized_float: Float32

# Implicit type declaration

var assumed_integer = 2 # Int32 is the default integer type
var assumed_float = 3.0 # Float32 is the default float type

# Explicit type declaration

var explicit_integer: UInt8 = 5
var explicit_float: Float16 = 6.0
```

## Strings

> [!NOTE]
> This has not been implemented yet

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
func square(x: Float32) -> Float32 {
    return x * x
}
```

## Structs

> [!NOTE]
> This has not been implemented yet

```julia
struct Point {
    x: Float32
    y: Float32

    func __init__(self, x: Float32, y: Float32) {
        self.x = x
        self.y = y
    }

    func add(self, other: Point) -> Point {
        return Point(self.x + other.x, self.y + other.y)
    }
}
```

## Imports

> [!NOTE]
> This has not been implemented yet

```julia

# Importing specific variables
import sin, cos, sqrt as square_root from math

println(sin(0.5))
println(cos(0.5))
println(square_root(2))

# Importing modules
import os
import time as t
import * as rng from random

println(os.cwd())
println(t.now())
println(rng.next_int())

# Importing everything from a module
import * from math

println(pi)
println(factorial(6))

```

## Type Aliases

> [!NOTE]
> This has not been implemented yet

```julia
struct Circle {
    radius: Float32
    x: Float32
    y: Float32
}

struct Rectangle {
    width: Float32
    height: Float32
    x: Float32
    y: Float32
}

type Shape = Circle | Rectangle
```
