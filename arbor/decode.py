import base64
from solana.rpc.async_api import AsyncClient
from solana.publickey import PublicKey
from solana.rpc.types import TokenAccountOpts
from construct import Struct, Int64ul, Bytes, Array, Adapter, Int64ul
import asyncio

TOKEN_PROGRAM_ID = PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")

# Define a custom Int128ul adapter


class Int128ul(Adapter):
    def _decode(self, obj, context, path):
        return obj[0] + (obj[1] << 64)

    def _encode(self, obj, context, path):
        return [obj & 0xFFFFFFFFFFFFFFFF, obj >> 64]


Int128ul = Int128ul(Int64ul[2])

# Define the LIQUIDITY_STATE_LAYOUT_V4 structure
LIQUIDITY_STATE_LAYOUT_V4 = Struct(
    "status" / Int64ul,
    "nonce" / Int64ul,
    "maxOrder" / Int64ul,
    "depth" / Int64ul,
    "baseDecimal" / Int64ul,
    "quoteDecimal" / Int64ul,
    "state" / Int64ul,
    "resetFlag" / Int64ul,
    "minSize" / Int64ul,
    "volMaxCutRatio" / Int64ul,
    "amountWaveRatio" / Int64ul,
    "baseLotSize" / Int64ul,
    "quoteLotSize" / Int64ul,
    "minPriceMultiplier" / Int64ul,
    "maxPriceMultiplier" / Int64ul,
    "systemDecimalValue" / Int64ul,
    "minSeparateNumerator" / Int64ul,
    "minSeparateDenominator" / Int64ul,
    "tradeFeeNumerator" / Int64ul,
    "tradeFeeDenominator" / Int64ul,
    "pnlNumerator" / Int64ul,
    "pnlDenominator" / Int64ul,
    "swapFeeNumerator" / Int64ul,
    "swapFeeDenominator" / Int64ul,
    "baseNeedTakePnl" / Int64ul,
    "quoteNeedTakePnl" / Int64ul,
    "quoteTotalPnl" / Int64ul,
    "baseTotalPnl" / Int64ul,
    "poolOpenTime" / Int64ul,
    "punishPcAmount" / Int64ul,
    "punishCoinAmount" / Int64ul,
    "orderbookToInitTime" / Int64ul,
    "swapBaseInAmount" / Int128ul,
    "swapQuoteOutAmount" / Int128ul,
    "swapBase2QuoteFee" / Int64ul,
    "swapQuoteInAmount" / Int128ul,
    "swapBaseOutAmount" / Int128ul,
    "swapQuote2BaseFee" / Int64ul,
    "baseVault" / Bytes(32),
    "quoteVault" / Bytes(32),
    "baseMint" / Bytes(32),
    "quoteMint" / Bytes(32),
    "lpMint" / Bytes(32),
    "openOrders" / Bytes(32),
    "marketId" / Bytes(32),
    "marketProgramId" / Bytes(32),
    "targetOrders" / Bytes(32),
    "withdrawQueue" / Bytes(32),
    "lpVault" / Bytes(32),
    "owner" / Bytes(32),
    "lpReserve" / Int64ul,
    "padding" / Array(3, Int64ul)
)


async def get_token_accounts(connection, owner):
    opts = TokenAccountOpts(program_id=TOKEN_PROGRAM_ID)
    resp = await connection.get_token_accounts_by_owner(owner, opts)
    accounts = []
    for token_account in resp['result']['value']:
        pubkey = token_account['pubkey']
        account_data = base64.b64decode(token_account['account']['data'][0])
        accounts.append({
            'pubkey': pubkey,
            'accountInfo': account_data
        })
    return accounts


async def parse_pool_info():
    connection = AsyncClient("https://api.mainnet-beta.solana.com")
    owner = PublicKey("VnxDzsZ7chE88e9rB6UKztCt2HUwrkgCTx8WieWf5mM")

    token_accounts = await get_token_accounts(connection, owner)

    # Get pool info
    sol_usdc_pool_id = PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")
    pool_info_resp = await connection.get_account_info(sol_usdc_pool_id)
    if pool_info_resp['result']['value']:
        pool_data = base64.b64decode(pool_info_resp['result']['value']['data'][0])
        parsed_pool_info = LIQUIDITY_STATE_LAYOUT_V4.parse(pool_data)
        print(parsed_pool_info)
    else:
        print("No pool info found")

    await connection.close()

asyncio.run(parse_pool_info())
