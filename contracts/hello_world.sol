pragma solidity>=0.8.0;

contract helloworld {
    string name;

    constructor() public {
        name = "Hello, World!";
    }

    function get() public view returns (string memory) {
        return name;
    }
    
    event Set(address indexed_from, string n);
    function set(string memory n) public {
        name = n;
        emit Set(msg.sender, n);
    }
}