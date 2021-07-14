package utils

func Ternary(condition bool, objectIfTrue interface{}, objectIfFalse interface{}) interface{} {
	if condition {
		return objectIfTrue
	} else {
		return objectIfFalse
	}
}
