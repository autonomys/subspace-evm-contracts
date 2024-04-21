❯ forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url http://127.0.0.1:8545 --broadcast --legacy -vvvv
[⠒] Compiling...
[⠒] Compiling 1 files with 0.8.24
[⠑] Solc 0.8.24 finished in 1.40s
Compiler run successful!
Traces:
  [229285] AutoBridgeScript::run()
    ├─ [0] VM::startBroadcast(0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   └─ ← [Return] 
    ├─ [2619] 0x378E37eb673FB0604AAAd644C8813e084c38Ab41::isPeer(40161 [4.016e4], 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff) [staticcall]
    │   └─ ← [Return] true
    ├─ [2664] 0x378E37eb673FB0604AAAd644C8813e084c38Ab41::balanceOf(0xB751710Af8Ce68677aB960adB103060f38d09714) [staticcall]
    │   └─ ← [Return] 10000000000000000 [1e16]
    ├─ [0] console::log("===Before deposit:") [staticcall]
    │   └─ ← [Stop] 
    ├─ [0] console::log("\tWTSSC Balance: ", 10000000000000000 [1e16]) [staticcall]
    │   └─ ← [Stop] 
    ├─ [109469] 0x378E37eb673FB0604AAAd644C8813e084c38Ab41::quoteSend(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), false) [staticcall]
    │   ├─ [97414] 0x302Bfc9057f7762fe47C6Deb846aD777E8f87E26::quote(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0x378E37eb673FB0604AAAd644C8813e084c38Ab41) [staticcall]
    │   │   ├─ [83640] 0xD133f9da046dc260F5b1BB2dE7B9BB8d6Ef39b88::quote(Packet({ nonce: 1, srcEid: 490000 [4.9e5], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false) [staticcall]
    │   │   │   ├─ [30174] 0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B::getFee(40161 [4.016e4], 1, 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, 0x) [staticcall]
    │   │   │   │   ├─ [14533] 0xe7bC84fA427E954Ef0f6E8ed09c13B7a13ac4330::getFee(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x) [staticcall]
    │   │   │   │   │   ├─ [8488] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeByEid(40161 [4.016e4], 452, 5000) [staticcall]
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ [25228] 0x7063240061be3ABEde39DC3fd0c4CfFEE4061497::getFee(40161 [4.016e4], 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, 40, 0x0100110100000000000000000000000000030d40) [staticcall]
    │   │   │   │   ├─ [7582] 0x56Ec18E6cfdf441a39F8b5c7ec9620EEFd12d010::getFee(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40) [staticcall]
    │   │   │   │   │   ├─ [2488] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeByEid(40161 [4.016e4], 40, 205000 [2.05e5]) [staticcall]
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 })
    ├─ [0] console::log("===Quote fee: ", 0) [staticcall]
    │   └─ ← [Stop] 
    ├─ [103158] 0x378E37eb673FB0604AAAd644C8813e084c38Ab41::send(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   ├─ emit Transfer(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, to: 0x0000000000000000000000000000000000000000, value: 10000000000000000 [1e16])
    │   ├─ [83714] 0x302Bfc9057f7762fe47C6Deb846aD777E8f87E26::send(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   │   ├─ [44397] 0xD133f9da046dc260F5b1BB2dE7B9BB8d6Ef39b88::send(Packet({ nonce: 1, srcEid: 490000 [4.9e5], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false)
    │   │   │   ├─ [12919] 0x7063240061be3ABEde39DC3fd0c4CfFEE4061497::assignJob(40161 [4.016e4], 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, 40, 0x0100110100000000000000000000000000030d40)
    │   │   │   │   ├─ [7622] 0x56Ec18E6cfdf441a39F8b5c7ec9620EEFd12d010::getFeeOnSend(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40)
    │   │   │   │   │   ├─ [2437] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeOnSend(40161 [4.016e4], 40, 205000 [2.05e5])
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ emit ExecutorFeePaid(executor: 0x7063240061be3ABEde39DC3fd0c4CfFEE4061497, fee: 0)
    │   │   │   ├─ [12291] 0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B::assignJob(AssignJobParam({ dstEid: 40161 [4.016e4], packetHeader: 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, payloadHash: 0xd6176822695c46292532b5f547f52ef225ca614809ec37de812b93ca9cd78d61, confirmations: 1, sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41 }), 0x)
    │   │   │   │   ├─ [6005] 0xe7bC84fA427E954Ef0f6E8ed09c13B7a13ac4330::getFeeOnSend(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x)
    │   │   │   │   │   ├─ [2437] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeOnSend(40161 [4.016e4], 452, 5000)
    │   │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   │   └─ ← [Return] 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   ├─ emit DVNFeePaid(requiredDVNs: [0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B], optionalDVNs: [], fees: [0])
    │   │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710
    │   │   ├─ emit PacketSent(encodedPayload: 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, sendLibrary: 0xD133f9da046dc260F5b1BB2dE7B9BB8d6Ef39b88)
    │   │   └─ ← [Return] MessagingReceipt({ guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, nonce: 1, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) })
    │   ├─ emit OFTSent(guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, dstEid: 40161 [4.016e4], fromAddress: 0xB751710Af8Ce68677aB960adB103060f38d09714, amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16])
    │   └─ ← [Return] MessagingReceipt({ guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, nonce: 1, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) }), OFTReceipt({ amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16] })
    ├─ [0] VM::stopBroadcast()
    │   └─ ← [Return] 
    └─ ← [Stop] 


Script ran successfully.

== Logs ==
  ===Before deposit:
    WTSSC Balance:  10000000000000000
  ===Quote fee:  0

