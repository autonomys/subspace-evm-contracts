pub use load::*;
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
pub mod load {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("arr1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("arr1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factorial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factorial"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("num"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
    pub static LOAD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\xC3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80ci\x87\xB1\xFB\x14a\0FW\x80c\x83qH4\x14a\0kW\x80c\xFB\x833\xEB\x14a\0~W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x01\xFCV[a\0\x93V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Ya\0y6`\x04a\x01\xFCV[a\0\xB4V[a\0\x91a\0\x8C6`\x04a\x01\xFCV[a\0\xE4V[\0[`\0\x81\x81T\x81\x10a\0\xA3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\0`\x01\x80[\x83\x81\x11a\0\xDDWa\0\xCB\x81\x83a\x02+V[\x91Pa\0\xD6\x81a\x02HV[\x90Pa\0\xBAV[P\x92\x91PPV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xFFWa\0\xFFa\x02aV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01(W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x01\x83W\x80\x80a\x01B\x81\x80a\x02+V[a\x01L\x91\x90a\x02+V[a\x01V\x91\x90a\x02+V[\x82\x82\x81Q\x81\x10a\x01hWa\x01ha\x02wV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x01|\x81a\x02HV[\x90Pa\x01.V[P\x80Qa\x01\x97\x90`\0\x90` \x84\x01\x90a\x01\x9CV[PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x01\xD7W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01\xD7W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x01\xBCV[Pa\x01\xE3\x92\x91Pa\x01\xE7V[P\x90V[[\x80\x82\x11\x15a\x01\xE3W`\0\x81U`\x01\x01a\x01\xE8V[`\0` \x82\x84\x03\x12\x15a\x02\x0EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02BWa\x02Ba\x02\x15V[\x92\x91PPV[`\0`\x01\x82\x01a\x02ZWa\x02Za\x02\x15V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xE0\xC8O\x19\x8DmdT}\xBAZ'V\xF2P>z\x83/\xAA\r#<\xA5\x9Cf\x9C\xBCQ*vEdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LOAD_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80ci\x87\xB1\xFB\x14a\0FW\x80c\x83qH4\x14a\0kW\x80c\xFB\x833\xEB\x14a\0~W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x01\xFCV[a\0\x93V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Ya\0y6`\x04a\x01\xFCV[a\0\xB4V[a\0\x91a\0\x8C6`\x04a\x01\xFCV[a\0\xE4V[\0[`\0\x81\x81T\x81\x10a\0\xA3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\0`\x01\x80[\x83\x81\x11a\0\xDDWa\0\xCB\x81\x83a\x02+V[\x91Pa\0\xD6\x81a\x02HV[\x90Pa\0\xBAV[P\x92\x91PPV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xFFWa\0\xFFa\x02aV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01(W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x01\x83W\x80\x80a\x01B\x81\x80a\x02+V[a\x01L\x91\x90a\x02+V[a\x01V\x91\x90a\x02+V[\x82\x82\x81Q\x81\x10a\x01hWa\x01ha\x02wV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x01|\x81a\x02HV[\x90Pa\x01.V[P\x80Qa\x01\x97\x90`\0\x90` \x84\x01\x90a\x01\x9CV[PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x01\xD7W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01\xD7W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x01\xBCV[Pa\x01\xE3\x92\x91Pa\x01\xE7V[P\x90V[[\x80\x82\x11\x15a\x01\xE3W`\0\x81U`\x01\x01a\x01\xE8V[`\0` \x82\x84\x03\x12\x15a\x02\x0EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02BWa\x02Ba\x02\x15V[\x92\x91PPV[`\0`\x01\x82\x01a\x02ZWa\x02Za\x02\x15V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xE0\xC8O\x19\x8DmdT}\xBAZ'V\xF2P>z\x83/\xAA\r#<\xA5\x9Cf\x9C\xBCQ*vEdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LOAD_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Load<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Load<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Load<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Load<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Load<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Load)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Load<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOAD_ABI.clone(),
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
                LOAD_ABI.clone(),
                LOAD_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `arr1` (0x6987b1fb) function
        pub fn arr_1(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 135, 177, 251], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factorial` (0x83714834) function
        pub fn factorial(
            &self,
            num: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 113, 72, 52], num)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setArray` (0xfb8333eb) function
        pub fn set_array(
            &self,
            count: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 131, 51, 235], count)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Load<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `arr1` function with signature `arr1(uint256)` and selector `0x6987b1fb`
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
    #[ethcall(name = "arr1", abi = "arr1(uint256)")]
    pub struct Arr1Call(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `factorial` function with signature `factorial(uint256)` and selector `0x83714834`
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
    #[ethcall(name = "factorial", abi = "factorial(uint256)")]
    pub struct FactorialCall {
        pub num: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setArray` function with signature `setArray(uint256)` and selector `0xfb8333eb`
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
    #[ethcall(name = "setArray", abi = "setArray(uint256)")]
    pub struct SetArrayCall {
        pub count: ::ethers::core::types::U256,
    }
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
    pub enum LoadCalls {
        Arr1(Arr1Call),
        Factorial(FactorialCall),
        SetArray(SetArrayCall),
    }
    impl ::ethers::core::abi::AbiDecode for LoadCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Arr1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Arr1(decoded));
            }
            if let Ok(decoded) = <FactorialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factorial(decoded));
            }
            if let Ok(decoded) = <SetArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetArray(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LoadCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Arr1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factorial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LoadCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Arr1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factorial(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetArray(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Arr1Call> for LoadCalls {
        fn from(value: Arr1Call) -> Self {
            Self::Arr1(value)
        }
    }
    impl ::core::convert::From<FactorialCall> for LoadCalls {
        fn from(value: FactorialCall) -> Self {
            Self::Factorial(value)
        }
    }
    impl ::core::convert::From<SetArrayCall> for LoadCalls {
        fn from(value: SetArrayCall) -> Self {
            Self::SetArray(value)
        }
    }
    ///Container type for all return fields from the `arr1` function with signature `arr1(uint256)` and selector `0x6987b1fb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Arr1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `factorial` function with signature `factorial(uint256)` and selector `0x83714834`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FactorialReturn(pub ::ethers::core::types::U256);
}
