import React, { useEffect, useRef, useState } from "react";
import { connect, WalletConnection, utils, Contract } from "near-api-js";
import { ethers } from "ethers";
import { getConfig } from "./config";
import "./App.css";

const {
  format: { formatNearAmount },
} = utils;

const accountChange = () => { };

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
    const { ethereum } = window;
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

  function handleSend() {
    console.log(amountRef.current.value);
    console.log(addressRef.current.value);
  }

  return (
    <div className="App">
      <header className="AppHeader">
        <button className="navbar-btn" onClick={() => connectMetamask()}>Login ETH</button>
        <button className="navbar-btn" onClick={() => handleLogin()}>Login NEAR</button>
      </header>

      <div>
        <input id="text" ref={amountRef} />
      </div>
      <div>
        <input id="text" ref={addressRef} />
      </div>
      <div>
      </div>
      <button className="Button" onClick={() => handleSend()}>Send</button>
      <button className="Button">Get</button>
    </div>
  );
};

export default App;
