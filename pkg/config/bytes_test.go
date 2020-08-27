package config

import (
	"reflect"
	"testing"
)

func Test_marshalBytes(t *testing.T) {
	tests := []struct {
		name    string
		args    Byte
		want    []byte
		wantErr bool
	}{
		{name: "should parse bytes to the postgres byte format",
			args: 10 * GB,
			want: []byte(`"10GB"`)},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := marshalBytes(&tt.args)
			if (err != nil) != tt.wantErr {
				t.Errorf("marshalBytes() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("marshalBytes() = %v, want %v", string(got), string(tt.want))
			}
		})
	}
}

func Test_formatBytes(t *testing.T) {
	type args struct {
		i int
	}
	tests := []struct {
		name string
		args int
		want string
	}{
		{"none", -1, "-1"},
		{"none", 0, "0"},
		{"bytes", 5, "5B"},
		{"kb", 455 * KB, "455KB"},
		{"mb", 1023 * MB, "1023MB"},
		{"gb", 565 * GB, "565GB"},
		{"tb", 396 * TB, "396TB"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := formatBytes(tt.args); got != tt.want {
				t.Errorf("formatBytes() = %v, want %v", got, tt.want)
			}
		})
	}
}
