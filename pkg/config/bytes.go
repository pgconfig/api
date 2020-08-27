package config

import (
	"bytes"
	"fmt"
)

const (
	// B is a byte
	B = 1

	// KB is a KibiByte
	KB = B * 1024

	// MB is a MebiByte
	MB = KB * 1024

	// GB is a GibiByte
	GB = MB * 1024

	// TB is a TebiByte
	TB = GB * 1024
)

// Byte is a int that is displayed in a fancy way
type Byte int

// MarshalJSON converst the byte
func (b *Byte) MarshalJSON() ([]byte, error) {
	return marshalBytes(b)
}

func marshalBytes(b *Byte) ([]byte, error) {

	buffer := bytes.NewBufferString("")
	buffer.WriteString(fmt.Sprintf(`"%s"`, formatBytes(int(*b))))

	return buffer.Bytes(), nil
}

// ideas from https://github.com/dustin/go-humanize/blob/master/bytes.go#L68
func formatBytes(i int) string {

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
