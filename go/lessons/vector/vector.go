package vector 

import "math"

type Vector interface {
	X() float64
	Y() float64
	Add(Vector) Vector
	Sub(Vector) Vector
	Mul(Vector) Vector
	Div(Vector) Vector
	Mag() float64
	Angle() float64
}

type vector struct {
	x float64
	y float64
}

func NewVector2(x, y float64) Vector {
	return &vector{x, y}
}

func (v *vector) X() float64 {
	return v.x
}

func (v *vector) Y() float64 {
	return v.x
}

// Add one vector from another and returns a new vector that is the result.
func (v *vector) Add(other Vector) Vector {
	return NewVector2(v.x+other.X(), v.y+other.Y())
}

// Subtracts one vector from another and returns a new vector that is the result.
func (v *vector) Sub(other Vector) Vector {
	return NewVector2(v.x-other.X(), v.y-other.Y())
}

// Multiply one vector by another and returns a new vector that is the result.
func (v *vector) Mul(other Vector) Vector {
	return NewVector2(v.x*other.X(), v.y*other.Y())
}

// Divides one vector by another and returns a new vector that is the result.
func (v *vector) Div(other Vector) Vector {
	return NewVector2(v.x/other.X(), v.y/other.Y())
}

// Calculates the magnitude (length) of the vector and returns the result as a float (this is simply the equation sqrt(x*x + y*y).)
func (v *vector) Mag() float64 {
	return math.Sqrt(v.x*v.x + v.y*v.y)
}

// Calculates the angle of rotation for this vector.
func (v *vector) Angle() float64 {
	return math.Atan2(v.y, v.x)
}
