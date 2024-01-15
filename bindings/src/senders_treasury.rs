pub use senders_treasury::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod senders_treasury {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("shutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shutdown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SENDERSTREASURY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x03T\x80a\0%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xA9\n\xE8\x87\x14a\0;W\x80c\xFC\x0Et\xD1\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02ZV[a\0XV[\0[a\0Na\x01|V[`\0\x82\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\0tW`\0\x80\xFD[`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U\x81Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x90\x81\x1B\x82\x16\x83\x85\x01R`4\x83\x01\x89\x90R`T\x83\x01\x88\x90R0\x90\x1B\x16`t\x82\x01R\x82Q\x80\x82\x03`h\x01\x81R`\x88\x82\x01\x84R\x80Q\x90\x83\x01 \x7F\x19Auto Request Payments:\n32\0\0\0\0\0\0`\xA8\x83\x01R`\xC2\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xE2\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90T`\x01`\x01`\xA0\x1B\x03\x16a\x015\x82\x84a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01HW`\0\x80\xFD[`@Q3\x90\x85\x15a\x08\xFC\x02\x90\x86\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01uW=`\0\x80>=`\0\xFD[PPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x93W`\0\x80\xFD[3\xFF[`\0\x80`\0\x80a\x01\xA5\x85a\x02\x15V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x02\0W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[`\0\x80`\0\x83Q`A\x14a\x02(W`\0\x80\xFD[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02oW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x95W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\xA9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xBBWa\x02\xBBa\x02DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xE3Wa\x02\xE3a\x02DV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02\xFCW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \xFA\x053\x94\xC1\x1B\xBC;\xD1\x18q\xF9\xE7\xEC\xB2O\xDB\xAE\x08\x96\xA2\x0E\x1Av:\x87\x9E!\xA1\xC1.\xD3dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SENDERSTREASURY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xA9\n\xE8\x87\x14a\0;W\x80c\xFC\x0Et\xD1\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02ZV[a\0XV[\0[a\0Na\x01|V[`\0\x82\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\0tW`\0\x80\xFD[`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U\x81Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x90\x81\x1B\x82\x16\x83\x85\x01R`4\x83\x01\x89\x90R`T\x83\x01\x88\x90R0\x90\x1B\x16`t\x82\x01R\x82Q\x80\x82\x03`h\x01\x81R`\x88\x82\x01\x84R\x80Q\x90\x83\x01 \x7F\x19Auto Request Payments:\n32\0\0\0\0\0\0`\xA8\x83\x01R`\xC2\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xE2\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90T`\x01`\x01`\xA0\x1B\x03\x16a\x015\x82\x84a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01HW`\0\x80\xFD[`@Q3\x90\x85\x15a\x08\xFC\x02\x90\x86\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01uW=`\0\x80>=`\0\xFD[PPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x93W`\0\x80\xFD[3\xFF[`\0\x80`\0\x80a\x01\xA5\x85a\x02\x15V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x02\0W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[`\0\x80`\0\x83Q`A\x14a\x02(W`\0\x80\xFD[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02oW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x95W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\xA9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xBBWa\x02\xBBa\x02DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xE3Wa\x02\xE3a\x02DV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02\xFCW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \xFA\x053\x94\xC1\x1B\xBC;\xD1\x18q\xF9\xE7\xEC\xB2O\xDB\xAE\x08\x96\xA2\x0E\x1Av:\x87\x9E!\xA1\xC1.\xD3dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SENDERSTREASURY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SendersTreasury<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SendersTreasury<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SendersTreasury<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SendersTreasury<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SendersTreasury<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SendersTreasury))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SendersTreasury<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SENDERSTREASURY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SENDERSTREASURY_ABI.clone(),
                SENDERSTREASURY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `claimPayment` (0xa90ae887) function
        pub fn claim_payment(
            &self,
            amount: ::ethers::core::types::U256,
            nonce: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 10, 232, 135], (amount, nonce, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdown` (0xfc0e74d1) function
        pub fn shutdown(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SendersTreasury<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `claimPayment` function with signature `claimPayment(uint256,uint256,bytes)` and selector `0xa90ae887`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimPayment", abi = "claimPayment(uint256,uint256,bytes)")]
    pub struct ClaimPaymentCall {
        pub amount: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `shutdown` function with signature `shutdown()` and selector `0xfc0e74d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "shutdown", abi = "shutdown()")]
    pub struct ShutdownCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum SendersTreasuryCalls {
        ClaimPayment(ClaimPaymentCall),
        Shutdown(ShutdownCall),
    }
    impl ::ethers::core::abi::AbiDecode for SendersTreasuryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimPayment(decoded));
            }
            if let Ok(decoded) = <ShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Shutdown(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SendersTreasuryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Shutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SendersTreasuryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::Shutdown(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimPaymentCall> for SendersTreasuryCalls {
        fn from(value: ClaimPaymentCall) -> Self {
            Self::ClaimPayment(value)
        }
    }
    impl ::core::convert::From<ShutdownCall> for SendersTreasuryCalls {
        fn from(value: ShutdownCall) -> Self {
            Self::Shutdown(value)
        }
    }
}
