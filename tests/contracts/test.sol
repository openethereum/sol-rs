contract GetSenderTest {
    function getSender() public returns(address) {
        return msg.sender;
    }
}

contract GetValueTest {
    function getValue() payable public returns(uint) {
        return msg.value;
    }
}
