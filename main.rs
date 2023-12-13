use std::process;
use std::str::FromStr;
use solana_client::thin_client;
use rand::Rng;
use {
    std::convert::TryInto,
    solana_program::{
        account_info::{
            next_account_info, AccountInfo
        },
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
    },
};

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    fn new() -> RGB {
        RGB {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    fn make_transaction(&mut self,sender_address: &str, receiver_address: &str, new_red: u8, new_green: u8, new_blue: u8) {
        if new_red <= 255 && new_green <= 255 && new_blue <= 255 {
            self.red = new_red;
            self.green = new_green;
            self.blue = new_blue;
            println!("RGB values set successfully!");
            println!("RGB values: {}, {}, {}", self.red, self.green, self.blue);
        } else {
            println!("invalid RGB values.");
            return;
        }

        // this is the function for the payments
        let lamports_to_send = 1000000; // Convert amount to lamports

        // Check if the sender_address and receiver_address are valid public keys
        let sender_pubkey = Pubkey::from_str(sender_address).expect("Invalid sender address");
        let receiver_pubkey = Pubkey::from_str(receiver_address).expect("Invalid receiver address");

        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(&sender_pubkey, &receiver_pubkey, lamports_to_send);

        // Invoke the transfer instruction
        let mut accounts = vec![];
        invoke(&transfer_instruction, &accounts).expect("Failed to send transaction");

        msg!("Transaction successful!");
      }
}

fn main() {
    let mut rgb = RGB::new();
    let mut sender_address = "SENDER ADDRESS";
    let mut receiver_address = "GyWTdbYfysHUXTpQEA3n6J2U4cRdd7Q1KCY9gcbPKfZn";
    rgb.make_transaction(sender_address , receiver_address , 120, 20, 50);
}
