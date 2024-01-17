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
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("claimPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestId"),
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
                (
                    ::std::borrow::ToOwned::to_owned("constructMessageOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("constructMessageOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBalanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getPaymentRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPaymentRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("statusCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum SendersTreasury.PayRequestCode",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("getRequestedPayIdsOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRequestedPayIdsOf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestPayId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestPayId"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("requestPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("signPayReq"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signPayReq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestId"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callerSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PayRequestSigned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PayRequestSigned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("requestPayId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PaymentDone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PaymentDone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("requestPayId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                (
                    ::std::borrow::ToOwned::to_owned("PaymentRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PaymentRequested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("requestPayId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("CallerIsNotReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerIsNotReceiver",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallerIsNotSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerIsNotSender"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalanceOf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidDepositAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidRequestId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidRequestId"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SENDERSTREASURY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\x02U4\x80\x15a\0\x15W`\0\x80\xFD[Pa\x11\x8E\x80a\0%`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c\x9F\x0CY.\x11a\0dW\x80c\x9F\x0CY.\x14a\x01yW\x80c\x9F\xDB\xED\\\x14a\x01\x99W\x80c\xB0\x9F\xBE\x1A\x14a\x01\xAFW\x80c\xC2\xF0\xDFd\x14a\x01\xDFW\x80c\xC4\xCA\x17,\x14a\x02\x0FW\x80c\xC6?\xDC\xC7\x14a\x02/W`\0\x80\xFD[\x80c\x07i\xECF\x14a\0\xA1W\x80c'\xE25\xE3\x14a\0\xB6W\x80c-v\x9A\x1E\x14a\0\xF6W\x80c.\x1A}M\x14a\x01#W\x80c\x9B\x96\xEE\xCE\x14a\x01CW[`\0\x80\xFD[a\0\xB4a\0\xAF6`\x04a\r\xFBV[a\x02OV[\0[4\x80\x15a\0\xC2W`\0\x80\xFD[Pa\0\xE3a\0\xD16`\x04a\x0E^V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x02W`\0\x80\xFD[Pa\x01\x16a\x01\x116`\x04a\x0E^V[a\x03\xC5V[`@Qa\0\xED\x91\x90a\x0E\x80V[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\xB4a\x01>6`\x04a\x0E\xC4V[a\x041V[4\x80\x15a\x01OW`\0\x80\xFD[Pa\0\xE3a\x01^6`\x04a\x0E^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[4\x80\x15a\x01\x85W`\0\x80\xFD[Pa\0\xB4a\x01\x946`\x04a\x0E\xDDV[a\x05\x19V[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\0\xE3`\x02T\x81V[4\x80\x15a\x01\xBBW`\0\x80\xFD[Pa\x01\xCFa\x01\xCA6`\x04a\x0E\xC4V[a\x06\xD8V[`@Qa\0\xED\x94\x93\x92\x91\x90a\x0F\x1DV[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x01\xFFa\x01\xFA6`\x04a\x0FbV[a\x08\x0CV[`@Q\x90\x15\x15\x81R` \x01a\0\xEDV[4\x80\x15a\x02\x1BW`\0\x80\xFD[Pa\0\xE3a\x02*6`\x04a\x0E\xC4V[a\x08AV[4\x80\x15a\x02;W`\0\x80\xFD[Pa\0\xB4a\x02J6`\x04a\x0E\xC4V[a\t\xFDV[`\0\x82\x81R`\x01` R`@\x90 \x82\x15\x80a\x02\x80WP`\x01\x81T`\xFF\x16`\x03\x81\x11\x15a\x02}Wa\x02}a\x0F\x07V[\x14\x15[\x15a\x02\xA6W`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xD5W`@Qcw\x98\"\x07`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q\x15\x80a\x02\xE9WPa\x02\xE9\x82\x843a\x08\x0CV[\x15a\x03\x07W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 T`\x02\x82\x01Ta\x03&4\x83a\x0F\xCFV[\x10\x15a\x03GW`@Qc\x03\xF7K\x15`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x02\x9DV[3`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x03f\x90\x84\x90a\x0F\xCFV[\x90\x91UPP`\0\x84\x81R`\x01` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x81U`\x03\x01a\x03\x91\x84\x82a\x10lV[P`@Q\x84\x903\x90\x7F<\xBE9\x0Ek\xE8F\r\xA8/\xBF\x10\x01\xAA\xB3\x92\x07\xF3\xD6\xBB\xD8\x8A\x9E\xFB\x99\xF6\xBF\xCB\xBCT\x0F\xAE\x90`\0\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x04%W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04\x11W[PPPPP\x90P\x91\x90PV[3`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x04dW`@Qc\x03\xF7K\x15`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x02\x9DV[a\x04n\x82\x82a\x11,V[3`\0\x81\x81R` \x81\x90R`@\x80\x82 \x93\x90\x93U\x91Q\x84\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xC3V[``\x91P[PP\x90P\x80a\x05\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FFailed to withdraw TSSC\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x9DV[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05@W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x05aW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@\x80Q`\xA0\x81\x01\x82R`\x01\x80\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x80\x84\x01\x91\x90\x91R3\x83\x85\x01R``\x83\x01\x86\x90R\x83Q\x80\x82\x01\x85R`\0\x80\x82R`\x80\x85\x01\x91\x90\x91R\x85\x81R\x90\x82\x90R\x92\x90\x92 \x81Q\x81T\x92\x93\x91\x92\x90\x91\x83\x91`\xFF\x19\x16\x90\x83`\x03\x81\x11\x15a\x05\xD4Wa\x05\xD4a\x0F\x07V[\x02\x17\x90UP` \x82\x01Q\x81Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x02\x17\x82U`@\x83\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U``\x82\x01Q`\x02\x82\x01U`\x80\x82\x01Q`\x03\x82\x01\x90a\x067\x90\x82a\x10lV[PP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x81\x01\x83U\x91\x85R\x83\x85 \x01\x86\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x83 \x80T\x91\x82\x01\x81U\x83R\x90\x82 \x01\x83\x90U`\x02\x80T\x90\x92Pa\x06\x8D\x90a\x11?V[\x90\x91UP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x903\x90\x7F\xB6\xED\xFD>\x1A\x01\xDC;\xA1\xD7\x0Bb\x8D\xD7:\x9B#\x15M~\nM\x1F\xE4f\xE9|\x97dW\x0C\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x83\x92\x83\x92\x91\x90\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x07\x12Wa\x07\x12a\x0F\x07V[`\x03\x81\x11\x15a\x07#Wa\x07#a\x0F\x07V[\x81R\x81T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16` \x83\x01R`\x01\x83\x01T\x16`@\x82\x01R`\x02\x82\x01T``\x82\x01R`\x03\x82\x01\x80T`\x80\x90\x92\x01\x91a\x07f\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x92\x90a\x0F\xE2V[\x80\x15a\x07\xDFW\x80`\x1F\x10a\x07\xB4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xDFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81Q` \x83\x01Q`@\x84\x01Q``\x90\x94\x01Q\x91\x9A\x90\x99P\x92\x97P\x95P\x90\x93PPPPV[`\0\x80a\x08\x18\x84a\x08AV[\x90P`\0a\x08&\x82\x87a\x0C\xA9V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x85\x16\x14\x15\x92PPP\x93\x92PPPV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x08uWa\x08ua\x0F\x07V[`\x03\x81\x11\x15a\x08\x86Wa\x08\x86a\x0F\x07V[\x81R\x81T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16` \x83\x01R`\x01\x83\x01T\x16`@\x82\x01R`\x02\x82\x01T``\x82\x01R`\x03\x82\x01\x80T`\x80\x90\x92\x01\x91a\x08\xC9\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xF5\x90a\x0F\xE2V[\x80\x15a\tBW\x80`\x1F\x10a\t\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x81\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94\x87\x1B\x85\x16\x81\x87\x01R\x91\x86\x1B\x84\x16`4\x83\x01R`H\x82\x01R`h\x81\x01\x97\x90\x97R0\x90\x93\x1B\x16`\x88\x86\x01R\x81Q\x80\x86\x03`|\x01\x81R`\x9C\x86\x01\x83R\x80Q\x90\x82\x01 \x7F\x19Auto Request Payments:\n32\0\0\0\0\0\0`\xBC\x87\x01R`\xD6\x80\x87\x01\x91\x90\x91R\x82Q\x80\x87\x03\x90\x91\x01\x81R`\xF6\x90\x95\x01\x90\x91R\x83Q\x93\x01\x92\x90\x92 \x92\x91PPV[`\0\x81\x81R`\x01` R`@\x90 \x81\x15\x80a\n.WP`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\n+Wa\n+a\x0F\x07V[\x14\x15[\x15a\nOW`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x02\x9DV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x81R` \x81\x90R`@\x90 T`\x02\x83\x01T\x90\x91\x81\x83\x10\x15a\n\xA1W`@Qc\x03\xF7K\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x02\x9DV[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xCEW`@Qc\x1A\x1A\x8Ck`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Be\x84`\x03\x01\x80Ta\n\xE0\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x0C\x90a\x0F\xE2V[\x80\x15a\x0BYW\x80`\x1F\x10a\x0B.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BYV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B<W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x86\x83a\x08\x0CV[\x15a\x0B\x83W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83T`\xFF\x19\x16`\x03\x17\x84Ua\x0B\x98\x82\x84a\x11,V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x92\x90\x92U\x90Q3\x90\x84\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xF3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xF8V[``\x91P[PP\x90P\x80a\x0CSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FclaimPayment: Failed to send TSS`D\x82\x01R`C`\xF8\x1B`d\x82\x01R`\x84\x01a\x02\x9DV[3`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x87\x7F\xD5\xD2\xAF:\xEFy_Z\xF4\n\xDBr\xB3\x9D\xD2>\xF1\x8B\\[\r\x08\xFD\xE3)\xEF!:\xB6\xE2\xC0\xDF\x86`@Qa\x0C\x99\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[`\0\x80`\0\x80a\x0C\xB8\x85a\r)V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\r\x13W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x93PPPP[\x92\x91PPV[`\0\x80`\0\x83Q`A\x14a\r<W`\0\x80\xFD[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\r\x7FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x9AWa\r\x9Aa\rXV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\r\xC2Wa\r\xC2a\rXV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\r\xDBW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x0EW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E,W`\0\x80\xFD[a\x0E8\x85\x82\x86\x01a\rnV[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EYW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0EpW`\0\x80\xFD[a\x0Ey\x82a\x0EBV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E\xB8W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x9CV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0E\xD6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xF0W`\0\x80\xFD[a\x0E\xF9\x83a\x0EBV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x80\x81\x01`\x04\x86\x10a\x0F?WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x94\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x82\x01R\x91\x90\x92\x16`@\x82\x01R``\x01R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FwW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x8EW`\0\x80\xFD[a\x0F\x9A\x86\x82\x87\x01a\rnV[\x93PP` \x84\x015\x91Pa\x0F\xB0`@\x85\x01a\x0EBV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\r#Wa\r#a\x0F\xB9V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xF6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x16WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x14W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x10EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x10dW\x82\x81U`\x01\x01a\x10QV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x86Wa\x10\x86a\rXV[a\x10\x9A\x81a\x10\x94\x84Ta\x0F\xE2V[\x84a\x10\x1CV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x10\xCFW`\0\x84\x15a\x10\xB7WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x10dV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x10\xFEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x10\xDFV[P\x85\x82\x10\x15a\x11\x1CW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\r#Wa\r#a\x0F\xB9V[`\0`\x01\x82\x01a\x11QWa\x11Qa\x0F\xB9V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 r\x8B\x95\xE7KD\xA62\xC2H\x9C#5\x1D\x80\xC8\x1F\x1F\xD2\xA0\0\x03\x8EY\x96\x80\xDE|\xDAK\xC7WdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static SENDERSTREASURY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c\x9F\x0CY.\x11a\0dW\x80c\x9F\x0CY.\x14a\x01yW\x80c\x9F\xDB\xED\\\x14a\x01\x99W\x80c\xB0\x9F\xBE\x1A\x14a\x01\xAFW\x80c\xC2\xF0\xDFd\x14a\x01\xDFW\x80c\xC4\xCA\x17,\x14a\x02\x0FW\x80c\xC6?\xDC\xC7\x14a\x02/W`\0\x80\xFD[\x80c\x07i\xECF\x14a\0\xA1W\x80c'\xE25\xE3\x14a\0\xB6W\x80c-v\x9A\x1E\x14a\0\xF6W\x80c.\x1A}M\x14a\x01#W\x80c\x9B\x96\xEE\xCE\x14a\x01CW[`\0\x80\xFD[a\0\xB4a\0\xAF6`\x04a\r\xFBV[a\x02OV[\0[4\x80\x15a\0\xC2W`\0\x80\xFD[Pa\0\xE3a\0\xD16`\x04a\x0E^V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x02W`\0\x80\xFD[Pa\x01\x16a\x01\x116`\x04a\x0E^V[a\x03\xC5V[`@Qa\0\xED\x91\x90a\x0E\x80V[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\xB4a\x01>6`\x04a\x0E\xC4V[a\x041V[4\x80\x15a\x01OW`\0\x80\xFD[Pa\0\xE3a\x01^6`\x04a\x0E^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[4\x80\x15a\x01\x85W`\0\x80\xFD[Pa\0\xB4a\x01\x946`\x04a\x0E\xDDV[a\x05\x19V[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\0\xE3`\x02T\x81V[4\x80\x15a\x01\xBBW`\0\x80\xFD[Pa\x01\xCFa\x01\xCA6`\x04a\x0E\xC4V[a\x06\xD8V[`@Qa\0\xED\x94\x93\x92\x91\x90a\x0F\x1DV[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x01\xFFa\x01\xFA6`\x04a\x0FbV[a\x08\x0CV[`@Q\x90\x15\x15\x81R` \x01a\0\xEDV[4\x80\x15a\x02\x1BW`\0\x80\xFD[Pa\0\xE3a\x02*6`\x04a\x0E\xC4V[a\x08AV[4\x80\x15a\x02;W`\0\x80\xFD[Pa\0\xB4a\x02J6`\x04a\x0E\xC4V[a\t\xFDV[`\0\x82\x81R`\x01` R`@\x90 \x82\x15\x80a\x02\x80WP`\x01\x81T`\xFF\x16`\x03\x81\x11\x15a\x02}Wa\x02}a\x0F\x07V[\x14\x15[\x15a\x02\xA6W`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xD5W`@Qcw\x98\"\x07`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q\x15\x80a\x02\xE9WPa\x02\xE9\x82\x843a\x08\x0CV[\x15a\x03\x07W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 T`\x02\x82\x01Ta\x03&4\x83a\x0F\xCFV[\x10\x15a\x03GW`@Qc\x03\xF7K\x15`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x02\x9DV[3`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x03f\x90\x84\x90a\x0F\xCFV[\x90\x91UPP`\0\x84\x81R`\x01` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x81U`\x03\x01a\x03\x91\x84\x82a\x10lV[P`@Q\x84\x903\x90\x7F<\xBE9\x0Ek\xE8F\r\xA8/\xBF\x10\x01\xAA\xB3\x92\x07\xF3\xD6\xBB\xD8\x8A\x9E\xFB\x99\xF6\xBF\xCB\xBCT\x0F\xAE\x90`\0\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x04%W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04\x11W[PPPPP\x90P\x91\x90PV[3`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x04dW`@Qc\x03\xF7K\x15`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x02\x9DV[a\x04n\x82\x82a\x11,V[3`\0\x81\x81R` \x81\x90R`@\x80\x82 \x93\x90\x93U\x91Q\x84\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xC3V[``\x91P[PP\x90P\x80a\x05\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FFailed to withdraw TSSC\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x9DV[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05@W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x05aW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@\x80Q`\xA0\x81\x01\x82R`\x01\x80\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x80\x84\x01\x91\x90\x91R3\x83\x85\x01R``\x83\x01\x86\x90R\x83Q\x80\x82\x01\x85R`\0\x80\x82R`\x80\x85\x01\x91\x90\x91R\x85\x81R\x90\x82\x90R\x92\x90\x92 \x81Q\x81T\x92\x93\x91\x92\x90\x91\x83\x91`\xFF\x19\x16\x90\x83`\x03\x81\x11\x15a\x05\xD4Wa\x05\xD4a\x0F\x07V[\x02\x17\x90UP` \x82\x01Q\x81Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x02\x17\x82U`@\x83\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U``\x82\x01Q`\x02\x82\x01U`\x80\x82\x01Q`\x03\x82\x01\x90a\x067\x90\x82a\x10lV[PP3`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x81\x01\x83U\x91\x85R\x83\x85 \x01\x86\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x83 \x80T\x91\x82\x01\x81U\x83R\x90\x82 \x01\x83\x90U`\x02\x80T\x90\x92Pa\x06\x8D\x90a\x11?V[\x90\x91UP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x903\x90\x7F\xB6\xED\xFD>\x1A\x01\xDC;\xA1\xD7\x0Bb\x8D\xD7:\x9B#\x15M~\nM\x1F\xE4f\xE9|\x97dW\x0C\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x83\x92\x83\x92\x91\x90\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x07\x12Wa\x07\x12a\x0F\x07V[`\x03\x81\x11\x15a\x07#Wa\x07#a\x0F\x07V[\x81R\x81T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16` \x83\x01R`\x01\x83\x01T\x16`@\x82\x01R`\x02\x82\x01T``\x82\x01R`\x03\x82\x01\x80T`\x80\x90\x92\x01\x91a\x07f\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x92\x90a\x0F\xE2V[\x80\x15a\x07\xDFW\x80`\x1F\x10a\x07\xB4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xDFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81Q` \x83\x01Q`@\x84\x01Q``\x90\x94\x01Q\x91\x9A\x90\x99P\x92\x97P\x95P\x90\x93PPPPV[`\0\x80a\x08\x18\x84a\x08AV[\x90P`\0a\x08&\x82\x87a\x0C\xA9V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x85\x16\x14\x15\x92PPP\x93\x92PPPV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x08uWa\x08ua\x0F\x07V[`\x03\x81\x11\x15a\x08\x86Wa\x08\x86a\x0F\x07V[\x81R\x81T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16` \x83\x01R`\x01\x83\x01T\x16`@\x82\x01R`\x02\x82\x01T``\x82\x01R`\x03\x82\x01\x80T`\x80\x90\x92\x01\x91a\x08\xC9\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xF5\x90a\x0F\xE2V[\x80\x15a\tBW\x80`\x1F\x10a\t\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x81\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94\x87\x1B\x85\x16\x81\x87\x01R\x91\x86\x1B\x84\x16`4\x83\x01R`H\x82\x01R`h\x81\x01\x97\x90\x97R0\x90\x93\x1B\x16`\x88\x86\x01R\x81Q\x80\x86\x03`|\x01\x81R`\x9C\x86\x01\x83R\x80Q\x90\x82\x01 \x7F\x19Auto Request Payments:\n32\0\0\0\0\0\0`\xBC\x87\x01R`\xD6\x80\x87\x01\x91\x90\x91R\x82Q\x80\x87\x03\x90\x91\x01\x81R`\xF6\x90\x95\x01\x90\x91R\x83Q\x93\x01\x92\x90\x92 \x92\x91PPV[`\0\x81\x81R`\x01` R`@\x90 \x81\x15\x80a\n.WP`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\n+Wa\n+a\x0F\x07V[\x14\x15[\x15a\nOW`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x02\x9DV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x81R` \x81\x90R`@\x90 T`\x02\x83\x01T\x90\x91\x81\x83\x10\x15a\n\xA1W`@Qc\x03\xF7K\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x02\x9DV[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xCEW`@Qc\x1A\x1A\x8Ck`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Be\x84`\x03\x01\x80Ta\n\xE0\x90a\x0F\xE2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x0C\x90a\x0F\xE2V[\x80\x15a\x0BYW\x80`\x1F\x10a\x0B.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BYV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B<W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x86\x83a\x08\x0CV[\x15a\x0B\x83W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83T`\xFF\x19\x16`\x03\x17\x84Ua\x0B\x98\x82\x84a\x11,V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x92\x90\x92U\x90Q3\x90\x84\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xF3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xF8V[``\x91P[PP\x90P\x80a\x0CSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FclaimPayment: Failed to send TSS`D\x82\x01R`C`\xF8\x1B`d\x82\x01R`\x84\x01a\x02\x9DV[3`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x87\x7F\xD5\xD2\xAF:\xEFy_Z\xF4\n\xDBr\xB3\x9D\xD2>\xF1\x8B\\[\r\x08\xFD\xE3)\xEF!:\xB6\xE2\xC0\xDF\x86`@Qa\x0C\x99\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[`\0\x80`\0\x80a\x0C\xB8\x85a\r)V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\r\x13W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x93PPPP[\x92\x91PPV[`\0\x80`\0\x83Q`A\x14a\r<W`\0\x80\xFD[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\r\x7FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x9AWa\r\x9Aa\rXV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\r\xC2Wa\r\xC2a\rXV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\r\xDBW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x0EW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E,W`\0\x80\xFD[a\x0E8\x85\x82\x86\x01a\rnV[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EYW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0EpW`\0\x80\xFD[a\x0Ey\x82a\x0EBV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E\xB8W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x9CV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0E\xD6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xF0W`\0\x80\xFD[a\x0E\xF9\x83a\x0EBV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x80\x81\x01`\x04\x86\x10a\x0F?WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x94\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x82\x01R\x91\x90\x92\x16`@\x82\x01R``\x01R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FwW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x8EW`\0\x80\xFD[a\x0F\x9A\x86\x82\x87\x01a\rnV[\x93PP` \x84\x015\x91Pa\x0F\xB0`@\x85\x01a\x0EBV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\r#Wa\r#a\x0F\xB9V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xF6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x16WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x14W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x10EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x10dW\x82\x81U`\x01\x01a\x10QV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x86Wa\x10\x86a\rXV[a\x10\x9A\x81a\x10\x94\x84Ta\x0F\xE2V[\x84a\x10\x1CV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x10\xCFW`\0\x84\x15a\x10\xB7WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x10dV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x10\xFEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x10\xDFV[P\x85\x82\x10\x15a\x11\x1CW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\r#Wa\r#a\x0F\xB9V[`\0`\x01\x82\x01a\x11QWa\x11Qa\x0F\xB9V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 r\x8B\x95\xE7KD\xA62\xC2H\x9C#5\x1D\x80\xC8\x1F\x1F\xD2\xA0\0\x03\x8EY\x96\x80\xDE|\xDAK\xC7WdsolcC\0\x08\x17\x003";
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
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimPayment` (0xc63fdcc7) function
        pub fn claim_payment(
            &self,
            request_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 63, 220, 199], request_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `constructMessageOf` (0xc4ca172c) function
        pub fn construct_message_of(
            &self,
            request_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([196, 202, 23, 44], request_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalanceOf` (0x9b96eece) function
        pub fn get_balance_of(
            &self,
            sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 150, 238, 206], sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPaymentRequest` (0xb09fbe1a) function
        pub fn get_payment_request(
            &self,
            request_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([176, 159, 190, 26], request_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRequestedPayIdsOf` (0x2d769a1e) function
        pub fn get_requested_pay_ids_of(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([45, 118, 154, 30], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestPayId` (0x9fdbed5c) function
        pub fn request_pay_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 219, 237, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestPayment` (0x9f0c592e) function
        pub fn request_payment(
            &self,
            sender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 12, 89, 46], (sender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signPayReq` (0x0769ec46) function
        pub fn sign_pay_req(
            &self,
            request_id: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 105, 236, 70], (request_id, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignature` (0xc2f0df64) function
        pub fn verify_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
            request_id: ::ethers::core::types::U256,
            caller_sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [194, 240, 223, 100],
                    (signature, request_id, caller_sender),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PayRequestSigned` event
        pub fn pay_request_signed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PayRequestSignedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PaymentDone` event
        pub fn payment_done_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PaymentDoneFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PaymentRequested` event
        pub fn payment_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PaymentRequestedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendersTreasuryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SendersTreasury<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerIsNotReceiver` with signature `CallerIsNotReceiver()` and selector `0xd0d46358`
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
    #[etherror(name = "CallerIsNotReceiver", abi = "CallerIsNotReceiver()")]
    pub struct CallerIsNotReceiver;
    ///Custom Error type `CallerIsNotSender` with signature `CallerIsNotSender()` and selector `0xef30440e`
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
    #[etherror(name = "CallerIsNotSender", abi = "CallerIsNotSender()")]
    pub struct CallerIsNotSender;
    ///Custom Error type `InsufficientBalanceOf` with signature `InsufficientBalanceOf(address)` and selector `0x0fdd2c54`
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
    #[etherror(name = "InsufficientBalanceOf", abi = "InsufficientBalanceOf(address)")]
    pub struct InsufficientBalanceOf(pub ::ethers::core::types::Address);
    ///Custom Error type `InvalidDepositAmount` with signature `InvalidDepositAmount()` and selector `0xfe9ba5cd`
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
    #[etherror(name = "InvalidDepositAmount", abi = "InvalidDepositAmount()")]
    pub struct InvalidDepositAmount;
    ///Custom Error type `InvalidRequestId` with signature `InvalidRequestId(uint256)` and selector `0xc969e0f2`
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
    #[etherror(name = "InvalidRequestId", abi = "InvalidRequestId(uint256)")]
    pub struct InvalidRequestId(pub ::ethers::core::types::U256);
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `0x8baa579f`
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
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
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
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Custom Error type `ZeroAmount` with signature `ZeroAmount()` and selector `0x1f2a2005`
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
    #[etherror(name = "ZeroAmount", abi = "ZeroAmount()")]
    pub struct ZeroAmount;
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
    pub enum SendersTreasuryErrors {
        CallerIsNotReceiver(CallerIsNotReceiver),
        CallerIsNotSender(CallerIsNotSender),
        InsufficientBalanceOf(InsufficientBalanceOf),
        InvalidDepositAmount(InvalidDepositAmount),
        InvalidRequestId(InvalidRequestId),
        InvalidSignature(InvalidSignature),
        ZeroAddress(ZeroAddress),
        ZeroAmount(ZeroAmount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SendersTreasuryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CallerIsNotReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallerIsNotReceiver(decoded));
            }
            if let Ok(decoded) = <CallerIsNotSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallerIsNotSender(decoded));
            }
            if let Ok(decoded) = <InsufficientBalanceOf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalanceOf(decoded));
            }
            if let Ok(decoded) = <InvalidDepositAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDepositAmount(decoded));
            }
            if let Ok(decoded) = <InvalidRequestId as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidRequestId(decoded));
            }
            if let Ok(decoded) = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded) = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAddress(decoded));
            }
            if let Ok(decoded) = <ZeroAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SendersTreasuryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerIsNotReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerIsNotSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidRequestId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SendersTreasuryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerIsNotReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerIsNotSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalanceOf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDepositAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidRequestId as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroAmount as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SendersTreasuryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerIsNotReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerIsNotSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidRequestId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SendersTreasuryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerIsNotReceiver> for SendersTreasuryErrors {
        fn from(value: CallerIsNotReceiver) -> Self {
            Self::CallerIsNotReceiver(value)
        }
    }
    impl ::core::convert::From<CallerIsNotSender> for SendersTreasuryErrors {
        fn from(value: CallerIsNotSender) -> Self {
            Self::CallerIsNotSender(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceOf> for SendersTreasuryErrors {
        fn from(value: InsufficientBalanceOf) -> Self {
            Self::InsufficientBalanceOf(value)
        }
    }
    impl ::core::convert::From<InvalidDepositAmount> for SendersTreasuryErrors {
        fn from(value: InvalidDepositAmount) -> Self {
            Self::InvalidDepositAmount(value)
        }
    }
    impl ::core::convert::From<InvalidRequestId> for SendersTreasuryErrors {
        fn from(value: InvalidRequestId) -> Self {
            Self::InvalidRequestId(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for SendersTreasuryErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for SendersTreasuryErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroAmount> for SendersTreasuryErrors {
        fn from(value: ZeroAmount) -> Self {
            Self::ZeroAmount(value)
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
    #[ethevent(name = "PayRequestSigned", abi = "PayRequestSigned(address,uint256)")]
    pub struct PayRequestSignedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub request_pay_id: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "PaymentDone",
        abi = "PaymentDone(uint256,address,address,uint256)"
    )]
    pub struct PaymentDoneFilter {
        #[ethevent(indexed)]
        pub request_pay_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "PaymentRequested",
        abi = "PaymentRequested(address,uint256,address,uint256)"
    )]
    pub struct PaymentRequestedFilter {
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub request_pay_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum SendersTreasuryEvents {
        PayRequestSignedFilter(PayRequestSignedFilter),
        PaymentDoneFilter(PaymentDoneFilter),
        PaymentRequestedFilter(PaymentRequestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SendersTreasuryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PayRequestSignedFilter::decode_log(log) {
                return Ok(SendersTreasuryEvents::PayRequestSignedFilter(decoded));
            }
            if let Ok(decoded) = PaymentDoneFilter::decode_log(log) {
                return Ok(SendersTreasuryEvents::PaymentDoneFilter(decoded));
            }
            if let Ok(decoded) = PaymentRequestedFilter::decode_log(log) {
                return Ok(SendersTreasuryEvents::PaymentRequestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SendersTreasuryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PayRequestSignedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PaymentDoneFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PaymentRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PayRequestSignedFilter> for SendersTreasuryEvents {
        fn from(value: PayRequestSignedFilter) -> Self {
            Self::PayRequestSignedFilter(value)
        }
    }
    impl ::core::convert::From<PaymentDoneFilter> for SendersTreasuryEvents {
        fn from(value: PaymentDoneFilter) -> Self {
            Self::PaymentDoneFilter(value)
        }
    }
    impl ::core::convert::From<PaymentRequestedFilter> for SendersTreasuryEvents {
        fn from(value: PaymentRequestedFilter) -> Self {
            Self::PaymentRequestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `claimPayment` function with signature `claimPayment(uint256)` and selector `0xc63fdcc7`
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
    #[ethcall(name = "claimPayment", abi = "claimPayment(uint256)")]
    pub struct ClaimPaymentCall {
        pub request_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `constructMessageOf` function with signature `constructMessageOf(uint256)` and selector `0xc4ca172c`
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
    #[ethcall(name = "constructMessageOf", abi = "constructMessageOf(uint256)")]
    pub struct ConstructMessageOfCall {
        pub request_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBalanceOf` function with signature `getBalanceOf(address)` and selector `0x9b96eece`
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
    #[ethcall(name = "getBalanceOf", abi = "getBalanceOf(address)")]
    pub struct GetBalanceOfCall {
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPaymentRequest` function with signature `getPaymentRequest(uint256)` and selector `0xb09fbe1a`
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
    #[ethcall(name = "getPaymentRequest", abi = "getPaymentRequest(uint256)")]
    pub struct GetPaymentRequestCall {
        pub request_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRequestedPayIdsOf` function with signature `getRequestedPayIdsOf(address)` and selector `0x2d769a1e`
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
    #[ethcall(name = "getRequestedPayIdsOf", abi = "getRequestedPayIdsOf(address)")]
    pub struct GetRequestedPayIdsOfCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `requestPayId` function with signature `requestPayId()` and selector `0x9fdbed5c`
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
    #[ethcall(name = "requestPayId", abi = "requestPayId()")]
    pub struct RequestPayIdCall;
    ///Container type for all input parameters for the `requestPayment` function with signature `requestPayment(address,uint256)` and selector `0x9f0c592e`
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
    #[ethcall(name = "requestPayment", abi = "requestPayment(address,uint256)")]
    pub struct RequestPaymentCall {
        pub sender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `signPayReq` function with signature `signPayReq(uint256,bytes)` and selector `0x0769ec46`
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
    #[ethcall(name = "signPayReq", abi = "signPayReq(uint256,bytes)")]
    pub struct SignPayReqCall {
        pub request_id: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifySignature` function with signature `verifySignature(bytes,uint256,address)` and selector `0xc2f0df64`
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
    #[ethcall(name = "verifySignature", abi = "verifySignature(bytes,uint256,address)")]
    pub struct VerifySignatureCall {
        pub signature: ::ethers::core::types::Bytes,
        pub request_id: ::ethers::core::types::U256,
        pub caller_sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum SendersTreasuryCalls {
        Balances(BalancesCall),
        ClaimPayment(ClaimPaymentCall),
        ConstructMessageOf(ConstructMessageOfCall),
        GetBalanceOf(GetBalanceOfCall),
        GetPaymentRequest(GetPaymentRequestCall),
        GetRequestedPayIdsOf(GetRequestedPayIdsOfCall),
        RequestPayId(RequestPayIdCall),
        RequestPayment(RequestPaymentCall),
        SignPayReq(SignPayReqCall),
        VerifySignature(VerifySignatureCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for SendersTreasuryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) = <ClaimPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimPayment(decoded));
            }
            if let Ok(decoded) = <ConstructMessageOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConstructMessageOf(decoded));
            }
            if let Ok(decoded) = <GetBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBalanceOf(decoded));
            }
            if let Ok(decoded) = <GetPaymentRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPaymentRequest(decoded));
            }
            if let Ok(decoded) = <GetRequestedPayIdsOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRequestedPayIdsOf(decoded));
            }
            if let Ok(decoded) = <RequestPayIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestPayId(decoded));
            }
            if let Ok(decoded) = <RequestPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestPayment(decoded));
            }
            if let Ok(decoded) = <SignPayReqCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignPayReq(decoded));
            }
            if let Ok(decoded) = <VerifySignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySignature(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SendersTreasuryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Balances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstructMessageOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPaymentRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRequestedPayIdsOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestPayId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignPayReq(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SendersTreasuryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConstructMessageOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPaymentRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRequestedPayIdsOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestPayId(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignPayReq(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalancesCall> for SendersTreasuryCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<ClaimPaymentCall> for SendersTreasuryCalls {
        fn from(value: ClaimPaymentCall) -> Self {
            Self::ClaimPayment(value)
        }
    }
    impl ::core::convert::From<ConstructMessageOfCall> for SendersTreasuryCalls {
        fn from(value: ConstructMessageOfCall) -> Self {
            Self::ConstructMessageOf(value)
        }
    }
    impl ::core::convert::From<GetBalanceOfCall> for SendersTreasuryCalls {
        fn from(value: GetBalanceOfCall) -> Self {
            Self::GetBalanceOf(value)
        }
    }
    impl ::core::convert::From<GetPaymentRequestCall> for SendersTreasuryCalls {
        fn from(value: GetPaymentRequestCall) -> Self {
            Self::GetPaymentRequest(value)
        }
    }
    impl ::core::convert::From<GetRequestedPayIdsOfCall> for SendersTreasuryCalls {
        fn from(value: GetRequestedPayIdsOfCall) -> Self {
            Self::GetRequestedPayIdsOf(value)
        }
    }
    impl ::core::convert::From<RequestPayIdCall> for SendersTreasuryCalls {
        fn from(value: RequestPayIdCall) -> Self {
            Self::RequestPayId(value)
        }
    }
    impl ::core::convert::From<RequestPaymentCall> for SendersTreasuryCalls {
        fn from(value: RequestPaymentCall) -> Self {
            Self::RequestPayment(value)
        }
    }
    impl ::core::convert::From<SignPayReqCall> for SendersTreasuryCalls {
        fn from(value: SignPayReqCall) -> Self {
            Self::SignPayReq(value)
        }
    }
    impl ::core::convert::From<VerifySignatureCall> for SendersTreasuryCalls {
        fn from(value: VerifySignatureCall) -> Self {
            Self::VerifySignature(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for SendersTreasuryCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `constructMessageOf` function with signature `constructMessageOf(uint256)` and selector `0xc4ca172c`
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
    pub struct ConstructMessageOfReturn {
        pub message: [u8; 32],
    }
    ///Container type for all return fields from the `getBalanceOf` function with signature `getBalanceOf(address)` and selector `0x9b96eece`
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
    pub struct GetBalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPaymentRequest` function with signature `getPaymentRequest(uint256)` and selector `0xb09fbe1a`
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
    pub struct GetPaymentRequestReturn {
        pub status_code: u8,
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRequestedPayIdsOf` function with signature `getRequestedPayIdsOf(address)` and selector `0x2d769a1e`
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
    pub struct GetRequestedPayIdsOfReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `requestPayId` function with signature `requestPayId()` and selector `0x9fdbed5c`
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
    pub struct RequestPayIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `verifySignature` function with signature `verifySignature(bytes,uint256,address)` and selector `0xc2f0df64`
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
    pub struct VerifySignatureReturn(pub bool);
}
