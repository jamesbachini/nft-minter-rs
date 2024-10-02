// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract NFT is ERC721 {
    uint256 private _nextTokenId;

    constructor() ERC721("Generic NFT", "NFT") {}

    function mint(address _to) public {
        uint256 tokenId = _nextTokenId++;
        _safeMint(_to, tokenId);
    }
}