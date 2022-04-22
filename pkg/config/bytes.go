package config

import (
	"bytes"
	"fmt"
	"strconv"
	"strings"
	"unicode"
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

// MarshalJSON converts the byte
func (b *Byte) MarshalJSON() ([]byte, error) {
	return marshalBytes(b)
}

// String converts bytes into human bytes
func (b *Byte) String() string {
	return FormatBytes(*b)
}

func (b *Byte) Set(v string) error {
	val, err := Parse(v)
	*b = val

	return err
}

func (b *Byte) Type() string {
	return "Byte"
}

func marshalBytes(b *Byte) ([]byte, error) {

	buffer := bytes.NewBufferString("")
	buffer.WriteString(fmt.Sprintf(`"%s"`, FormatBytes(Byte(*b))))

	return buffer.Bytes(), nil
}

// ideas from https://github.com/dustin/go-humanize/blob/master/bytes.go#L68
func FormatBytes(i Byte) string {

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

// Parses a postgres-like bytes string into Bytes
func Parse(s string) (Byte, error) {

	if len(strings.TrimSpace(s)) == 0 {
		return 0, nil
	}

	lastDigit := 0
	for _, r := range s {
		if !(unicode.IsDigit(r)) {
			break
		}
		lastDigit++
	}

	num := s[:lastDigit]

	if len(strings.TrimSpace(num)) == 0 {
		return 0, nil
	}
	f, err := strconv.ParseFloat(num, 64)
	if err != nil {
		return 0, fmt.Errorf("fail to parse float: %w", err)
	}

	unity := strings.ToLower(strings.TrimSpace(s[lastDigit:]))

	return Byte(f) * extractBytesUnity(unity), nil
}

func extractBytesUnity(val string) Byte {
	switch val {
	case "kb":
		return KB
	case "mb":
		return MB
	case "gb":
		return GB
	case "tb":
		return TB
	default:
		return B
	}
}
