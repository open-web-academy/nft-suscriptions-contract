
learn.ow.academy <- Acceder al contenido
subs.ow.academy <- Comprar suscripciones y Minar

ID=dev-1675454434094-81755855367013
USDTCONTRACT=usdt.fakes.testnet

Inicializar contrato:

    near call $ID new_default_meta '{"owner_id": "'$ID'"}' --accountId $ID

Mostrar Costos

    near view $ID show_costs

Actualizar Costos

    near call $ID change_costs '{"one_month_cost": "10000000", "six_months_cost": "50000000", "one_year_cost": "100000000", "permanent_cost": "200000000"}' --accountId $ID

Mostrar contrato de USDT

    near view $ID show_usdt_contract

Cambiar contrato de USDT

    near call $ID change_usdt_contract '{"new_contract": "usdt.fakes.testnet"}' --accountId $ID

Mostrar contador de suscripciones de usuario

    near call $ID show_pendant_suscriptions --accountId yairnava.testnet

Consultar balance de USDT

    near view $USDTCONTRACT ft_balance_of '{"account_id": "yairnava.testnet"}'

Transferir 10 USDT.e

    near call $USDTCONTRACT ft_transfer_call '{"receiver_id": "'$ID'", "amount": "10000000", "msg": ""}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Transferir 50 USDT.e

    near call $USDTCONTRACT ft_transfer_call '{"receiver_id": "'$ID'", "amount": "50000000", "msg": ""}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Transferir 100 USDT.e

    near call $USDTCONTRACT ft_transfer_call '{"receiver_id": "'$ID'", "amount": "100000000", "msg": ""}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Transferir 200 USDT.e

    near call $USDTCONTRACT ft_transfer_call '{"receiver_id": "'$ID'", "amount": "200000000", "msg": ""}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Consultar balance

    near view $USDTCONTRACT ft_balance_of '{"account_id": "'$ID'"}'

Minar

    near call $ID mint '{ "receiver_id": "'yairnava.testnet'", "type_suscription": "'one_month'" }' --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

    near call $ID mint '{"receiver_id": "'yairnava.testnet'", "type_suscription": "'six_months'" }' --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

    near call $ID mint '{"receiver_id": "'yairnava.testnet'", "type_suscription": "'one_year'" }' --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

    near call $ID mint '{"receiver_id": "'yairnava.testnet'", "type_suscription": "'permanent'" }' --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

Consultar NFT

    near view $ID nft_token '{"token_id": "0"}'

Consultar NFT de un segmento

    near view $ID nft_tokens '{"from_index": "0", "limit": 50}' --accountId yairnava.testnet

Consultar NFT de un segmento por usuario

    near view $ID nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' 