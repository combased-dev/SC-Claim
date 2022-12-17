# SC-Claim for MaiarWizards

The init method initializes the contract by setting the token identifier for the payment token, the nonce for the payment token, the identifier for a non-fungible token (NFT), and the number of NFTs in the contract's collection.

The claim method allows users to claim a reward by sending NFTs to the contract. The method checks that the claim is still open, verifies that the NFTs being sent are valid, and calculates the reward based on the number of eligible NFTs the user has and the reward per NFT set by the contract owner. It then sends the reward to the user.

The fund method allows the contract owner to add funds to the contract by sending payment tokens to it.

The start_claim method allows the contract owner to open the claim period and set the reward per NFT. It also resets the list of NFT nonces that have already been claimed and sets the deadline for the claim period.

The close_claim method allows the contract owner to close the claim period.

The extend_claim method allows the contract owner to extend the claim period by a certain number of seconds.

The set_reward_per_nft method allows the contract owner to set the reward per NFT.

The view_balance method allows the contract owner to view the balance of payment tokens in the contract.

The view_nonce method allows the contract owner to view the current nonce for the payment tokens in the contract.

The view_reward_per_nft method allows the contract owner to view the current reward per NFT set by the contract.

The view_nft_in_collection method allows the contract owner to view the current number of NFTs in the contract's collection.

The view_nonces_nft_already_claimed method allows the contract owner to view the list of NFT nonces that have already been claimed.
