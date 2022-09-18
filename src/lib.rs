// counter to increment, decrement, get_state and rest to 0

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador
{
    valor: i8,
}

#[near_bindgen]
impl Contador
{
    //get_state
    pub fn get_num(&self) -> i8
    {
        return self.valor;
    }

    //increment
    pub fn increment(&mut self)
    {
        self.valor += 1;
        let log_message = format!("value incremented to {}",self.valor);
        env::log( log_message.as_bytes() );
        after_change()
    }

    // decrement
    pub fn decrement(&mut self)
    {
        self.valor -= 1;
        let log_message = format!("value decremented to {}",self.valor);
        env::log( log_message.as_bytes() );
        after_change()
    }

    //reset
    pub fn reset(&mut self)
    {
        self.valor = 0;
        let log_message = format!("value reset to {}",self.valor);
        env::log( log_message.as_bytes() );
        after_change()
    }

}

fn after_change()
{
    env::log( "Value changed".as_bytes() );
}



#[cfg(test)]
mod tests 
{
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Preparación del contaxto de la Blockchain virtual
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext
    {
        VMContext
        {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0,1,2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    
    // Nuestras pruebas unitarias

    #[test]
    fn increment() 
    {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contador{ valor: 0};

        contract.increment();

        println!("Valor despues del incremento: {}", contract.valor);
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() 
    {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contador{ valor: 0};

        contract.decrement();

        println!("Valor despues del incremento: {}", contract.valor);
        assert_eq!(-1, contract.get_num());
    }


    #[test]
    fn reset() 
    {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contador{ valor: 10};
        assert_eq!(10, contract.get_num());
        println!("Valor Antes del reset: {}", contract.valor);

        contract.reset();

        println!("Valor despues del reset: {}", contract.valor);
        assert_eq!(0, contract.get_num());
    }
}