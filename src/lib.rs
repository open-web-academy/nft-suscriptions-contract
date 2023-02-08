use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::migrate::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;
mod migrate;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
pub const ICON: &str = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/4gHYSUNDX1BST0ZJTEUAAQEAAAHIAAAAAAQwAABtbnRyUkdCIFhZWiAAAAAAAAAAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAACRyWFlaAAABFAAAABRnWFlaAAABKAAAABRiWFlaAAABPAAAABR3dHB0AAABUAAAABRyVFJDAAABZAAAAChnVFJDAAABZAAAAChiVFJDAAABZAAAAChjcHJ0AAABjAAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEJYWVogAAAAAAAAb6IAADj1AAADkFhZWiAAAAAAAABimQAAt4UAABjaWFlaIAAAAAAAACSgAAAPhAAAts9YWVogAAAAAAAA9tYAAQAAAADTLXBhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACAAAAAcAEcAbwBvAGcAbABlACAASQBuAGMALgAgADIAMAAxADb/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/2wBDAQMDAwQDBAgEBAgQCwkLEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBD/wAARCABgAGADASIAAhEBAxEB/8QAHgAAAQUBAQEBAQAAAAAAAAAACAAFBgcJBAIDAQr/xAA+EAABAwMCAwUDCgQGAwAAAAABAgMEAAUGBxESITEIEyJBUWGRoQkUFRYjMkJxscFSU2OBMzRDcpLwYqLx/8QAGwEAAgMBAQEAAAAAAAAAAAAABgcEBQgCAAP/xAA0EQABAgQDBAgFBQEAAAAAAAABAgMABAUGETFREiFBgQcTIiNhcZGxMjNS0eEUFRahwUL/2gAMAwEAAhEDEQA/ANU6VKlXo9CrytaG0lS1hIHmTtUK1K1axbTO2qlXiWhUhQPdR0nxrNCJqB2ls1zV9xiBLVbYCiQGmjzI9vWiih2lUK53jY2W/qOXLWAm6L8pdsDq3Ttu/QnPmchBkX/U/BsbChdchitLT+DiJPwqEy+07gTTvcwUSZqt9h3Wx3oVtO9O8u1Suoahh4scX20lzcpSKLzTzQbD8HjtuuxETpwAKnnRvsfZVzVKHQ7eHVzTinXfpGA9dIEqNc93XevrKeyiXY+tQKvTX0wjuxrU64ZPwuQ8MuaGVdHVhAG3/Kp42pS20qWgpJHMHypIbZYRwtoShI9BsBUVy3VbAcJbUvIskhxVJ/ApfP4UGOATrmzKM4eAxUYZkqHKYxtVGZ2jqQlI5YfcxLaVUMrtq6DNTvmUnK2WfFw94onh/SrdxXNcWza3N3bFr3GuMVwbpcZVuK5mKfNyg2n21JHiDEuXqEpN/IcSryIh8pUqVQ4mQqg+ruptr0vxORfJq0l8pKYzRPNa/T9amrriWWlurOyUJKifYKzm7UGsbuoudSLXBfJtdocLLIB5LP8AF8SKK7Ot83BUUsr+Wnery05wKXlXjQKYp5v5iuynzPHlEOzDPL3nV8kXu9THHnHlkpSTyQPQVKtGdM7pqhkrNtjIUiG0oKkvbckp/wC7VVdvS9NlNRGElbjywhIHmTWkOg+nlu0t09YdloQ3LkNB+Y4rkQfT3AU5LvrLdsU4NywAWrsoGnjy94Q1oWyu7KqXJwktp7Szr4Y+PtE5w/D7LhNlYs1mioZaZSApQHNR9TXfMvVqt7a3Zc9ltKBurdXSqmyXM8my2O5IsU36KtCVcDb/APqSfanqNvz26U42TTqwJSj6RYcmPLTu46+rms/2O1Z0cdW+4XHTiScSY08yw3LNBlgBKQMABkIqHXjtL3YokWHBlLjNDdC5g6q/2/8Ayghzi73u7SHZdxuT77qySVLVvua00yTRrB7zEUyuytt+HkUDr76pHKuwtb79IfVZsjVFLm5aQ4PCPZyTTOt25aLJM9T1fVHXPHnnCbuGzrkm5z9U48H047gOzsjQJJw/vGM3L9Mf3U0+oqB6E+VXl2N9ScxwaXOkQbk+YjT4+yUrdChsOVdWuXYw1a0+YeuTVrN2gN7lT8VJISPU77V+6U44cbxtll5vgkP+N0HqFdP2o6lnZSrtnYIWg58YEromZm3qeE4Ft4kYcCCN+IjUbTjP7ZqDjzN3grAd4QHmt+aFelSygl7OmfPYpl7FveeIh3FQaWknkk9d/hRspUFpCx0I3FJe6KJ+yTxbR8Ct6fLTlDd6P7r/AJXSQ8781HZX58DzG+K17RudDTvRzJcmS5wPR4au6O/4iQP3rKSxXVy4w0zH1lTjxK1KJ5kkmjx+Upvi7VoIqIhwp+fySyQPMcJO3wrOXArql60soKubY4FU0uiZhCJd1w/Es+0DnSalT4bSMk/7BOdlzFE5jqva2Xm+NiEsPugjltzH6mjj1pu8hqHacNt4Jdvb4jKSjrwcJP6poX+wFEafzK8zFEFTcMBP/NNE7qNPj2fUPGbnOhqeYB4EqA/w1+I8XuoR6UJpT1a6g5ISAOe8xddGskiWpBeGa1EnluEe7Lb7fZrRHxdTKXLs6nuCF/gT13Hs/enxu0XG0S4Vsizg9urvF991I2I8q4LPCt+a3u5X1EvhMZz5vFebPiSnYK3G/tJr6x4GRJy8sNXZMgMRfvP9fvewUt4YcPsWC+9Kdhz3x4PG2E9NqdG0BZDTgHG2eSh5impyJcWrhGemSxu4e7V3fTbmfOo/qxqtjelOPuXGfJQqYpJEdgK8Tiq+0vLuzboZZGKjkIjTc2zIsqmJhQSlIxJMRXtL6rwcJxVyxRXG3LlckFtKeRKE/wAW39qCNh0uKKz1UdzSzLO7xn2RScgvMhS1vLJQknkhPoKiN3zmLZ1BiIUvSB1HkPzrQFr0H9olQynetW9R8fxGUbwq81etVxYSdhO5I0Gp884IfR3Ti6ZXdWrk6+IFvirDi5Lh4Ry8hRy2x2O9AYXFkJfa4AEuJO4Vty3rJ2BneomdvM2k3uQxbWtt2WjshKfdvWkvZ9KxpXZGVrKu6YCQSdyeZoP6RKbMtttzUwsZ4BIGQPEniThDS6MkU2ivrpEvip5SdtauG44AYcM/OKO+Uvsy7loEZraFK+j5ReVt5DhI3PvrK3C759Gyu5dV9k7yPsPrW3vaRwAalaNZNiqGwp+TCV3PLooEH9BWFcyDJs90k2yW0pt6K6ptSSNiCDUno8qBalilB7SFY8jBVdsmHldsblD2g++whnEey6om1SX0hu7MBhsk/i4gf2o9dS8elXm0NXK1pCp9qX85jpI++diNvcTWJGnuol6wm9Qb1b3ld7BdDjZB5jatGMR+Ul06fscVGRWG4InJbCXuAo4Sr1G5rnpBpDtWnEVCRQVFQwUOII4+REQ7QqLNLllSU2oJAOIOoMXVpezbL7GnymJb9on/ADnZ1DewPFwjyO9SC1xLvHy2cF3wrDTPNx0gbp3HoKE7MO3Tgbc1VzwTGprct1XE6lwoDTv+7ZW/p0qmc47VOoubzHnYUz6Gjvo7tbUVR2Unffnxb0HyNmVObUOsTsDU/aLmo3pTJFBKFbatB94M/WztL43p6wbfb7u3dby2TwtMq3S2rbqrpQSZvqTfc4ur1/yi5rcKiVALV4UD0FVPfc6g2vjkT5hkSVc+Eq3UTVfXjN7nf3eAuFpjfk2k+VNW37YlaQnFsYrOajny0hWVecq13r73u2BkOH5P9RZN+1H70qg2dRCeinQev5V8MUst0ye4JajIWvc7uLPQCmnTvTq+ZfJS4GVswwRxvKHL+1ExjOLWzF4CIcBkAgeNe3NR9TRkHUS6cEZwI1ityNssGUkgFPHPw8Sf8j3jmPRLBBbiR0Dj2HGrzUaPrQiM7G0ztCXRsVM7j3mgns8B66XSLAYQVOPupSAPOtA8UtSbJjsC1oGwjshP7/vSp6SJsfpmmCe0pRPID8xddCMu/OVSaqTpJwSBjqVHH2EOi0JcQptY3SoEEeorLHt89k+7YdlknVLDbY5Is90WXZiGUf5dfr+WwHvrVCuW52u3XmG7b7pDalRnk8K23E7pUKXFGq7tGmQ83vGRGojQ8/Ion2i2rceBj+fOMFoPCtJSR5EbU7xXUpAKlAfmaLv5RiNoppChrGMD05h/Wm8ILqn2kq2jp3I4h4uu4+NZ7NWjUm9+BuNLCFfltTrpdRFTlxMpQUg5YwvJ6g9Ssh55KR4mLQm5lYrK2TKmoKx0QDzNRO7aszp+8e0IMdvpx/iNc1o0LyS4rDt1ntRknmQoniqzcX0bw6yKS9cXFT3E89nNuHf+21WzeJzEUb71ApY2lKLy9AMR9v7itcdsWTZbMSiDDfkrcPNZHIUQuneg9vtxbuGWSEvuDZQjp+6Pzp0ttxtVnZTHtsZqO2kbAIFdv1qH8741LDmAwgFrtx1KpgsyieqR4fF68OUWdCftdujoiwmUNNoGwSkbCuj6VjfxCqp+tQ/nfGrD0awTJtXckZtdoYc+aIUDJkEeFtP/AHavhMTbUq2XnlYJGZgDl7Wmp58NNAqWowRPZhwY5JfzlEtg/NLed2yRyUvl+xovAABsOgpgwbDbXguORMftbSUojoCVKA5rV6mpBSAuKsqrc6p//kbkjw/Ma0sm127TpSZMb1ntLOqj9soVKlSqigvimO0B2ZMO1ygd/LaREvTCClialO6vPkd9+XP0rN/WHs66saPTnhcLA/MtqCeCbHRu2pPr6/Cth655kCFcWVR50Vp9pY2UlxIIIono11TlIAa+JvQ8PIwOVe2JKrK61Y2V6jj5xg27kjzCih4LbUORCgQa8fWv+p8a2Bz3sc6C6gLcfuWEw48lwkqfYSQrf15naqZvPyXOkM58u2/JLxESTvwJDew/9aOZe/JBwd6FJPlj7QKOWIpJ7BBjOT61/wBT410wr1PuLyY8Fh59xZ2SltJJJrR3Hvkw9F7W6l253i6XAA78DoRsfcBV7afdmbRjTUIXjWEwGpCNvt1IJV8TtXEzfsi2nuElR9BHTVhlR7wgD1gCdBOxzqXqbJj3bKYr1jspIWVPDZbqf/Hr8a0a010uxTS2wNWLGbe2ylCR3ju3icV6mpY202ygNtNpQkcgEjYCvdAFYuKcrKsHTgjgkZc9YL6Rb0lR+0ynFf1HP8QqVKlVDF7H/9k=";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokensCosts {
    permanent_cost: U128,
    one_year_cost: U128,
    six_months_cost: U128,
    one_month_cost: U128
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokensCounter {
    canbuy: bool,
    permanent: u64,
    one_year: u64,
    six_months: u64,
    one_month: u64
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub tokens_to_mint_counter: LookupMap<AccountId, TokensCounter>,
    pub permanent_cost: U128,
    pub one_year_cost: U128,
    pub six_months_cost: U128,
    pub one_month_cost: U128,
    pub usdt_contract: String,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub tokens_to_mint_counter: LookupMap<AccountId, TokensCounter>,
    pub permanent_cost: U128,
    pub one_year_cost: U128,
    pub six_months_cost: U128,
    pub one_month_cost: U128,
    pub usdt_contract: String,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    TokensToMintCounter,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "OWA".to_string(),
                symbol: "OWA".to_string(),
                icon: Some(ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            tokens_to_mint_counter: LookupMap::new(StorageKey::TokensToMintCounter.try_to_vec().unwrap()),
            permanent_cost: near_sdk::json_types::U128(200000000), //200 USDT.e
            one_year_cost: near_sdk::json_types::U128(100000000), //100 USDT.e
            six_months_cost: near_sdk::json_types::U128(50000000), //50 USDT.e
            one_month_cost: near_sdk::json_types::U128(10000000), //10 USDT.e
            usdt_contract: "usdt.fakes.testnet".to_string()
        };

        //return the Contract object
        this
    }
    
}

#[cfg(test)]
mod tests;