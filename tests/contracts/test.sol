contract GetSenderTest {
    function getSender() public returns(address) {
        return msg.sender;
    }
}

contract GetValueTest {
    function getValue() public returns(uint) {
        return msg.value;
    }
}
