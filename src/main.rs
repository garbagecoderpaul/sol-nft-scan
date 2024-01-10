use borsh::BorshDeserialize;
use image::GenericImageView;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, program_pack::Pack};
use structopt::StructOpt;

type Error = Box<dyn std::error::Error>;

#[derive(StructOpt)]
struct Arguments {
    #[structopt(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    node_uri: String,
    mint_address: Pubkey,
}

fn main() -> Result<(), Error> {
    let args = Arguments::from_args();

    // RPC
    let client = RpcClient::new(args.node_uri);

    // Mint Info
    let mint_account = client
        .get_account(&args.mint_address)
        .map_err(|e| format!("Failed to get mint account: {:?}", e))?;
    let mint = spl_token::state::Mint::unpack(&mint_account.data)
        .map_err(|_| "Account is not a Token Program Mint")?;

    println!("Mint account details:");
    println!("  Owner: {}", mint_account.owner);
    println!("  Supply (1 for NFT): {}", mint.supply);
    println!("  Decimals (0 for NFT): {}", mint.decimals);
    println!(
        "  Authority: {}",
        mint.mint_authority
            .map(|p| p.to_string())
            .unwrap_or_else(|| "(none)".to_string())
    );
    println!();

    // Master Edition Info
    let (master_address, _bump) =
        mpl_token_metadata::pda::find_master_edition_account(&args.mint_address);
    let master_account = client
        .get_account(&master_address)
        .map_err(|e| format!("Failed to get master edition account: {:?}", e))?;
    let master =
        mpl_token_metadata::state::MasterEditionV2::try_from_slice(&master_account.data)
            .map_err(|_| "Master edition account not deserializable")?;
    println!("Master edition account details:");
    println!(
        "  Max Supply: {}",
        master
            .max_supply
            .map(|s| s.to_string())
            .unwrap_or_else(|| "(none)".to_string())
    );
    println!("  Supply: {}", master.supply);
    println!();

    Ok(())
}
