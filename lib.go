package cosmwasm

import (
	"github.com/zjg555543/movevm/api"
)

// LibwasmvmVersion returns the version of the loaded library
// at runtime. This can be used for debugging to verify the loaded version
// matches the expected version.
func LibwasmvmVersion() (string, error) {
	return api.LibwasmvmVersion()
}

func LibExecute(gas_limited uint64) {
	api.LibExecute(gas_limited)
}
