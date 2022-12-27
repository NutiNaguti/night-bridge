#/bin/bash

near dev-deploy ./out/ft_erc20.wasm --initFunction "init" --initArgs '{"name": "FunCoin", "symbol": "FUNC", "decimals": 18, "total_supply": 1000000000000, "admin_list": ["nutinaguti.testnet"]}'
# near dev-deploy 
