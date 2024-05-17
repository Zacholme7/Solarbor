import requests
import json


def get_multiple_accounts(pubkeys, endpoint="https://mainnet.helius-rpc.com/?api-key=4df1ba34-ef23-4193-8bc0-e25bb4380d26", encoding="jsonParsed"):
    headers = {
        "Content-Type": "application/json"
    }

    # Define params for getting multiple accounts
    params = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getMultipleAccounts",
        "params": [
            pubkeys,
            {
                "encoding": encoding
            }
        ]
    }

    response = requests.post(endpoint, headers=headers, data=json.dumps(params))

    if response.status_code == 200:
        result = response.json()
        return result.get("result", {}).get("value", [])
    else:
        print(f"Error: {response.status_code}")
        print(response.text)
        return None


if __name__ == "__main__":
    pubkeys = [
        "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2"
    ]
    accounts_info = get_multiple_accounts(pubkeys)

    if accounts_info is not None:
        for account in accounts_info:
            if account is None:
                print("Account does not exist")
            else:
                print(json.dumps(account, indent=4))
    else:
        print("No accounts found.")
