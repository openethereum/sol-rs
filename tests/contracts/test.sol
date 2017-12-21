contract GetSenderTest {
    function getSender() public returns(address) {
        return msg.sender;
    }
}
