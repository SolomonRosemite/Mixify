package common

import "strconv"

func LiteralToPtr[T any](v T) *T {
	return &v
}

func StringToUint(s string, bitSize int) (uint, error) {
	var id uint64
	var err error

	if id, err = strconv.ParseUint(s, 10, bitSize); err != nil {
		return 0, err
	}

	return uint(id), nil
}
