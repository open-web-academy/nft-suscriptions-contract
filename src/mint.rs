use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue, PromiseResult, PanicOnDefault, log, BorshStorageKey, require};
use near_sdk::json_types::{U128, U64};
use near_sdk::Promise;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};

pub const COMMON: &str = "QmdchH3pkgZYvmWiU5f2TUXkrzrhQmFYqsFpnZY2gzxFSX";
pub const RARE: &str = "QmWp4Mfvv3czXTgk1qjAWmXLwSk4BrbMA5hksdWHz564GR";
pub const SUPERRARE: &str = "QmaCGBCABiq3cuBKrUb2DSDMikq3VEWK3KLZTdKZFcv1WL";
pub const LEGENDARY: &str = "Qmcn6Px9qii11fRoPWxaYsGzRwvBfFqwLGL5fr5BjxjyYt";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraData {
    init: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NewTokenMetadata {
    title: String, 
    description: String, 
    media: String, 
    extra: String
}

#[near_bindgen]
impl Contract {

    pub fn show_costs(&self) -> TokensCosts{
        let costs = TokensCosts {
            one_month_cost: self.one_month_cost,
            six_months_cost: self.six_months_cost,
            one_year_cost: self.one_year_cost,
            permanent_cost: self.permanent_cost
        };

        return costs;
    }

    pub fn change_costs(&mut self, one_month_cost: U128 , six_months_cost: U128 , one_year_cost: U128 , permanent_cost: U128) -> TokensCosts {
        self.assert_owner();
        self.one_month_cost = one_month_cost;
        self.six_months_cost = six_months_cost;
        self.one_year_cost = one_year_cost;
        self.permanent_cost = permanent_cost;

        let costs = TokensCosts {
            one_month_cost: self.one_month_cost,
            six_months_cost: self.six_months_cost,
            one_year_cost: self.one_year_cost,
            permanent_cost: self.permanent_cost
        };

        return costs;
    }

    pub fn show_pendant_suscriptions(&self) -> TokensCounter{
        let signer_id = env::signer_account_id();
        let user_counter = self.tokens_to_mint_counter.get(&signer_id.clone());

        if !user_counter.is_none() {
            // Ya existe registro de los contadores
            let mut user_counter = TokensCounter {
                canbuy: user_counter.clone().unwrap().canbuy,
                permanent: user_counter.clone().unwrap().permanent,
                one_year: user_counter.clone().unwrap().one_year,
                six_months: user_counter.clone().unwrap().six_months,
                one_month: user_counter.clone().unwrap().one_month
            };

            return user_counter;

        } else {
            // No existe registro de los contadores
            let mut null_counter = TokensCounter {
                canbuy: true,
                permanent: 0,
                one_year: 0,
                six_months: 0,
                one_month: 0
            };

            return null_counter;
            
        }

    }

    pub fn show_usdt_contract(&self) -> String {
        return self.usdt_contract.to_string();
    }

    pub fn change_usdt_contract(&mut self, new_contract: String) -> String {
        self.assert_owner();
        self.usdt_contract = new_contract;
        return self.usdt_contract.to_string();
    }

    pub fn ft_on_transfer(&mut self,sender_id: AccountId, amount: U128, msg: String,)  -> PromiseOrValue<U128>{
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let amount: Balance = amount.into();

        if contract_id.clone().to_string() != self.usdt_contract.to_string(){
            env::panic_str("Only the USDT.e contract can call this method");
        }

        log!("contract_id: {}",contract_id.clone());
        log!("signer_id: {}",signer_id.clone());
        log!("sender_id: {}",sender_id.clone());
        log!("amount: {}",amount.clone());
        log!("message: {}",msg.clone());

        // Validar si la cantidad transferida corresponde a una suscripción, de lo contrario regresar el dinero
        if near_sdk::json_types::U128(amount.clone()) != self.one_month_cost && near_sdk::json_types::U128(amount.clone()) != self.six_months_cost && near_sdk::json_types::U128(amount.clone()) != self.one_year_cost && near_sdk::json_types::U128(amount.clone()) != self.permanent_cost {
            env::panic_str("The amount sent does not correspond to any subscription");
        }

        // Guardar registro en contador
        self.save_mint_counter(signer_id.clone(), near_sdk::json_types::U128(amount.clone()));

        //return near_sdk::PromiseOrValue::Value(true); // Regresar tokens
        PromiseOrValue::Value(U128(0)) // No Regresar tokens

    }

    //#[private]
    fn save_mint_counter(&mut self, signer_id: AccountId, amount: U128 ) {
        let user_counter = self.tokens_to_mint_counter.get(&signer_id.clone());

        if !user_counter.is_none() {
            // Ya existe registro de los contadores
            let mut old_counter = TokensCounter {
                canbuy: user_counter.clone().unwrap().canbuy,
                permanent: user_counter.clone().unwrap().permanent,
                one_year: user_counter.clone().unwrap().one_year,
                six_months: user_counter.clone().unwrap().six_months,
                one_month: user_counter.clone().unwrap().one_month
            };

            // Actualizar contador dependiendo de la compra
            if amount == self.one_month_cost {
                old_counter.one_month += 1;
            }
            if amount == self.six_months_cost {
                old_counter.six_months += 1;
            }
            if amount == self.one_year_cost {
                old_counter.one_year += 1;
            }
            if amount == self.permanent_cost {
                old_counter.permanent += 1;
            }
            old_counter.canbuy = false;

            // Guardar nuevo registro de contador
            self.tokens_to_mint_counter.insert(&signer_id.clone(), &old_counter);
        } else {
            // No existe registro de los contadores
            let mut new_counter = TokensCounter {
                canbuy: false,
                permanent: 0,
                one_year: 0,
                six_months: 0,
                one_month: 0
            };

            if amount == self.one_month_cost {
                new_counter.one_month = 1;
            }
            if amount == self.six_months_cost {
                new_counter.six_months = 1;
            }
            if amount == self.one_year_cost {
                new_counter.one_year = 1;
            }
            if amount == self.permanent_cost {
                new_counter.permanent = 1;
            }

            // Guardar nuevo registro de contador
            self.tokens_to_mint_counter.insert(&signer_id.clone(), &new_counter);
        }

    }

    #[payable]
    pub fn mint( &mut self, receiver_id: AccountId, type_suscription: String) -> bool {
        log!("receiver_id: {}",receiver_id.clone());
        log!("type_suscription: {}",type_suscription.clone());
        let deposit = env::attached_deposit(); // 0.1 Nears
        log!("deposit: {}",deposit.clone());

        // Verificar que se envio 0.01 NEAR para cubrir el storage
        if deposit < 10000000000000000000000 {
            log!("Debe enviar 0.01 NEAR");
            return false;
        }

        // Verificar si existe el tipo de suscripción
        if type_suscription != "one_month".to_string() && type_suscription != "six_months".to_string() && type_suscription != "one_year".to_string() && type_suscription != "permanent".to_string() {
             return false;
        }

        // Verificar si el DAO es quien intenta minar el token, de lo contrario verificar las suscripciones
        if self.is_owner(&env::predecessor_account_id()){
            let mut new_token = TokenMetadata {
                title:  Some("".to_string()), 
                description:  Some("Este NFT sirve como pase al contenido de Open Web Academy".to_string()),
                media:  Some("".to_string()),
                expires_at: None,
                starts_at: None,
                copies: None,
                extra: None,
                issued_at: None,
                media_hash: None,
                reference: None,
                reference_hash: None,
                updated_at: None
            };

            let mut nft_image = "".to_string();

            if type_suscription == "one_month".to_string() {
                new_token.title = Some("Suscripción de 1 Mes".to_string());
                nft_image = COMMON.to_string();
            }
            if type_suscription == "six_months".to_string() {
                new_token.title = Some("Suscripción de 6 Meses".to_string());
                nft_image = RARE.to_string();
            }
            if type_suscription == "one_year".to_string() {
                new_token.title = Some("Suscripción de 1 Año".to_string());
                nft_image = SUPERRARE.to_string();
            }
            if type_suscription == "permanent".to_string() {
                new_token.title = Some("Suscripción Permanente".to_string());
                nft_image = LEGENDARY.to_string();
            }
    
            let initial_storage_usage = env::storage_usage();
    
            new_token.media = Some(nft_image);
            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
    
            // Agregar fechas de caducidad si la suscripción es diferente de permanente
            if type_suscription != "permanent".to_string() {
                let actual_epoch = env::block_timestamp();
                log!("actual_epoch {}",actual_epoch);
                // Fecha inicial
                new_token.starts_at = Some(actual_epoch);
                // Fecha expiración
                if type_suscription == "one_month".to_string() {
                    new_token.expires_at = Some(actual_epoch+2592000000000000);
                }
                if type_suscription == "six_months".to_string() {
                    new_token.expires_at = Some(actual_epoch+15552000000000000);
                }
                if type_suscription == "one_year".to_string() {
                    new_token.expires_at = Some(actual_epoch+31536000000000000);
                }
            }
    
            // create a royalty map to store in the token
            let mut royalty = HashMap::new();
    
            //specify the token struct that contains the owner ID 
            let token = Token {
                //set the owner ID equal to the receiver ID passed into the function
                owner_id: receiver_id,
                //we set the approved account IDs to the default value (an empty map)
                approved_account_ids: Default::default(),
                //the next approval ID is set to 0
                next_approval_id: 0,
                //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
                royalty,
            };
    
            //insert the token ID and token struct and make sure that the token doesn't exist
            assert!(
                self.tokens_by_id.insert(&token_id, &token).is_none(),
                "Token already exists"
            );
    
            //insert the token ID and metadata
            self.token_metadata_by_id.insert(&token_id, &new_token);
    
            //call the internal method for adding the token to the owner
            self.internal_add_token_to_owner(&token.owner_id, &token_id);
    
            //calculate the required storage which was the used - initial
            let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
    
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(required_storage_in_bytes);
    
            return true;
        } else {
            // Validar que el ususuario tenga suscripciones pendientes de canjear
            let signer_id = env::signer_account_id();
            let user_counter = self.tokens_to_mint_counter.get(&signer_id.clone());
            
            if user_counter.is_none() {
                return false;
            }
    
            // Existe registro de los contadores
            let mut user_counter_data = TokensCounter {
                canbuy: user_counter.clone().unwrap().canbuy,
                permanent: user_counter.clone().unwrap().permanent,
                one_year: user_counter.clone().unwrap().one_year,
                six_months: user_counter.clone().unwrap().six_months,
                one_month: user_counter.clone().unwrap().one_month
            };
    
            let mut new_token = TokenMetadata {
                title:  Some("".to_string()), 
                description:  Some("Este NFT sirve como pase al contenido de Open Web Academy".to_string()),
                media:  Some("".to_string()),
                expires_at: None,
                starts_at: None,
                copies: None,
                extra: None,
                issued_at: None,
                media_hash: None,
                reference: None,
                reference_hash: None,
                updated_at: None
            };
    
            let mut nft_image = "".to_string();

            // Verificar si tiene suscripción pendiente del tipo que está intentando mintear
            if type_suscription == "one_month".to_string() {
                if user_counter_data.one_month > 0 { user_counter_data.one_month -= 1; new_token.title = Some("Suscripción de 1 Mes".to_string()); } else { log!("No tiene suscripcion pendiente de: {}", type_suscription.clone()); return false; }
                nft_image = COMMON.to_string();
            }
            if type_suscription == "six_months".to_string() {
                if user_counter_data.six_months > 0 { user_counter_data.six_months -= 1; new_token.title = Some("Suscripción de 6 Meses".to_string()); } else { log!("No tiene suscripcion pendiente de: {}", type_suscription.clone()); return false; }
                nft_image = RARE.to_string();
            }
            if type_suscription == "one_year".to_string() {
                if user_counter_data.one_year > 0 { user_counter_data.one_year -= 1; new_token.title = Some("Suscripción de 1 Año".to_string()); } else { log!("No tiene suscripcion pendiente de: {}", type_suscription.clone()); return false; }
                nft_image = SUPERRARE.to_string();
            }
            if type_suscription == "permanent".to_string() {
                if user_counter_data.permanent > 0 { user_counter_data.permanent -= 1; new_token.title = Some("Suscripción Permanente".to_string()); } else { log!("No tiene suscripcion pendiente de: {}", type_suscription.clone()); return false; }
                nft_image = LEGENDARY.to_string();
            }
    
            let initial_storage_usage = env::storage_usage();
    
            new_token.media = Some(nft_image);
            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
    
            // Agregar fechas de caducidad si la suscripción es diferente de permanente
            if type_suscription != "permanent".to_string() {
                let actual_epoch = env::block_timestamp();
                // Fecha inicial
                new_token.starts_at = Some(actual_epoch);
                // Fecha expiración
                if type_suscription == "one_month".to_string() {
                    new_token.expires_at = Some(actual_epoch+2592000000000000);
                }
                if type_suscription == "six_months".to_string() {
                    new_token.expires_at = Some(actual_epoch+15552000000000000);
                }
                if type_suscription == "one_year".to_string() {
                    new_token.expires_at = Some(actual_epoch+31536000000000000);
                }
            }
    
            // create a royalty map to store in the token
            let mut royalty = HashMap::new();
    
            //specify the token struct that contains the owner ID 
            let token = Token {
                //set the owner ID equal to the receiver ID passed into the function
                owner_id: receiver_id,
                //we set the approved account IDs to the default value (an empty map)
                approved_account_ids: Default::default(),
                //the next approval ID is set to 0
                next_approval_id: 0,
                //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
                royalty,
            };
    
            //insert the token ID and token struct and make sure that the token doesn't exist
            assert!(
                self.tokens_by_id.insert(&token_id, &token).is_none(),
                "Token already exists"
            );
    
            //insert the token ID and metadata
            self.token_metadata_by_id.insert(&token_id, &new_token);
    
            //call the internal method for adding the token to the owner
            self.internal_add_token_to_owner(&token.owner_id, &token_id);
    
            //calculate the required storage which was the used - initial
            let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
    
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(required_storage_in_bytes);
    
            // Restar el contador de suscripciones pendientes del usuario
            self.tokens_to_mint_counter.insert(&signer_id.clone(), &user_counter_data);
    
            return true;
        }
    }

    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::predecessor_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_id.as_str()
    }

}