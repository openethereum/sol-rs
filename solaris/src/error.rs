use ethabi;
use ethcore::client::EvmTestError;
use evm;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Ethabi(ethabi::Error);
        TransactError(evm::TransactError);
    }

    errors {
        // required because `ethcore::client::EvmTestError` does not implement `std::error::Error`
        EVM(err: EvmTestError) {
            description("VM Error"),
            display("{:?}", err),

        }
    }
}

impl From<EvmTestError> for Error {
    fn from(err: EvmTestError) -> Self {
        ErrorKind::EVM(err).into()
    }
}
