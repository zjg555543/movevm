package cosmwasm

import (
	"github.com/zjg555543/movevm/api"
	"github.com/zjg555543/movevm/types"
)

type KVStore = api.KVStore

// GoAPI is a reference to some "precompiles", go callbacks
type GoAPI = api.GoAPI

// Querier lets us make read-only queries on other modules
type Querier = types.Querier

// GasMeter is a read-only version of the sdk gas meter
type GasMeter = api.GasMeter

func Version() (string, error) {
	return api.ApiMoveVersion()
}

func Publish(env []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {
	api.ApiPublish(env, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}

func Run(env []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {
	api.ApiRun(env, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}

func InputOutput(env []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) ([]byte, error) {
	return api.ApiInputOutput(env, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}
