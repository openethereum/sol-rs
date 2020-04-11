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

contract EventLogTest {
    event Foo(address sender);
    event Bar(uint value);
    event Baz();

    function emitFoo() public {
        emit Foo(msg.sender);
    }

    function emitBar(uint value) public {
        emit Bar(value);
    }
}
