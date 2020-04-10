# All networks

## Mainnet

Explorer

- <https://tronscan.org/>
- <https://tronscan.io/>
- <https://www.trxplorer.io/>

Full Node HTTP API / Event API: <https://api.trongrid.io>

Tronscan API: <https://apilist.tronscan.org>

gRPC

- grpc.trongrid.io:50051

Public Full Node:

```text
3.225.171.164
52.53.189.99
18.196.99.16
34.253.187.192
52.56.56.149
35.180.51.163
54.252.224.209
18.228.15.36
52.15.93.92
34.220.77.106
13.127.47.162
13.124.62.58
35.182.229.162
18.209.42.127
3.218.137.187
34.237.210.82
```

Data backup:

- <https://backups.trongrid.io/> LevelDB with internalTx, old backups deleted more frequently
- <http://47.89.178.46/> LevelDB
- <http://47.89.178.46:18811/> RocksDB

## Nile Testnet

NOTE: Nile uses newer test branch than the Mainnet.

Home page: <http://nileex.io/>

Faucet: <http://nileex.io/join/getJoinPage>

Explorer: <https://nile.tronscan.org/>

Status: <http://nileex.io/status/getStatusPage>

Full Node HTTP API: <https://api.nileex.io/>

Event API: <https://event.nileex.io/>

Tronscan API: <https://nileapi.tronscan.org>

Public Fullnode:

- 47.252.19.181
- 47.252.3.238

Data backup:

- <http://47.90.243.177/>

## Shasta Testnet

NOTE: You can NOT join the Shasta testnet.

Home page: <https://www.trongrid.io/shasta>

Faucet: <https://www.trongrid.io/faucet>

Explorer: <https://shasta.tronscan.org/>

Full Node / Event HTTP API: <https://api.shasta.trongrid.io>

Tronscan API: <https://api.shasta.tronscan.org>

gRPC

- grpc.shasta.trongrid.io:50051

## Tronex Testnet

Home page: <http://testnet.tronex.io/>

Faucet: <http://testnet.tronex.io/join/getJoinPage>

Explorer: <http://3.14.14.175:9000/>

Status: <http://testnet.tronex.io/status/getStatusPage>

Event API: <https://testapi.tronex.io>

Public Fullnode

- 47.252.87.28
- 47.252.85.13

NOTE: This testnet has a dappchain testnet side-chain.

## DAppChain (SUN Network)

Home page: <https://tron.network/sunnetwork/>

Explorer: <https://dappchain.tronscan.org/>

Event Server: <https://sun.tronex.io/event>

NOTE: event listening is via:
> <https://sun.tronex.io/event/contract/{CONTRACT_ADDRESS}/{EVENT_NAME}>

### Cross Chain

SideChainID
> 41E209E4DE650F0150788E8EC5CAFA240A23EB8EB7

TRON Network MainChain gateway contract address
> TWaPZru6PR5VjgT4sJrrZ481Zgp3iJ8Rfo

SUN Network DAppChain(SideChain) gateway contract address
> TGKotco6YoULzbYisTBuP6DWXDjEgJSpYz

### Full Nodes

HTTP API:

- <https://sun.tronex.io/wallet> (CORS enabled)
- 47.90.245.159:8090
- 47.90.211.50:8090
- 47.252.6.19:8090
- 47.89.185.14:8090

gRPC:

- 47.90.245.159:50051
- 47.90.211.50:50051
- 47.252.6.19:50051
- 47.89.185.14:50051

### Solidity Nodes

HTTP API

- sun.tronex.io/walletsolidity (CORS enabled)
- 47.90.245.159:8091
- 47.90.211.50:8091
- 47.252.6.19:8091
- 47.89.185.14:8091

gRPC:

- 47.90.245.159:50061
- 47.90.211.50:50061
- 47.252.6.19:50061
- 47.89.185.14:50061

## DAppChain Testnet (SUN Network Testnet)

Home page: <https://tron.network/sunnetwork/doc/>

Faucet: Use <http://testnet.tronex.io/join/getJoinPage> to get main-chain coin.

Event Server:

- <http://47.252.81.14:8070> (CORS Enabled) - main chain
- <http://47.252.87.129:8070> (CORS Enabled) - side chain
- <https://suntest.tronex.io/event> (CORS Enabled) - side chain

NOTE: event listening is via:
> <http://47.252.87.129:8070/events/contract/{CONTRACT_ADDRESS}/{EVENT_NAME}>

### Testnet Cross Chain

sidechainid
> 413AF23F37DA0D48234FDD43D89931E98E1144481B

main chain gateway contract address
> TFLtPoEtVJBMcj6kZPrQrwEdM3W3shxsBU

side chain gateway contract address
> TRDepx5KoQ8oNbFVZ5sogwUxtdYmATDRgX

### Testnet Full Nodes

HTTP API:

- 47.252.85.90:8090
- 47.252.85.90:8070(CORS Enabled)
- 47.252.80.185:8090
- 47.252.84.141:8090

gRPC:

- 47.252.85.90:50051
- 47.252.80.185:50051
- 47.252.84.141:50051

### Solidity Notes

HTTP API:

- 47.252.85.90:8091
- 47.252.85.90:8071(CORS Enabled)
- 47.252.80.185:8091
- 47.252.84.141:8091

RPC Interface

- 47.252.85.90:50060
- 47.252.80.185:50060
- 47.252.84.141:50060