## Setting up 1 EVM.
==========================
Simulated On-chain Traces:

  [162258] 0x378E37eb673FB0604AAAd644C8813e084c38Ab41::send(SendParam({ dstEid: 40161 [4.016e4], to: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d09714, amountLD: 10000000000000000 [1e16], minAmountLD: 10000000000000000 [1e16], extraOptions: 0x00030100110100000000000000000000000000030d40, composeMsg: 0x, oftCmd: 0x }), MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    ├─ emit Transfer(from: 0xB751710Af8Ce68677aB960adB103060f38d09714, to: 0x0000000000000000000000000000000000000000, value: 10000000000000000 [1e16])
    ├─ [144714] 0x302Bfc9057f7762fe47C6Deb846aD777E8f87E26::send(MessagingParams({ dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, payInLzToken: false }), 0xB751710Af8Ce68677aB960adB103060f38d09714)
    │   ├─ [96897] 0xD133f9da046dc260F5b1BB2dE7B9BB8d6Ef39b88::send(Packet({ nonce: 1, srcEid: 490000 [4.9e5], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, dstEid: 40161 [4.016e4], receiver: 0x00000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, message: 0x000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710 }), 0x00030100110100000000000000000000000000030d40, false)
    │   │   ├─ [33919] 0x7063240061be3ABEde39DC3fd0c4CfFEE4061497::assignJob(40161 [4.016e4], 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, 40, 0x0100110100000000000000000000000000030d40)
    │   │   │   ├─ [16122] 0x56Ec18E6cfdf441a39F8b5c7ec9620EEFd12d010::getFeeOnSend(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, calldataSize: 40, defaultMultiplierBps: 12000 [1.2e4] }), DstConfig({ baseGas: 5000, multiplierBps: 10000 [1e4], floorMarginUSD: 10000000000 [1e10], nativeCap: 1000000000 [1e9] }), 0x0100110100000000000000000000000000030d40)
    │   │   │   │   ├─ [8437] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeOnSend(40161 [4.016e4], 40, 205000 [2.05e5])
    │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] 0
    │   │   ├─ emit ExecutorFeePaid(executor: 0x7063240061be3ABEde39DC3fd0c4CfFEE4061497, fee: 0)
    │   │   ├─ [24791] 0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B::assignJob(AssignJobParam({ dstEid: 40161 [4.016e4], packetHeader: 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff, payloadHash: 0xd6176822695c46292532b5f547f52ef225ca614809ec37de812b93ca9cd78d61, confirmations: 1, sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41 }), 0x)
    │   │   │   ├─ [6005] 0xe7bC84fA427E954Ef0f6E8ed09c13B7a13ac4330::getFeeOnSend(FeeParams({ priceFeed: 0x8434BBFebA81d93469E97ab888089d65e84e1fd5, dstEid: 40161 [4.016e4], confirmations: 1, sender: 0x378E37eb673FB0604AAAd644C8813e084c38Ab41, quorum: 1, defaultMultiplierBps: 100 }), DstConfig({ gas: 5000, multiplierBps: 0, floorMarginUSD: 0 }), 0x)
    │   │   │   │   ├─ [2437] 0x8434BBFebA81d93469E97ab888089d65e84e1fd5::estimateFeeOnSend(40161 [4.016e4], 452, 5000)
    │   │   │   │   │   └─ ← [Return] 0, 0, 100000000000000000000 [1e20], 0
    │   │   │   │   └─ ← [Return] 0
    │   │   │   └─ ← [Return] 0
    │   │   ├─ emit DVNFeePaid(requiredDVNs: [0x69B388ea53111B351a541bA3C5e9f6766e8eBB5B], optionalDVNs: [], fees: [0])
    │   │   └─ ← [Return] MessagingFee({ nativeFee: 0, lzTokenFee: 0 }), 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710
    │   ├─ emit PacketSent(encodedPayload: 0x01000000000000000100077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ff33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710, options: 0x00030100110100000000000000000000000000030d40, sendLibrary: 0xD133f9da046dc260F5b1BB2dE7B9BB8d6Ef39b88)
    │   └─ ← [Return] MessagingReceipt({ guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, nonce: 1, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) })
    ├─ emit OFTSent(guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, dstEid: 40161 [4.016e4], fromAddress: 0xB751710Af8Ce68677aB960adB103060f38d09714, amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16])
    └─ ← [Return] MessagingReceipt({ guid: 0x33346b0e17afd131fe056b1567b00471c1e36b84971c490a2fb323a4fe223cf9, nonce: 1, fee: MessagingFee({ nativeFee: 0, lzTokenFee: 0 }) }), OFTReceipt({ amountSentLD: 10000000000000000 [1e16], amountReceivedLD: 10000000000000000 [1e16] })


==========================

Chain 490000

Estimated gas price: 1.059033548 gwei

Estimated total gas used for script: 272074

Estimated amount required: 0.000288135493538552 ETH

==========================
##
Sending transactions [0 - 0].
⠁ [00:00:00] [#################################################################################################################################] 1/1 txes (0.0s)##
Waiting for receipts.
⠉ [00:00:00] [#############################################################################################################################] 1/1 receipts (0.0s)
##### 490000
✅  [Success]Hash: 0x74af694af850206133c355397dfd971a9a728de03c9d230391428b63c0b99d66
Block: 373112
Paid: 0.000197016247068632 ETH (186034 gas * 1.059033548 gwei)



==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
Total Paid: 0.000197016247068632 ETH (186034 gas * avg 1.059033548 gwei)

Transactions saved to: /Users/abhi3700/F/coding/github_repos/subspace/subspace-evm-contracts/broadcast/auto_bridge.s.sol/490000/run-latest.json

Sensitive values saved to: /Users/abhi3700/F/coding/github_repos/subspace/subspace-evm-contracts/cache/auto_bridge.s.sol/490000/run-latest.json
