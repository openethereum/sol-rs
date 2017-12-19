//! Private Transactions State Storage
//! By Parity Team, 2017.
//!
//! This contract keeps track of a private transaction state (supposedly encrypted)
//! and allows its change only if all the Validators (from a static list, initialized in constructor)
//! have signed a new state (hashed together with a current nonce, for replay protection).

pragma solidity 0.4.18;


contract PrivateContract {
    address[] public validators;
    bytes public state;
    bytes public code;
    uint256 public nonce;
	// debugging variable for querying nonced state hash
	bytes32 public noncedStateHash;
	// debugging variable for querying addressed returned by ecrecover
	address[] public recoveredAddress;

    function PrivateContract(address[] initialValidators, bytes initialCode, bytes initialState) public {
        validators = initialValidators;
		recoveredAddress = new address[](validators.length);
        code = initialCode;
        state = initialState;
        nonce = 1;
    }

    function setState(bytes newState, uint8[] v, bytes32[] r, bytes32[] s) public {
        noncedStateHash = keccak256([keccak256(newState), bytes32(nonce)]);

        for (uint i = 0; i < validators.length; i++) {
			recoveredAddress[i] = ecrecover(noncedStateHash, v[i], r[i], s[i]);
        }

        state = newState;
        nonce = nonce + 1;
    }
}

