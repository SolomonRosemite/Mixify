package common

func LiteralToPtr[T any](v T) *T {
	return &v
}
