use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Coin,
};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{NftInfo, State, STATE, NFTS};

const FEE_PERCENT: u64 = 2; // 2%

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
        fee_accumulator: Uint128::zero(),
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintNft { token_id, price } => try_mint_nft(deps, info, token_id, price),
        ExecuteMsg::TransferNft { token_id, to } => try_transfer_nft(deps, info, token_id, to),
        ExecuteMsg::BuyNft { token_id } => try_buy_nft(deps, env, info, token_id),
    }
}

pub fn try_mint_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    price: Uint128,
) -> Result<Response, ContractError> {
    let nft = NftInfo {
        owner: info.sender.clone(),
        price,
    };
    NFTS.save(deps.storage, &token_id, &nft)?;
    Ok(Response::new()
        .add_attribute("method", "mint_nft")
        .add_attribute("token_id", token_id)
        .add_attribute("owner", info.sender.to_string()))
}

pub fn try_transfer_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    to: String,
) -> Result<Response, ContractError> {
    let mut nft = NFTS.load(deps.storage, &token_id)?;
    if nft.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    nft.owner = deps.api.addr_validate(&to)?;
    NFTS.save(deps.storage, &token_id, &nft)?;
    Ok(Response::new()
        .add_attribute("method", "transfer_nft")
        .add_attribute("token_id", token_id)
        .add_attribute("new_owner", to))
}

pub fn try_buy_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let mut nft = NFTS.load(deps.storage, &token_id)?;
    let state = STATE.load(deps.storage)?;

    let payment_amount = info.funds.iter().find(|coin| coin.denom == "ujuno").map(|coin| coin.amount).unwrap_or(Uint128::zero());
    if payment_amount < nft.price {
        return Err(ContractError::InsufficientFunds {});
    }

    let fee = nft.price.multiply_ratio(FEE_PERCENT, 100);
    let seller_amount = nft.price.checked_sub(fee)?;

    let seller_payment = BankMsg::Send {
        to_address: nft.owner.to_string(),
        amount: vec![Coin {
            denom: "ujuno".to_string(),
            amount: seller_amount,
        }],
    };

    let mut new_state = state.clone();
    new_state.fee_accumulator += fee;
    STATE.save(deps.storage, &new_state)?;

    nft.owner = info.sender.clone();
    NFTS.save(deps.storage, &token_id, &nft)?;

    Ok(Response::new()
        .add_message(seller_payment)
        .add_attribute("method", "buy_nft")
        .add_attribute("token_id", token_id)
        .add_attribute("buyer", info.sender.to_string())
        .add_attribute("fee_collected", fee.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNftInfo { token_id } => to_binary(&query_nft_info(deps, token_id)?),
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
    }
}

fn query_nft_info(deps: Deps, token_id: String) -> StdResult<NftInfo> {
    let nft = NFTS.load(deps.storage, &token_id)?;
    Ok(nft)
}

fn query_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage)?;
    Ok(state)
}
