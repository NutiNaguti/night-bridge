import React, { useEffect, useRef, useState } from "react";
import { connect, WalletConnection, utils, Contract } from "near-api-js";
import { ethers } from "ethers";
import { getConfig } from "./config";
import "./App.css";

const ethereum = window.ethereum;

const abi = [
  "function lock(uint256) external returns (bytes32)"
]

const {
  format: { formatNearAmount },
} = utils;

const App = () => {
  const [errorMessage, setErrorMessage] = useState(null);

  const [walletNear, setWalletNear] = useState(null);
  const [contractNear, setContractNear] = useState(null);

  const [accountEth, setAccountEth] = useState(null);
  const [haveMetamask, setHaveMetamask] = useState(true);
  const [meatamaskConnected, setMetamaskConnected] = useState(false);

  const amountRef = useRef(null);
  const addressRef = useRef(null);

  useEffect(() => {
    const checkMetamaskAvailability = async () => {
      if (!ethereum) {
        setHaveMetamask(false);
      }
      setHaveMetamask(true);
    };
    checkMetamaskAvailability();
  }, []);


  // Establish a connection to the NEAR blockchain on component mount
  useEffect(() => {
    connect(getConfig()).then((near) =>
      setWalletNear(new WalletConnection(near))
    );
  }, []);

  // Initialize the contract object when the wallet is available
  useEffect(() => {
    if (walletNear) {
      setContractNear(
        new Contract(walletNear.account(), "crystal.bridge.testnet", {
          viewMethods: ["test"],
          changeMethods: [],
        })
      );
    }
  }, [walletNear]);

  const isSignedIn = Boolean(
    walletNear && walletNear.isSignedIn() && contractNear
  );

  // Handle the sign in call by requesting a sign in through the NEAR Wallet
  const handleLogin = () => {
    walletNear.requestSignIn({
      contractId: "dev-1669636776340-19796264538943",
    });
  };

  const connectMetamask = async () => {
    const { ethereum } = window;
    if (!ethereum) {
      setHaveMetamask(false);
    }

    const accounts = await ethereum.request({
      method: 'eth_requestAccounts'
    })
    setAccountEth(accounts[0]);
    setMetamaskConnected(true);
  }

  const lockTokens = async () => {
    const provider = new ethers.providers.Web3Provider(ethereum);
    const walletAddress = accountEth;
    const signer = provider.getSigner(walletAddress);
    const BridgeContract = new ethers.Contract("0x9431f9bba577B037D97ad6F7086a00eFB572c871", abi, signer);
    const tx = await BridgeContract.lock(ethers.BigNumber.from(amountRef.current.value));
    const receipt = await tx.wait();
    console.log("topic: ", receipt.logs[2].topics[1]);
    console.log("blockNumber: ", receipt.logs[2].blockNumber);
    console.log("txHash: ", receipt.logs[2].transactionHash);

    await completeTransfer(receipt.logs[2].blockNumber, receipt.logs[2].topics[1], addressRef.current.value);
  }

  const completeTransfer = async (blockNumber, topic, address) => {
    //TODO implement near transfer
  }

  const handleSend = async () => {
    console.log(amountRef.current.value);
    console.log(addressRef.current.value);

    await lockTokens();
  }

  return (
    <div className="App">
      <header className="AppHeader">
        <h1>Crystal Bridge</h1>
        <button className="navbar-btn" onClick={() => connectMetamask()}>Login ETH</button>
        <button className="navbar-btn" onClick={() => handleLogin()}>Login NEAR</button>
      </header>

      <div>
        <input id="text" ref={amountRef} />&nbsp;&#8212;&nbsp;<span>Amount</span>
      </div>
      <div>
        <input id="text" ref={addressRef} />&nbsp;&#8212;&nbsp;<span>NEAR Address</span>
      </div>
      <div>
      </div>
      <button className="Button" onClick={() => handleSend()}>Send</button>
    </div>
  );
};

export default App;
