package profile

import (
	"testing"
)

func TestProfile_Set(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    Profile
		wantErr bool
	}{
		{
			name:    "Web uppercase",
			input:   "WEB",
			want:    Web,
			wantErr: false,
		},
		{
			name:    "Web lowercase",
			input:   "web",
			want:    Web,
			wantErr: false,
		},
		{
			name:    "OLTP uppercase",
			input:   "OLTP",
			want:    OLTP,
			wantErr: false,
		},
		{
			name:    "OLTP lowercase",
			input:   "oltp",
			want:    OLTP,
			wantErr: false,
		},
		{
			name:    "DW uppercase",
			input:   "DW",
			want:    DW,
			wantErr: false,
		},
		{
			name:    "DW lowercase",
			input:   "dw",
			want:    DW,
			wantErr: false,
		},
		{
			name:    "Mixed uppercase",
			input:   "MIXED",
			want:    Mixed,
			wantErr: false,
		},
		{
			name:    "Mixed mixed case",
			input:   "Mixed",
			want:    Mixed,
			wantErr: false,
		},
		{
			name:    "Mixed lowercase",
			input:   "mixed",
			want:    Mixed,
			wantErr: false,
		},
		{
			name:    "Desktop uppercase",
			input:   "DESKTOP",
			want:    Desktop,
			wantErr: false,
		},
		{
			name:    "Desktop mixed case",
			input:   "Desktop",
			want:    Desktop,
			wantErr: false,
		},
		{
			name:    "Desktop lowercase",
			input:   "desktop",
			want:    Desktop,
			wantErr: false,
		},
		{
			name:    "Invalid profile",
			input:   "invalid",
			want:    "",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var p Profile
			err := p.Set(tt.input)

			if (err != nil) != tt.wantErr {
				t.Errorf("Profile.Set() error = %v, wantErr %v", err, tt.wantErr)
				return
			}

			if !tt.wantErr && p != tt.want {
				t.Errorf("Profile.Set() = %v, want %v", p, tt.want)
			}
		})
	}
}
