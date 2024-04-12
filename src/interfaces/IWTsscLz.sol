// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IOFT} from "@layerzerolabs/oapp/contracts/oft/interfaces/IOFT.sol";
import {IWETH} from "./IWETH.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

interface IWTsscLz is IOFT, IERC20, IWETH {}
