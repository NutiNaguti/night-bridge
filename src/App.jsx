import React, { useEffect, useState } from 'react';
import { connect, WalletConnection, utils, Contract } from 'near-api-js';
import { getConfig } from './config';

const {
  format: { formatNearAmount },
} = utils;

const App = () => {
  const [wallet, setWallet] = useState(null);
  const [contract, setContract] = useState(null);
  const [counter, setCounter] = useState(0);
  const [deposit, setDeposit] = useState(0);
  const [balance, setBalance] = useState('');

  // Establish a connection to the NEAR blockchain on component mount
  useEffect(() => {
    connect(getConfig()).then((near) => setWallet(new WalletConnection(near)));
  }, []);

  // Initialize the contract object when the wallet is available
  useEffect(() => {
    if (wallet) {
      setContract(
        new Contract(wallet.account(), 'counter.testnet', {
          viewMethods: ['getCounter'],
          changeMethods: [
            'resetCounter',
            'incrementCounter',
            'decrementCounter',
          ],
        })
      );

      // We can get the account balance of a user through the wallet
      // Since this is requesting data from the blockchain, the method returns a Promise
      wallet
        .account()
        .getAccountBalance()
        .then(({ available }) => setBalance(available));
    }
  }, [wallet]);

  const isSignedIn = Boolean(wallet && wallet.isSignedIn() && contract);

  // Update the counter value when the contract is available
  // (which means that the user is signed in and the contract has been initialized)
  // Calling contract functions is similar to calling API endpoints in traditional web apps
  // The call happens asynchronously and the result is returned in a Promise
  useEffect(() => {
    if (isSignedIn) {
      contract.getCounter().then((counter) => {
        setCounter(counter);
      });
    }
  }, [contract, isSignedIn]);

  // Handle the sign in call by requesting a sign in through the NEAR Wallet
  const handleLogin = () => {
    wallet.requestSignIn({
      contractId: 'counter.testnet',
      methodNames: [
        'resetCounter',
        'incrementCounter',
        'decrementCounter',
        'getCounter',
      ],
    });
  };

  const handleReset = async () => {
    // Call the reset function on the counter contract
    // We have to deposit at least one yoctoNEAR (1e-24 NEAR) to be able to call change functions
    await contract.resetCounter({
      args: {},
      amount: deposit.toFixed(0),
    });

    // When calling view functions, we don't need to deposit
    setCounter(await contract.getCounter());
  };

  const handleIncrement = async () => {
    // Call the increment function on the counter contract
    await contract.incrementCounter({
      args: { value: 1 },
      amount: deposit.toFixed(0),
    });

    setCounter(await contract.getCounter());
  };

  const handleDecrement = async () => {
    // Call the decrement function on the counter contract
    await contract.decrementCounter({
      args: { value: 1 },
      amount: deposit.toFixed(0),
    });

    setCounter(await contract.getCounter());
  };

  return (
    <section>
      <h1>🎉 Congrats on starting your NEAR journey in React! 🎉</h1>
      {/* Only show the sign in button when the user is not signed in */}
      {isSignedIn ? (
        <div>
          {/* We can get the account id of the currently signed in user through the wallet */}
          <div>Hi, {wallet.getAccountId()}!</div>
          <p>
            Your account ballance is{' '}
            {/* The balance will be retrieved in yoctoNEAR so we have to format it to a NEAR amount */}
            <strong>{formatNearAmount(balance, 4)}</strong>
          </p>
          <p>
            The current value of the counter is: <strong>{counter}</strong>
          </p>
          <label htmlFor="deposit">
            <span>Deposit value (in yoctoNEAR): </span>
            <input
              id="deposit"
              type="number"
              min={1}
              value={deposit}
              onChange={({ target: { value } }) => setDeposit(parseInt(value))}
            />
          </label>
          <div
            style={{ display: 'flex', flexDirection: 'column', width: '50%' }}
          >
            <button onClick={() => handleReset()}>Reset Counter</button>
            <button onClick={() => handleIncrement()}>Increment counter</button>
            <button onClick={() => handleDecrement()}>Decrement counter</button>
          </div>
        </div>
      ) : (
        <div>
          <button onClick={() => handleLogin()}>Login with NEAR</button>
        </div>
      )}
    </section>
  );
};

export default App;
