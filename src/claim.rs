#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait Claim {
    #[init]
    fn init(
        &self,
        tokenid: EgldOrEsdtTokenIdentifier,
        nonce_id: u64,
        identifiernft: EgldOrEsdtTokenIdentifier,
        nft_counter: BigUint,
    ) {
        self.token_id().set(&tokenid);
        self.nonce_token().set(nonce_id);
        self.token_id_nft().set(&identifiernft);
        self.nft_in_collection().set(nft_counter);
        self.status().set(true);
        self.timer().set(604800); // 7 days time to claim
    }

    #[endpoint]
    #[payable("*")]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();
        let mut counter_reward = BigUint::from(0u32);
        let reward_per_nft = self.reward_per_nft().get() ;
        let timestamp = self.blockchain().get_block_timestamp();

        require!(self.status().get() == true, "Claim is closed");
        require!(self.deadline().get() >= timestamp, "Claim is closed");

        for payment in payments.into_iter() {
            require!(
                payment.token_identifier == self.token_id_nft().get(),
                "Incorrect NFT"
            );
            let mut finder = 0;

            let nonces = self.nonces_nft_already_claimed();

            for nonce in nonces.iter() {
                if nonce == payment.token_nonce {
                    finder = 1;
                }
            }

            if finder == 0 {
                counter_reward = counter_reward + BigUint::from(1u32); //check how many eligible nft a user have
            }

            self.nonces_nft_already_claimed().push(&payment.token_nonce);

            // Send back nfts
            self.send().direct_esdt(
                &caller,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }

        require!(counter_reward != 0, "You have already received the rewards!");

        let amount = counter_reward * reward_per_nft;
        let nonce = self.nonce_token().get();
        self.send()
            .direct(&caller, &self.token_id().get(), nonce, &amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint]
    fn fund(&self) {
        let payments = self.call_value().all_esdt_transfers();

        for payment in payments.into_iter() {
            self.nonce_token().set(payment.token_nonce); //set new nonce for lkmex
        }
    }

    #[only_owner]
    #[endpoint]
    fn start_claim(&self, nftcounter:BigUint) {
        let timestamp = self.blockchain().get_block_timestamp();
        let timer = self.timer().get();
        self.status().set(true);
        self.deadline().set(timestamp+timer);

        let token_id = self.token_id().get();
        let nonce = self.nonce_token().get();

        let balance = self.blockchain().get_sc_balance(&token_id, nonce);
        self.nft_in_collection().set(&nftcounter);

        self.nonces_nft_already_claimed().clear();

        self.reward_per_nft().set(balance/nftcounter);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint]
    fn withdraw(&self) {
        require!(self.status().get() == false, "Wait until status is off");
        let token_id = self.token_id().get();
        let nonce = self.nonce_token().get();

        let caller = self.blockchain().get_caller();
        let balance = self.blockchain().get_sc_balance(&token_id, nonce);

        self.send().direct(&caller, &token_id, nonce, &balance);
    }

    #[only_owner]
    #[endpoint]
    fn stop_claim(&self) {
        self.status().set(false);
    }

    #[only_owner]
    #[endpoint]
    fn change_timer(&self, newtimer:u64) {
        self.timer().set(newtimer);
    }

    #[only_owner]
    #[endpoint]
    fn change_token_id(&self, tokenid: EgldOrEsdtTokenIdentifier) {
        self.token_id().set(tokenid);
    }

    #[only_owner]
    #[endpoint]
    fn change_token_nonce(&self, token_nonce: u64) {
        self.nonce_token().set(token_nonce);
    }

    // STORAGE AREA
    #[view(getTokenId)]
    #[storage_mapper("tokenid")]
    fn token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getTokenIdNft)]
    #[storage_mapper("tokenidnft")]
    fn token_id_nft(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getRewardPerNft)]
    #[storage_mapper("rewardpernft")]
    fn reward_per_nft(&self) -> SingleValueMapper<BigUint>;

    #[view(getTimer)]
    #[storage_mapper("timer")]
    fn timer(&self) -> SingleValueMapper<u64>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getNonce)]
    #[storage_mapper("nonce")]
    fn nonce(&self) -> SingleValueMapper<u64>;

    #[view(getNonceAlreadyClaimed)]
    #[storage_mapper("nonceclaimed")]
    fn nonces_nft_already_claimed(&self) -> VecMapper<u64>;

    #[view(getStatus)]
    #[storage_mapper("status")]
    fn status(&self) -> SingleValueMapper<bool>; //true - open // false - closed

    #[view(getNonceId)]
    #[storage_mapper("nonceid")]
    fn nonce_token(&self) -> SingleValueMapper<u64>;

    #[view(gtNftInCollection)]
    #[storage_mapper("nftincollection")]
    fn nft_in_collection(&self) -> SingleValueMapper<BigUint>;
}
