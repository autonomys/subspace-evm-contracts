# AutoBridge Trial-1

<u>About</u>: "LZInfra script: a part is LOCAL_EID, another part is REMOTE_EID".

<u>Commit</u>: [e3067737ceaa8bd99ffe0f9db89a7dc8eab91c4c](https://github.com/subspace/subspace-evm-contracts/commit/e3067737ceaa8bd99ffe0f9db89a7dc8eab91c4c)

## Description

![](../../img/autobridge_trial_1.png)

## 1. Deploy EndpointV2 contracts on both chains

Use `script/EndpointV2.s.sol:EndpointV2Script`

### Nova

```sh
$ forge script ./script/EndpointV2.s.sol:EndpointV2Script --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${NOVA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL --legacy

##### 490000
✅  [Success]Hash: 0x656ff1711eea5d2e174db184aff07fe89d1f8a10dcd23529eb051db493b35aa9
Contract Address: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5
Block: 418128
Paid: 0.0024027835 ETH (4805567 gas * 0.5 gwei)
```

```json
"arguments": [
    "490000",
    "0xB751710Af8Ce68677aB960adB103060f38d09714"
],
```

### Sepolia

```sh
$ forge script ./script/EndpointV2.s.sol:EndpointV2Script --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL

##### sepolia
✅  [Success]Hash: 0xb3984c7d36eead35b55b36dc1fa31ee0643a07b3d1c6b7ae7b6f3e8e75f10c9d
Contract Address: 0xc839Ea8baCB7604245c09fa32Dc0a80250663C6d
Block: 5761844
Paid: 0.004805573775303385 ETH (4805555 gas * 1.000003907 gwei)
```

```json
"arguments": [
    "40161",
    "0xB751710Af8Ce68677aB960adB103060f38d09714"
],
```

## 2. Setup LZInfra on both chains

Run LZInfra script: `script/LzInfra.s.sol:LzInfraScript` with already deployed EndpointV2 contracts on both chains in [step-1](#1-deploy-endpointv2-contracts-on-both-chains).

Now, spin up a forked Nova RPC (as there is a protocol issue with Nova chain) on another open terminal:

```sh
$ anvil --fork-url $NOVA_RPC_URL
Fork
==================
Endpoint:       https://nova-squids.gemini-3h.subspace.network/ws
Block number:   418211
Block hash:     0x3512dd1964165853c68114925d3d7d64158a04dde45538e5d632a15bcda1801a
Chain ID:       490000
```

### Nova

In order to view the arguments used for each tx on forked RPC, refer this file: [lzinfra-nova-run.json](./lzinfra-nova-run.json) and in order to see all the addresses, refer this file: [lz_infra_addresses_nova.txt](./lz_infra_addresses_nova.txt)

```sh
forge script ./script/LzInfra.s.sol:LzInfraScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8545 --broadcast --legacy
```

<details><summary>Log:</summary>

```sh
==========================

Chain 490000

Estimated gas price: 1.4375 gwei

Estimated total gas used for script: 37781590

Estimated amount required: 0.054311035625 ETH

==========================
##
Sending transactions [0 - 28].
⠒ [00:00:00] [################################################################################################>-------------------------------------------] 20/29 txes (0.0s)##
Waiting for receipts.
⠤ [00:00:00] [#############################################################################################>------------------------------------------] 20/29 receipts (0.0s)
##### 490000
✅  [Success]Hash: 0x67b97a9b818fa128bf4f5f40df5c9400c492dd752b889e5e3607d5bb85b579f2
Contract Address: 0xC2d6cD33c3a5AE45195c7F600cD81c84a16B9374
Block: 418212
Paid: 0.0009975344375 ETH (693937 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xb1ecbfde48baa89d7b61afa285bf12b9af11adb658522d4418501e023c1db8bd
Contract Address: 0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B
Block: 418213
Paid: 0.002116991875 ETH (1472690 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x287731f48c9f2cda1ac8284a473fe4ce8cfdd6bde1d2be6564995997ddb49380
Contract Address: 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58
Block: 418214
Paid: 0.0059495005625 ETH (4138783 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x29d0c0e8ac71163ddbc8dec6ffb80227ebfec6322e175f8d1fec8211314094fa
Contract Address: 0xe7bC84fA427E954Ef0f6E8ed09c13B7a13ac4330
Block: 418214
Paid: 0.003117836875 ETH (2168930 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x89047c97a475cf15826c3adefdf4451b3f21d1d7d27b5c2456ae74c7f5ec9914
Block: 418214
Paid: 0.0001121293125 ETH (78003 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x2c11e24644aff25adefe6359e0f64af9c26aa49d424ab058146a4066d5b1847b
Block: 418214
Paid: 0.00011215375 ETH (78020 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xc085f3d2d56f56f837367767873a9e2101f1d64ca3366dccfe27852b0317c786
Block: 418215
Paid: 0.0001122644375 ETH (78097 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xc20ed592f0b1813422b9f7405f0d3a57766e70a269683dda9ce3b975aeca5927
Contract Address: 0x56Ec18E6cfdf441a39F8b5c7ec9620EEFd12d010
Block: 418216
Paid: 0.003648878125 ETH (2538350 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xb3c11ac327fbcb148bc549984c694201bc2e2cf210d5d20fd866bfc37f77d268
Contract Address: 0x04aAc97FC51cC6AbE276D0E32BA4A46224fC3b88
Block: 418217
Paid: 0.000586238375 ETH (407818 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xc80256ef3974eebeef994a221bda3ee49b966fec82ffd8a8f60f947278af4c5a
Contract Address: 0x390877EaFbb0C0Ba1c79106FADeF51427E4eDAf9
Block: 418217
Paid: 0.000429295 ETH (298640 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x4883aaecdf91ae1b46cdc63a819f8661cb1c7d0e161e759cc693e753976d9511
Contract Address: 0xfC2f95FA02e77efd97F306243271D0861B7f4922
Block: 418218
Paid: 0.0058976039375 ETH (4102681 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x634462d6c8c44a361eac1e8b649910005771e277190e7df1e850c29ca3b00b67
Contract Address: 0xA529d218928a801d9A4504d8308D95Ce9792C619
Block: 418218
Paid: 0.0036425258125 ETH (2533931 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xb88608c3ad9fa85c5c92339a890ae14a5485b93e20874286e6f8c2181591a3cf
Contract Address: 0xF602F10dc5Dc741Ed1299841B053948951C1a999
Block: 418218
Paid: 0.0018807401875 ETH (1308341 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x9eb7cf6ebee50aefd420241d20dcc986809aa769d27cc451848b5f169c73c6d3
Block: 418218
Paid: 0.000227909875 ETH (158546 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x4bcd9f2bf01293f9c837b9a5b909a07815d81163ff18ae43f04144d718391839
Contract Address: 0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28
Block: 418220
Paid: 0.0046411024375 ETH (3228593 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x3eb0db439169b4a1657a21a84e9e061586cef59ad7c6638730ab4f275ff3046a
Block: 418220
Paid: 0.000073508 ETH (51136 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xab6873500b91c7d8ac87172b2b71c32d3354db54876112de52283d9404e200b6
Contract Address: 0xd3242Fe2085d3D7dC73De281D4c834EEA61447C6
Block: 418220
Paid: 0.0012661385 ETH (880792 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xf8709eadc38f80afdf2da83362f3e672644d7a442454a8d2c6e4c086ba07f254
Block: 418220
Paid: 0.0000436439375 ETH (30361 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xae74322401dbb786c2bb47417c7068f3936a56761aeba6868960cd390a1d0cd0
Block: 418220
Paid: 0.0000433219375 ETH (30137 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x154f89077a0902dfe5f9d2c3846f852cab5fe2b26ddda654353b3a9e700714d9
Contract Address: 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B
Block: 418220
Paid: 0.003980676125 ETH (2769166 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xb084c6306a754ce8c7bc064e25104c2c5af782e06dc4e0f26f226b51e5c9ce55
Contract Address: 0xefcC10b4E00eb7a587c6e9bA0d418FcA4776435b
Block: 418220
Paid: 0.0015614771875 ETH (1086245 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xeab98ff2bf599c763b5450dd4244d235f6e5d958be76a2ab07eabb9a2a959a96
Block: 418221
Paid: 0.0004520549375 ETH (314473 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x97cb65416e20b18eca643e0a8d4e2af6fd216a0dfde788ed64b140e24e9d582a
Block: 418221
Paid: 0.000043599375 ETH (30330 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xe68697f878896d4db98b3f90dd828d787e64e23df5788b00a1f53b9ad86cd52f
Block: 418222
Paid: 0.00010608175 ETH (73796 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xf9a65b9cf443a4db473dc05817983c3ea07342c4838ec1e5cd967de8d8a0c494
Block: 418222
Paid: 0.0000727446875 ETH (50605 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xa8b14ba5e8c58bd3a66cf8dc23bed3b0f21103ff8fa1781f8db2e0ee8dc4c53d
Block: 418222
Paid: 0.0001541675625 ETH (107247 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x7b5589f7771985cd958eb4b155acb98e95346c8e13c010b66de4469527f5603b
Block: 418223
Paid: 0.00015401375 ETH (107140 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0xb00934f4b424d69eddf6aa934f4e836d8990eed4f87a5f3b277201d6eaae977c
Block: 418223
Paid: 0.0000814358125 ETH (56651 gas * 1.4375 gwei)


##### 490000
✅  [Success]Hash: 0x0b71fcb92e1a062e7eab0e200b3704b53061894c4d627b599915a974d67c0c18
Block: 418224
Paid: 0.0000907393125 ETH (63123 gas * 1.4375 gwei)



==========================
```

</details>

### Sepolia

In order to view the arguments used for each tx on forked RPC, refer this file: [lzinfra-sepolia-run.json](./lzinfra-sepolia-run.json) and in order to see all the addresses, refer this file: [lz_infra_addresses_sepolia.txt](./lz_infra_addresses_sepolia.txt).

```sh
forge script ./script/LzInfra.s.sol:LzInfraScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL
```

<details><summary>Log:</summary>

```sh
==========================

Chain 11155111

Estimated gas price: 1.018295246 gwei

Estimated total gas used for script: 37784756

Estimated amount required: 0.038476037406069976 ETH

==========================
##
Sending transactions [0 - 28].
⠁ [00:00:09] [############################################################################################################################################] 29/29 txes (0.0s)##
Waiting for receipts.
⠁ [00:00:24] [########################################################################################################################################] 29/29 receipts (0.0s)
##### sepolia
✅  [Success]Hash: 0x255f0903b9af3a8a45090101788b3e0b0573686175da14cf78811ad18252b365
Contract Address: 0x7EB103cEF33C2fD17976a6abcC2DA24DA4f2B788
Block: 5761927
Paid: 0.00070663238311264 ETH (693937 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xb806599f38da4bacdc1dc590c5e181771b7a18ade184ac8d81b3b8a1715c5ada
Contract Address: 0x26601B4776bBe1C973dd7b745Da60b31ac586EdD
Block: 5761927
Paid: 0.0014996324511968 ETH (1472690 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0x450b7364d02dc2387fd5ecd7aa957fcadbb7a474bcde3b8e2e03c7c7c1ca861a
Contract Address: 0x312756f49B588FE5C7E48a6FD4EBd27F5bE894e7
Block: 5761927
Paid: 0.00421450087612576 ETH (4138783 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0x091863b07ca29682a8c8b142b10667e09bf78690fa53b9672cc5eaa6edb256f7
Contract Address: 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64
Block: 5761927
Paid: 0.0022086099670496 ETH (2168930 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xf702d6082636e7b850687300df43d5b4a22133830addcdb9f817f3d10365e239
Block: 5761927
Paid: 0.00007943004304416 ETH (78003 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xfe1e1e27cec8d069d9d08d6e595069d86af22e9c4685160c15ce1e1f9e5604ea
Block: 5761927
Paid: 0.0000794473540544 ETH (78020 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xc65744593c390f98ccda32744142e1cd7bff5f5e820bfb2617a444dedb09e24d
Block: 5761927
Paid: 0.00007952576274784 ETH (78097 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0x301a229a84c8dcbd829427d37c951a8f7ce7a6b7292f08a12a56fbeb16f7fc55
Contract Address: 0xD0e70c4f57227B3afe30d090369a9B14D23C9D21
Block: 5761927
Paid: 0.002584788402512 ETH (2538350 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xc3bf712315d9e2a656daf55ce5823f776ab09b2102e324f976f8094ce4bb6e59
Contract Address: 0x4f010A852F20CE9e39654B34e93D558698e0942f
Block: 5761927
Paid: 0.00041527891612096 ETH (407818 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0xf73b83cf5dd9cf43e814e22ae654a648290f4249c57621eaf72df0f2179fcc7d
Contract Address: 0x7D13cfA59608b1F4ae111578C9F041e1fCd7B83e
Block: 5761927
Paid: 0.0003041035351808 ETH (298640 gas * 1.01829472 gwei)


##### sepolia
✅  [Success]Hash: 0x22f66402a3ae9c72b6094c2406a202e3a80c0886db609cbb72392c5e3a1950c0
Contract Address: 0x228dFa90076e503365c98D86183415ea0b0eb86a
Block: 5761928
Paid: 0.004177726488307855 ETH (4102669 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x247135f0621fa24f424a7aa6314ba1e6088be47687a079ddcc3192e920a9a943
Contract Address: 0x9dC7cc9C2A36772e96F3c44eb1f3332D146c8128
Block: 5761928
Paid: 0.002580276528651605 ETH (2533919 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x12e3de0c41d7eaf29a9c5857669f4bdcece7aa20774d1eb7f15668d434a00d9d
Contract Address: 0xD4243b1C9996c00a187184284F1389469186C1D2
Block: 5761928
Paid: 0.001332276830385095 ETH (1308341 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x9a70bad83c654452d10da10731c76b7691c89b6d62d852c83579dcb4aa9fbdca
Block: 5761928
Paid: 0.00016144656656807 ETH (158546 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x31f9e88618491ab055705e9f8abef07dc3138f341cf85c4670f366ca3df3f158
Contract Address: 0x71100123d19a2Cf1e007D4b2Fd45bF9eac6542E9
Block: 5761928
Paid: 0.003287647227535895 ETH (3228581 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x8616e3666ba3fab26763e5f135e73e400e9107d46e79d0e95beb10576725c4d0
Block: 5761928
Paid: 0.00005208374217466 ETH (51148 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0xb23916d71c4a0787c67cf6a204c101d4b65ab0ab3cf3554a2fc7f99ea4507da1
Contract Address: 0xB9A14Bea82C816d5d0Ab5983c6cfA0de74C9d74E
Block: 5761928
Paid: 0.00089690590907764 ETH (880792 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0x4e5cb2d0614f356745657331d90275b57a879a2d37ae6694ce9d0aceaa8a7c90
Block: 5761928
Paid: 0.000030916448270995 ETH (30361 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0xb6504144050584e3b8be15ded5e8db896bb1c42958fc0322a666dcb9aa1a3821
Block: 5761928
Paid: 0.000030688350236915 ETH (30137 gas * 1.018294795 gwei)


##### sepolia
✅  [Success]Hash: 0xe5f5efe0d04e42de9214c8adb63b5cecf8e375c2cfdcde2243bbc05ad82fe353
Contract Address: 0xB37EA44C3a07fDEa7c45d74F4227bC18F181FF1F
Block: 5761929
Paid: 0.002819827567977578 ETH (2769166 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0xa7d58f78438d2cc967de3db12d081d267d06321844e922cf0c3be9be1aa47435
Contract Address: 0x2D4c95E9930738B980c5D3D4e6B24c8a8CD2d43A
Block: 5761929
Paid: 0.001106117725184335 ETH (1086245 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0x74edeecf2a7dc1f0cd5e95382268bb10edcb6e09f83d61ea92d58a567b9fa51c
Block: 5761929
Paid: 0.000320226246741659 ETH (314473 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0xe070cd229b1213a9ecd010c792ec8e8093ee0a08dbd2f600c33bf891577396f0
Block: 5761929
Paid: 0.00003088488380139 ETH (30330 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0x4fed0820145e24ded5edd227c24f43077708a31076f1bd10b7ad1832bfee29d7
Block: 5761929
Paid: 0.000075158308724464 ETH (73808 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0xf59ea422b54d1e2734c995fb19d138970d4b3b0bc2d70e64352096130bc46d02
Block: 5761929
Paid: 0.000051543032092811 ETH (50617 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0x35440cb3b6b69b05d8bcceb16ea607c29297871bb9e1aa7ea4749f5510efc689
Block: 5761929
Paid: 0.000109221290855697 ETH (107259 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0xea51ab814b61a57f0acaa3504c25f6a1f48584bd66138a4a358e8cb3f5935e6d
Block: 5761929
Paid: 0.000109112333303216 ETH (107152 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0xd10442af2953ed6dd531978a93bb72ad56aa387ae32b879a95e9cbb9f56b0992
Block: 5761929
Paid: 0.000057699642955429 ETH (56663 gas * 1.018294883 gwei)


##### sepolia
✅  [Success]Hash: 0x8d03a8c1fd8724242dc4c3180418a2542ee0de56227b5b527dac7b854d9291c0
Block: 5761929
Paid: 0.000064290047438205 ETH (63135 gas * 1.018294883 gwei)



==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
Total Paid: 0.029465998861428474 ETH (28936610 gas * avg 1.018294799 gwei)
```

</details>

## 3. Deploy WTsscLz contracts on both chains

Use `script/WTsscLz.s.sol:WTsscLzScript`.

### Nova

```sh
$ forge script script/WTsscLz.s.sol:WTsscLzScript --rpc-url http://127.0.0.1:8545 --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL --legacy

##### 490000
✅  [Success]Hash: 0xc80948035ec70cf717701eb5ce1b0152fa38bce5740a23d3e4fbc289b6dd8433
Contract Address: 0x8fF90adA0487bC78A5E0E28463964A98ab041720
Block: 418225
Paid: 0.003322520072536392 ETH (3084676 gas * 1.077105042 gwei)
```

```json
"arguments": [
    "\"Wrapped Subspace Token\"",
    "\"WTSSC\"",
    "0x8434BBFebA81d93469E97ab888089d65e84e1fd5",
    "0xB751710Af8Ce68677aB960adB103060f38d09714"    
],
```

### Sepolia

```sh
$ forge script script/WTsscLz.s.sol:WTsscLzScript --rpc-url $SEPOLIA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL

##### sepolia
✅  [Success]Hash: 0x9f0208030f7622ab9fd3e336174b4093205930db5bebca05bece28ee95db6234
Contract Address: 0xa92fcb41e084320469D6dD8915735Df1D0eF686d
Block: 5762008
Paid: 0.003084676314636952 ETH (3084676 gas * 1.000000102 gwei)
```

```json
"arguments": [
    "\"Wrapped Subspace Token\"",
    "\"WTSSC\"",
    "0xc839Ea8baCB7604245c09fa32Dc0a80250663C6d",
    "0xB751710Af8Ce68677aB960adB103060f38d09714"
],
```

## 4. Send tokens from src chain

Run AutoBridge script: `script/auto_bridge.s.sol:AutoBridgeScript` mainly on src chain to send tokens to dst chain.

> The destination chain is mainly for setting peers.

1. set peers on both sides, if not/incorrectly set. Try to use TS script for this unless you want to debug using Solidity.

Arguments for Nova:

| Name | Type | Data
| --| -- | -- |
| _eid | uint32 | 40161 |
| _peer | bytes32 | 0x0000000000000000000000001ae29d803707aa8154aa82c4eb4eff8401a1b94b |

Arguments for Sepolia:

| Name | Type | Data
| --| -- | -- |
| _eid | uint32 | 490000 |
| _peer | bytes32 | 0x71a8a8fa7fba2e5f74d58dcb567ce87c46cceea60000000000000000000 |

1. `quoteSend`: get quote for sending tokens from src side.
2. `send`: send tokens from src side

Result of TS script is same as expected from Solidity script 🎉.

```sh
# Network:
# src: Nova Anvil, dst: Sepolia
❯ bun auto-bridge
$ tsc && node dist/demos/src/auto-bridge/index.js
Incorrect/No peer was set on Nova.
    So, correct peer set on Nova via tx hash: 0x830e318c1393a9c5b44f5bad5510857e0946b247e7257188919594cdd61f7460 in block #417815
Incorrect/No peer was set on Sepolia.
    So, correct peer set on Sepolia via tx hash: 0xb6d6aab3de65c9b6ca027a5c38e04730b4ef6e874823537df2cae98aafcb0789 in block #5761782

Deposited 10000000000000000 via tx hash: 0x629b0a827dd4cac99224ee5b4016bccf71e583d51ab701d873bb3ebc74a9e437 in block #417816
quote: 
 nativeFee:  0 
 lzFee:  0
token[0]'s total supply: 0.01 WTSSC
Tx hash for sending tokens from contract '0x8fF9...1720': 
    '0xef7e03e39f3d01c04e0cdfe08adcb78c5fda25435c4d3c1c3547a0fa9dab75fb' in block #417817
token[1]'s total supply: 0.0 WTSSC
```

<details><summary>Here is the Solidity log for reference:</summary>

```sh
❯ forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8545 --broadcast --legacy -vvvv
Traces:
  [230276] AutoBridgeScript::run()
    ├─ [0] VM::startBroadcast(0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   └─ ← [Return] 
    ├─ [2619] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::isPeer(40161 [4.016e4], 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d) [staticcall]
    │   └─ ← [Return] true
    ├─ [2664] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::balanceOf(0xB751710Af8Ce68677aB960adB103060f38d09714) [staticcall]
    │   └─ ← [Return] 0
    ├─ [0] console::log("===Before deposit:") [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::log("\tWTSSC Balance: ", 0) [staticcall]
    │   └─ ← [Stop] 
    ├─ [45874] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::deposit{value: 10000000000000000}()
    │   ├─ emit Transfer(from: 0x0000000000000000000000000000000000000000, to: 0xB751710Af8Ce68677aB960adB103060f38d09714, value: 10000000000000000 [1e16])
    │   ├─ emit Deposit(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, amount: 10000000000000000 [1e16])
    │   └─ ← [Stop] 
    ├─ [0] console::log("===After deposit:") [staticcall]
    │   └─ ← [Stop] 
    ├─ [664] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::balanceOf(0xB751710Af8Ce68677aB960adB103060f38d09714) [staticcall]
    │   └─ ← [Return] 10000000000000000 [1e16]
    ├─ [0] console::log("\tWTSSC Balance: ", 10000000000000000 [1e16]) [staticcall]
    │   └─ ← [Stop] 
    ├─ [109469] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::quoteSend(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), false) [staticcall]
    │   ├─ [97414] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::quote(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0x8fF90adA0487bC78A5E0E28463964A98ab041720) [staticcall]
    │   │   ├─ [83640] 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58::quote(Packet({ nonce: 6, srcEid: 490000 [4.9e5], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false) [staticcall]
    │   │   │   ├─ [30174] 0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28::getFee(40161 [4.016e4], 1, 0x8fF90adA0487bC78A5E0E28463964A98ab041720, 0x) [staticcall]
    │   │   │   │   ├─ [14533] 0xd3242Fe2085d3D7dC73De281D4c834EEA61447C6::getFee(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x) [staticcall]
    │   │   │   │   │   ├─ [8488] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeByEid(40161 [4.016e4], 452, 5000) [staticcall]
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ [25228] 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B::getFee(40161 [4.016e4], 0x8fF90adA0487bC78A5E0E28463964A98ab041720, 40, 0x0100110100000000000000000000000000030d40) [staticcall]
    │   │   │   │   ├─ [7582] 0xefcC10b4E00eb7a587c6e9bA0d418FcA4776435b::getFee(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40) [staticcall]
    │   │   │   │   │   ├─ [2488] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeByEid(40161 [4.016e4], 40, 205000 [2.05e5]) [staticcall]
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    ├─ [0] console::log("===Quote fee: ", 0) [staticcall]
    │   └─ ← [Stop] 
    ├─ [78458] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::send(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   ├─ emit Transfer(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, to: 0x0000000000000000000000000000000000000000, value: 10000000000000000 [1e16])
    │   ├─ [66614] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::send(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   │   ├─ [44397] 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58::send(Packet({ nonce: 6, srcEid: 490000 [4.9e5], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false)
    │   │   │   ├─ [12919] 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B::assignJob(40161 [4.016e4], 0x8fF90adA0487bC78A5E0E28463964A98ab041720, 40, 0x0100110100000000000000000000000000030d40)
    │   │   │   │   ├─ [7622] 0xefcC10b4E00eb7a587c6e9bA0d418FcA4776435b::getFeeOnSend(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40)
    │   │   │   │   │   ├─ [2437] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeOnSend(40161 [4.016e4], 40, 205000 [2.05e5])
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ emit ExecutorFeePaid(executor: 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B, fee: 0)
    │   │   │   ├─ [12291] 0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28::assignJob(AssignJobParam({ dstEid: 40161 [4.016e4], packetHeader: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, payloadHash: 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2, confirmations: 1, sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720 }), 0x)
    │   │   │   │   ├─ [6005] 0xd3242Fe2085d3D7dC73De281D4c834EEA61447C6::getFeeOnSend(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x)
    │   │   │   │   │   ├─ [2437] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeOnSend(40161 [4.016e4], 452, 5000)
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ emit DVNFeePaid(requiredDVNs: [0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28], optionalDVNs: [], fees: [0])
    │   │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710
    │   │   ├─ emit PacketSent(encodedPayload: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, sendLibrary: 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58)
    │   │   └─ ← [Return] MessagingReceipt({ guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, nonce: 6, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) })
    │   ├─ emit OFTSent(guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, dstEid: 40161 [4.016e4], fromAddress: 0xB751710Af8Ce68677aB960adB103060f38d09714, amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16])
    │   └─ ← [Return] MessagingReceipt({ guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, nonce: 6, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) }), OFTReceipt({ amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16] })
    ├─ [0] VM::stopBroadcast()
    │   └─ ← [Return] 
    └─ ← [Stop] 


Script ran successfully.

== Logs ==
  ===Before deposit:
    WTSSC Balance:  0
  ===After deposit:
    WTSSC Balance:  10000000000000000
  ===Quote fee:  0

## Setting up 1 EVM.
==========================
Simulated On-chain Traces:

  [47874] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::deposit{value: 10000000000000000}()
    ├─ emit Transfer(from: 0x0000000000000000000000000000000000000000, to: 0xB751710Af8Ce68677aB960adB103060f38d09714, value: 10000000000000000 [1e16])
    ├─ emit Deposit(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, amount: 10000000000000000 [1e16])
    └─ ← [Stop] 

  [145158] 0x8fF90adA0487bC78A5E0E28463964A98ab041720::send(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    ├─ emit Transfer(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, to: 0x0000000000000000000000000000000000000000, value: 10000000000000000 [1e16])
    ├─ [127614] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::send(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   ├─ [96897] 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58::send(Packet({ nonce: 6, srcEid: 490000 [4.9e5], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, dstEid: 40161 [4.016e4], receiver: 0x000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false)
    │   │   ├─ [33919] 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B::assignJob(40161 [4.016e4], 0x8fF90adA0487bC78A5E0E28463964A98ab041720, 40, 0x0100110100000000000000000000000000030d40)
    │   │   │   ├─ [16122] 0xefcC10b4E00eb7a587c6e9bA0d418FcA4776435b::getFeeOnSend(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40)
    │   │   │   │   ├─ [8437] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeOnSend(40161 [4.016e4], 40, 205000 [2.05e5])
    │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] 0
    │   │   ├─ emit ExecutorFeePaid(executor: 0x1ae29D803707aa8154aa82c4eb4Eff8401a1B94B, fee: 0)
    │   │   ├─ [24791] 0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28::assignJob(AssignJobParam({ dstEid: 40161 [4.016e4], packetHeader: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, payloadHash: 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2, confirmations: 1, sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720 }), 0x)
    │   │   │   ├─ [6005] 0xd3242Fe2085d3D7dC73De281D4c834EEA61447C6::getFeeOnSend(FeeParams({ priceFeed: 0xF602F10dc5Dc741Ed1299841B053948951C1a999, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x8fF90adA0487bC78A5E0E28463964A98ab041720, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x)
    │   │   │   │   ├─ [2437] 0xF602F10dc5Dc741Ed1299841B053948951C1a999::estimateFeeOnSend(40161 [4.016e4], 452, 5000)
    │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] 0
    │   │   ├─ emit DVNFeePaid(requiredDVNs: [0x31f9F1Ff259bE6f27d8C3B0ccb30f04dfd647D28], optionalDVNs: [], fees: [0])
    │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710
    │   ├─ emit PacketSent(encodedPayload: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, sendLibrary: 0xEDbC50168a397c4d28911Ccb80f5348090Cc2d58)
    │   └─ ← [Return] MessagingReceipt({ guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, nonce: 6, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) })
    ├─ emit OFTSent(guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, dstEid: 40161 [4.016e4], fromAddress: 0xB751710Af8Ce68677aB960adB103060f38d09714, amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16])
    └─ ← [Return] MessagingReceipt({ guid: 0x3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae, nonce: 6, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) }), OFTReceipt({ amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16] })


==========================

Chain 490000

Estimated gas price: 1.015530288 gwei

Estimated total gas used for script: 334160

Estimated amount required: 0.00033934960103808 ETH

==========================
##
Sending transactions [0 - 1].
⠉ [00:00:00] [##############################################################################################################################################] 2/2 txes (0.0s)##
Waiting for receipts.
⠙ [00:00:00] [##########################################################################################################################################] 2/2 receipts (0.0s)
##### 490000
✅  [Success]Hash: 0x22a22966e178c07334ce4d651630f83f00e1f5bac3212a855ce582b50802c44c
Block: 418237
Paid: 0.000070008626994144 ETH (68938 gas * 1.015530288 gwei)


##### 490000
✅  [Success]Hash: 0x220a23d30b1c34679d3fa75dddfb7d0b727c382db58a24c923514bbe2f8ac871
Block: 418238
Paid: 0.000171557593672992 ETH (168934 gas * 1.015530288 gwei)



==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
Total Paid: 0.000241566220667136 ETH (237872 gas * avg 1.015530288 gwei)
```

</details>

> NOTE: Native fee & LZToken fee both are zero.

- [ ] TODO: Need to diagnose that later via setting suitable values in `PriceFeed`.

## 5. Role of DVN, Executor

Listen to emitted events from src chain and extract message for verification & relaying message over to dst chain.

> Scripts available in both Solidity & TS. Use solidity only if you want to debug

### Using TS script

<details><summary>Log:</summary>

```sh
bun auto-bridge:dvn
$ tsc && node dist/demos/src/auto-bridge/dvn.js
Listening for emitted events from 0x8fF9...1720 on Nova...
DVN Fee Paid!
Transfer! Source
Executor Fee Paid!
====Packet Sent!
Encoded Packet Hex: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710}
OFT Sent!
Receiver MessageLib Address: 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64
Uln config: {"confirmations":"1","requiredDVNCount":"1","optionalDVNCount":"0","optionalDVNThreshold":"0","requiredDVNs":["0x71100123d19a2Cf1e007D4b2Fd45bF9eac6542E9"],"optionalDVNs":[]}
Header: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d
Payload Hash: 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2
Verify | tx hash: 0x6774bca102bc254b63ebce1534854cc7a0f82f1f3275f39a0a72e9c863256970 in block #5762183
/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/logger/lib/index.js:238
        var error = new Error(message);
                    ^

Error: transaction failed [ See: https://links.ethers.org/v5-errors-CALL_EXCEPTION ] (transactionHash="0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976", transaction={"type":2,"chainId":11155111,"nonce":212,"maxPriorityFeePerGas":{"type":"BigNumber","hex":"0x59682f00"},"maxFeePerGas":{"type":"BigNumber","hex":"0x59682f20"},"gasPrice":null,"gasLimit":{"type":"BigNumber","hex":"0x030d40"},"to":"0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64","value":{"type":"BigNumber","hex":"0x00"},"data":"0x0894edf100000000000000000000000000000000000000000000000000000000000000402c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2000000000000000000000000000000000000000000000000000000000000005101000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d000000000000000000000000000000","accessList":[],"hash":"0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976","v":1,"r":"0xbe78035b64be63f8c47fa959bd4076166b2336d148b4c511ec9b93e4a4b65aa6","s":"0x77a8ea2433742fbe2d4afa26babea176fc2d843c60b188efdafee8e73349c7bb","from":"0xB751710Af8Ce68677aB960adB103060f38d09714","confirmations":0}, receipt={"to":"0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64","from":"0xB751710Af8Ce68677aB960adB103060f38d09714","contractAddress":null,"transactionIndex":42,"gasUsed":{"type":"BigNumber","hex":"0xa94a"},"logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","blockHash":"0xe406c52a8bae2aa20b447b48820b1a6381c774c980d306ba2d61d29557203883","transactionHash":"0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976","logs":[],"blockNumber":5762184,"confirmations":1,"cumulativeGasUsed":{"type":"BigNumber","hex":"0x8d8bea"},"effectiveGasPrice":{"type":"BigNumber","hex":"0x59682f10"},"status":0,"type":2,"byzantium":true}, code=CALL_EXCEPTION, version=providers/5.7.2)
    at Logger.makeError (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/logger/lib/index.js:238:21)
    at Logger.throwError (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/logger/lib/index.js:247:20)
    at JsonRpcProvider.<anonymous> (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/providers/lib/base-provider.js:1693:36)
    at step (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/providers/lib/base-provider.js:48:23)
    at Object.next (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/providers/lib/base-provider.js:29:53)
    at fulfilled (/Users/abhi3700/F/coding/github_repos/subspace/layerzero-demo/demos/node_modules/@ethersproject/providers/lib/base-provider.js:20:58) {
  reason: 'transaction failed',
  code: 'CALL_EXCEPTION',
  transactionHash: '0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976',
  transaction: {
    type: 2,
    chainId: 11155111,
    nonce: 212,
    maxPriorityFeePerGas: BigNumber { _hex: '0x59682f00', _isBigNumber: true },
    maxFeePerGas: BigNumber { _hex: '0x59682f20', _isBigNumber: true },
    gasPrice: null,
    gasLimit: BigNumber { _hex: '0x030d40', _isBigNumber: true },
    to: '0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64',
    value: BigNumber { _hex: '0x00', _isBigNumber: true },
    data: '0x0894edf100000000000000000000000000000000000000000000000000000000000000402c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2000000000000000000000000000000000000000000000000000000000000005101000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d000000000000000000000000000000',
    accessList: [],
    hash: '0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976',
    v: 1,
    r: '0xbe78035b64be63f8c47fa959bd4076166b2336d148b4c511ec9b93e4a4b65aa6',
    s: '0x77a8ea2433742fbe2d4afa26babea176fc2d843c60b188efdafee8e73349c7bb',
    from: '0xB751710Af8Ce68677aB960adB103060f38d09714',
    confirmations: 0,
    wait: [Function (anonymous)]
  },
  receipt: {
    to: '0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64',
    from: '0xB751710Af8Ce68677aB960adB103060f38d09714',
    contractAddress: null,
    transactionIndex: 42,
    gasUsed: BigNumber { _hex: '0xa94a', _isBigNumber: true },
    logsBloom: '0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
    blockHash: '0xe406c52a8bae2aa20b447b48820b1a6381c774c980d306ba2d61d29557203883',
    transactionHash: '0x04a516fd732730c87fdca22be927e0c910b589a7b41efaec1c2f41282f714976',
    logs: [],
    blockNumber: 5762184,
    confirmations: 1,
    cumulativeGasUsed: BigNumber { _hex: '0x8d8bea', _isBigNumber: true },
    effectiveGasPrice: BigNumber { _hex: '0x59682f10', _isBigNumber: true },
    status: 0,
    type: 2,
    byzantium: true
  }
}

Node.js v18.18.0
error: script "auto-bridge:dvn" exited with code 1
```

</details>

So, far, the encodedPacket from emitted event `PacketSent` event is:

```sh
Encoded Packet Hex: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d3af32de1c8a11ba3d3248245141c13652026a4ecca27d8e8ebc104ab98fb5fae000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710}
```

And `ReceiveUln302.verify()` function passes ✅, whereas `ReceiveUln302.commitVerification()` fails ❌.

### Using Solidity script

In order to debug further as to why the commit verification fails, used AutoBridgeDVN script: `script/autobridge_dvn.s.sol:AutoBridgeDVNScript` script hardcoding `encodedPacket` value.

```sh
forge script script/autobridge_dvn.s.sol:AutoBridgeDVNScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL --broadcast -vvvv
```

<details><summary>Log:</summary>

```sh
Traces:
  [1484463] → new AutoBridgeDVNScript@0x5b73C5498c1E3b4dbA84de0F1833c4a029d90519
    └─ ← [Return] 7083 bytes of code

  [58135] AutoBridgeDVNScript::setUp()
    ├─ [0] VM::envUint("DEPLOYER_PRIVATE_KEY") [staticcall]
    │   └─ ← [Return] <env var value>
    ├─ [0] VM::addr(<pk>) [staticcall]
    │   └─ ← [Return] 0xB751710Af8Ce68677aB960adB103060f38d09714
    ├─ [0] VM::envAddress("SEPOLIA_ENDPOINT_V2") [staticcall]
    │   └─ ← [Return] <env var value>
    ├─ [0] VM::envAddress("WTSSCLZ_SEPOLIA") [staticcall]
    │   └─ ← [Return] <env var value>
    ├─ [316] 0xc839Ea8baCB7604245c09fa32Dc0a80250663C6d::eid() [staticcall]
    │   └─ ← [Return] 40161 [4.016e4]
    ├─ [0] VM::envUint("NOVA_ENDPOINT_V2_ID") [staticcall]
    │   └─ ← [Return] <env var value>
    └─ ← [Stop] 

  [87626] AutoBridgeDVNScript::run()
    ├─ [0] VM::startBroadcast(0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   └─ ← [Return] 
    ├─ [2614] 0xc839Ea8baCB7604245c09fa32Dc0a80250663C6d::defaultReceiveLibrary(490000 [4.9e5]) [staticcall]
    │   └─ ← [Return] 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64
    ├─ [11225] 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64::getUlnConfig(0xa92fcb41e084320469D6dD8915735Df1D0eF686d, 490000 [4.9e5]) [staticcall]
    │   └─ ← [Return] UlnConfig({ confirmations: 1, requiredDVNCount: 1, optionalDVNCount: 0, optionalDVNThreshold: 0, requiredDVNs: [0x71100123d19a2Cf1e007D4b2Fd45bF9eac6542E9], optionalDVNs: [] })
    ├─ [0] console::log("Confirmations: ", 1) [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::log("Packet Header:") [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::logBytes(0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d) [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::log("Payload hash:") [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::logBytes32(0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2) [staticcall]
    │   └─ ← [Stop] 
    ├─ [6606] 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64::verify(0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2, 1)
    │   ├─ emit PayloadVerified(dvn: 0xB751710Af8Ce68677aB960adB103060f38d09714, header: 0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, confirmations: 1, proofHash: 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2)
    │   └─ ← [Stop] 
    ├─ [0] console::log("Packet header hash") [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::logBytes32(0xef000b28d63de81d5f6693c6f38a37cfba1d80e176b2082620f11bd7b21a02d6) [staticcall]
    │   └─ ← [Stop] 
    ├─ [8486] 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64::verifiable(UlnConfig({ confirmations: 1, requiredDVNCount: 1, optionalDVNCount: 0, optionalDVNThreshold: 0, requiredDVNs: [0x71100123d19a2Cf1e007D4b2Fd45bF9eac6542E9], optionalDVNs: [] }), 0xef000b28d63de81d5f6693c6f38a37cfba1d80e176b2082620f11bd7b21a02d6, 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2) [staticcall]
    │   ├─ [0] console::log("inside verifiable function") [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("==before-_config.requiredDVNCount") [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("==outside-if-verified", 0) [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("_verified() fn: ", false, 0, 1) [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("==inside-if-verified", 0) [staticcall]
    │   │   └─ ← [Stop] 
    │   └─ ← [Return] false
    ├─ [8034] 0x9D94d3370b57186a765Ef8FD8B7193778A0EcE64::commitVerification(0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d, 0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2)
    │   ├─ [0] console::log("==before-_config.requiredDVNCount") [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("==outside-if-verified", 0) [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("_verified() fn: ", false, 0, 1) [staticcall]
    │   │   └─ ← [Stop] 
    │   ├─ [0] console::log("==inside-if-verified", 0) [staticcall]
    │   │   └─ ← [Stop] 
    │   └─ ← [Revert] LZ_ULN_Verifying()
    └─ ← [Revert] LZ_ULN_Verifying()



== Logs ==
  Confirmations:  1
  Packet Header:
  0x01000000000000000600077a100000000000000000000000008ff90ada0487bc78a5e0e28463964a98ab04172000009ce1000000000000000000000000a92fcb41e084320469d6dd8915735df1d0ef686d
  Payload hash:
  0x2c35d5908540a7f6e12646e4e4b7a2e39602ced477ac71feb883b626563e82d2
  Packet header hash
  0xef000b28d63de81d5f6693c6f38a37cfba1d80e176b2082620f11bd7b21a02d6
  inside verifiable function
  ==before-_config.requiredDVNCount
  ==outside-if-verified 0
  _verified() fn:  false 0 1
  ==inside-if-verified 0
  ==before-_config.requiredDVNCount
  ==outside-if-verified 0
  _verified() fn:  false 0 1
  ==inside-if-verified 0
Error: 
script failed: LZ_ULN_Verifying()
```

</details>

As the entire solidity script failed with error: `LZ_ULN_Verifying`, so successful txs are not broadcasted. If `commitVerification()` setter function call is commented, then you get to see the `verify()` function call tx hash. Here is a `verify()` tx made on sepolia after commenting:

```sh
##### sepolia
✅  [Success]Hash: 0x360a852d3688bcb75ab0d48744961dc3484e8c1f76ef1728e04a7f7e81586d63
Block: 5755016
Paid: 0.00004403512881975 ETH (49450 gas * 0.890498055 gwei)
```

Another thing to notice is the intermediary logs:

```sh
==before-_config.requiredDVNCount
==outside-if-verified 0
==inside-if-verified 0
```

This is achieved by inserting `console2.log` in the dependency functions like this:

<details><summary>Code:</summary>

```solidity

    function verifiable(UlnConfig memory _config, bytes32 _headerHash, bytes32 _payloadHash)
        external
        view
        returns (bool)
    {
        console2.log("inside verifiable function");
        return _checkVerifiable(_config, _headerHash, _payloadHash);
    }

    function _checkVerifiable(UlnConfig memory _config, bytes32 _headerHash, bytes32 _payloadHash)
        internal
        view
        returns (bool)
    {
        console2.log("==before-_config.requiredDVNCount");
        // iterate the required DVNs
        if (_config.requiredDVNCount > 0) {
            for (uint8 i = 0; i < _config.requiredDVNCount; ++i) {
                console2.log("==outside-if-verified", i);
                if (!_verified(_config.requiredDVNs[i], _headerHash, _payloadHash, _config.confirmations)) {
                    // return if any of the required DVNs haven't signed
                    console2.log("==inside-if-verified", i);
                    return false;
                }
            }
            if (_config.optionalDVNCount == 0) {
                // returns early if all required DVNs have signed and there are no optional DVNs
                return true;
            }
        }
        console2.log("==after-_config.requiredDVNCount");

        // then it must require optional validations
        uint8 threshold = _config.optionalDVNThreshold;
        for (uint8 i = 0; i < _config.optionalDVNCount; ++i) {
            if (_verified(_config.optionalDVNs[i], _headerHash, _payloadHash, _config.confirmations)) {
                // increment the optional count if the optional DVN has signed
                threshold--;
                if (threshold == 0) {
                    // early return if the optional threshold has hit
                    return true;
                }
            }
        }

        // return false as a catch-all
        return false;
    }
}
```

</details>

Still incomplete!

Items in TODO:

- [ ] Add Executor role
- [ ] Confirm Packet if received.
