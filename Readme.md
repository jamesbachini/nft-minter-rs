# NFT Minter Bot

Full tutorial is available at https://jamesbachini.com/web3-interactions-rust/

This repo contains a Rust based bot for minting NFTs on the Ethereum Sepolia testnet. It interacts with a simple ERC721 NFT smart contract to create new tokens.

## Prerequisites

- Rust and Cargo (latest stable version)
- An Infura API key
- A private key for an Ethereum wallet with some Sepolia ETH

## Setup

1. Clone this repository:
   ```
   git clone https://github.com/jamesbachini/nft-minter-rs.git
   cd nft-minter-rs
   ```

2. Create a `.env` file in the project root with the following content:
   ```
   PRIVATE_KEY=your_private_key_here
   INFURA_API_KEY=your_infura_api_key_here
   ```

3. Install dependencies:
   ```
   cargo build
   ```

## Usage

To run the NFT minter bot:

```
cargo run
```

This will connect to the Sepolia testnet, mint an NFT to your address, and display the transaction details.

## Smart Contract

The NFT smart contract used in this project is deployed at:
`0xaF3148FE00021529cBdBd6129383119dc16bDF0C`

You can view it on [Sepolia Etherscan](https://sepolia.etherscan.io/address/0xaf3148fe00021529cbdbd6129383119dc16bdf0c#code).

Or you can deploy your own, code is in contracts/NFT.sol

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Disclaimer

This is a test project and should not be used with real assets on the Ethereum mainnet without proper review and modification.

## Links

- [Website](https://jamesbachini.com)
- [YouTube](https://www.youtube.com/c/JamesBachini?sub_confirmation=1)
- [Substack](https://bachini.substack.com)
- [Podcast](https://podcasters.spotify.com/pod/show/jamesbachini)
- [Spotify](https://open.spotify.com/show/2N0D9nvdxoe9rY3jxE4nOZ)
- [Twitter](https://twitter.com/james_bachini)
- [LinkedIn](https://www.linkedin.com/in/james-bachini/)
- [GitHub](https://github.com/jamesbachini)