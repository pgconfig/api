package config

import (
	"bytes"
	"fmt"
)

// IEC Sizes
const (
	B Byte = 1 << (iota * 10)
	KB
	MB
	GB
	TB
	PB
	EB
)

// Byte is a int that is displayed in a fancy way.
// Follows ISO/IEC 80000 spec.
type Byte int64

// MarshalJSON converst the byte
func (b *Byte) MarshalJSON() ([]byte, error) {
	return marshalBytes(b)
}

// String converst bytes into human bytes
func (b *Byte) String() string {
	return formatBytes(*b)
}

func marshalBytes(b *Byte) ([]byte, error) {

	buffer := bytes.NewBufferString("")
	buffer.WriteString(fmt.Sprintf(`"%s"`, formatBytes(Byte(*b))))

	return buffer.Bytes(), nil
}

// ideas from https://github.com/dustin/go-humanize/blob/master/bytes.go#L68
func formatBytes(i Byte) string {

	if i <= 0 {
		return fmt.Sprintf("%d", i)
	}
	if i < 1024 {
		return fmt.Sprintf("%dB", i)
	}
	if i < 1024*KB {
		return fmt.Sprintf("%dKB", i/KB)
	}
	if i < 1024*MB {
		return fmt.Sprintf("%dMB", i/MB)
	}
	if i < 1024*GB {
		return fmt.Sprintf("%dGB", i/GB)
	}
	if i < 1024*TB {
		return fmt.Sprintf("%dTB", i/TB)
	}

	return ""
}
