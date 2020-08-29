package category

import "github.com/pgconfig/api/pkg/config"

// NetworkCfg is the main memory category
type NetworkCfg struct {
	ListenAddresses string `json:"listen_addresses"`
	MaxConnections  int    `json:"max_connections"`
}

// NewNetworkCfg creates a new Network Configuration
func NewNetworkCfg(in config.Input) *NetworkCfg {
	return &NetworkCfg{
		ListenAddresses: "*",
		MaxConnections:  in.MaxConnections,
	}
}
