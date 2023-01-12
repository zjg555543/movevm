package movevm

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

func Build(gasMeter GasMeter, store KVStore) {
	api.ApiBuild(&gasMeter, store)
}

func Publish(codeByte []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {
	api.ApiPublish(codeByte, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}

func Run(codeByte []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {
	api.ApiRun(codeByte, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}

func InputOutput(codeByte []byte, info []byte, msg []byte, gasMeter GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) ([]byte, error) {
	return api.ApiInputOutput(codeByte, info, msg, &gasMeter, store, goApi, querier, gasLimit, printDebug)
}
