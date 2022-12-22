import * as nearAPI from "near-api-js";
const { connect, WalletConnection, keyStores } = nearAPI;

const connectionConfig = {
  networkId: "testnet",
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
}

const nearConnection = await connect(connectionConfig);

const walletConnection = new WalletConnection(nearConnection);

export function signInNearWallet() {
  walletConnection.requestSignIn("dev-1669803549073-25511761548859", "Night Bridge");
}

export function signOutNearWaller() {
  walletConnection.signOut();
}

export function isNearWalletSignedIn() {
  return walletConnection.isSignedIn();
}
