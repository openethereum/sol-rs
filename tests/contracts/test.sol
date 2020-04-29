pragma solidity ^0.5.17;

library TestLibrary {
    function getValue() public returns(uint) {
        return 300;
    }
}

contract GetSenderTest {
    function getSender() public returns(address) {
        return msg.sender;
    }
}

contract GetValueTest {
    function getValue() public payable returns(uint) {
        return msg.value;
    }
}

contract ConstructorTest {
    uint public value = 0;

    constructor(uint _value) public {
        value = _value;
    }

    function getValue() public returns(uint) {
        return value;
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

contract LibraryTest {
    function getValueFromLibrary() public returns(uint) {
        return TestLibrary.getValue();
    }
}
