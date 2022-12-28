package cosmwasm

import (
	"github.com/zjg555543/movevm/api"
)

func LibwasmvmVersion() (string, error) {
	return api.ApiMoveVersion()
}

func LibPublish(gas_limited uint64) {
	api.ApiPublish(gas_limited)
}

func LibRun(gas_limited uint64) {
	api.ApiRun(gas_limited)
}
