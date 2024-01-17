pub use fund::*;
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
pub mod fund {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("transferTsscToMany"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferTsscToMany"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tos"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientFundsInContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientFundsInContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReceiverAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidReceiverAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroTSSC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroTSSC"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FUND_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\x85\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c\xAE\xE1\x92?\x14a\0.W`\0\x80\xFD[6a\0)W\0[`\0\x80\xFD[a\0Aa\0<6`\x04a\x02+V[a\0CV[\0[`\0\x81Q4a\0R\x91\x90a\x02\xF0V[\x90P\x80`\0\x03a\0uW`@Qcn\xA7\x88\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x82Q\x81\x10\x15a\0\xB4Wa\0\xA4\x83\x82\x81Q\x81\x10a\0\x96Wa\0\x96a\x03\x12V[` \x02` \x01\x01Q\x83a\0\xB9V[a\0\xAD\x81a\x03(V[\x90Pa\0xV[PPPV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80a\0\xD7WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x15a\0\xF5W`@QcP/\xFA?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x01\x16W`@Qc\x05\xB4\xEEa`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01cW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01hV[``\x91P[PP\x90P\x80a\x01\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs#0\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x10\"\xBA42\xB9`a\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x903\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02&W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x02>W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02VW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02jW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02|Wa\x02|a\x01\xF9V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x02\xA1Wa\x02\xA1a\x01\xF9V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x02\xBFW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x02\xE4Wa\x02\xD5\x85a\x02\x0FV[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x02\xC4V[\x98\x97PPPPPPPPV[`\0\x82a\x03\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x03HWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFA\0\xF8\xB4\x08\x04\xBCQ\x88\xDEae<\x86-G\xC2\x9FTr8]\xEEv3u\x11Q\xE3\xBC+\xB2dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static FUND_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c\xAE\xE1\x92?\x14a\0.W`\0\x80\xFD[6a\0)W\0[`\0\x80\xFD[a\0Aa\0<6`\x04a\x02+V[a\0CV[\0[`\0\x81Q4a\0R\x91\x90a\x02\xF0V[\x90P\x80`\0\x03a\0uW`@Qcn\xA7\x88\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x82Q\x81\x10\x15a\0\xB4Wa\0\xA4\x83\x82\x81Q\x81\x10a\0\x96Wa\0\x96a\x03\x12V[` \x02` \x01\x01Q\x83a\0\xB9V[a\0\xAD\x81a\x03(V[\x90Pa\0xV[PPPV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80a\0\xD7WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x15a\0\xF5W`@QcP/\xFA?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x01\x16W`@Qc\x05\xB4\xEEa`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01cW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01hV[``\x91P[PP\x90P\x80a\x01\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs#0\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x10\"\xBA42\xB9`a\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x903\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02&W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x02>W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02VW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02jW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02|Wa\x02|a\x01\xF9V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x02\xA1Wa\x02\xA1a\x01\xF9V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x02\xBFW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x02\xE4Wa\x02\xD5\x85a\x02\x0FV[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x02\xC4V[\x98\x97PPPPPPPPV[`\0\x82a\x03\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x03HWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFA\0\xF8\xB4\x08\x04\xBCQ\x88\xDEae<\x86-G\xC2\x9FTr8]\xEEv3u\x11Q\xE3\xBC+\xB2dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static FUND_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Fund<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Fund<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Fund<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Fund<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Fund<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Fund)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Fund<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FUND_ABI.clone(),
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
                FUND_ABI.clone(),
                FUND_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `transferTsscToMany` (0xaee1923f) function
        pub fn transfer_tssc_to_many(
            &self,
            tos: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 225, 146, 63], tos)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Fund<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientFundsInContract` with signature `InsufficientFundsInContract()` and selector `0xdd4f1102`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InsufficientFundsInContract",
        abi = "InsufficientFundsInContract()"
    )]
    pub struct InsufficientFundsInContract;
    ///Custom Error type `InvalidReceiverAddress` with signature `InvalidReceiverAddress()` and selector `0xa05ff47e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidReceiverAddress", abi = "InvalidReceiverAddress()")]
    pub struct InvalidReceiverAddress;
    ///Custom Error type `ZeroTSSC` with signature `ZeroTSSC()` and selector `0x2da77308`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroTSSC", abi = "ZeroTSSC()")]
    pub struct ZeroTSSC;
    ///Container type for all of the contract's custom errors
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
    pub enum FundErrors {
        InsufficientFundsInContract(InsufficientFundsInContract),
        InvalidReceiverAddress(InvalidReceiverAddress),
        ZeroTSSC(ZeroTSSC),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FundErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InsufficientFundsInContract as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientFundsInContract(decoded));
            }
            if let Ok(decoded) = <InvalidReceiverAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReceiverAddress(decoded));
            }
            if let Ok(decoded) = <ZeroTSSC as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroTSSC(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FundErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientFundsInContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReceiverAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroTSSC(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FundErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientFundsInContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReceiverAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroTSSC as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FundErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientFundsInContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReceiverAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroTSSC(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FundErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientFundsInContract> for FundErrors {
        fn from(value: InsufficientFundsInContract) -> Self {
            Self::InsufficientFundsInContract(value)
        }
    }
    impl ::core::convert::From<InvalidReceiverAddress> for FundErrors {
        fn from(value: InvalidReceiverAddress) -> Self {
            Self::InvalidReceiverAddress(value)
        }
    }
    impl ::core::convert::From<ZeroTSSC> for FundErrors {
        fn from(value: ZeroTSSC) -> Self {
            Self::ZeroTSSC(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferTsscToMany` function with signature `transferTsscToMany(address[])` and selector `0xaee1923f`
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
    #[ethcall(name = "transferTsscToMany", abi = "transferTsscToMany(address[])")]
    pub struct TransferTsscToManyCall {
        pub tos: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}
