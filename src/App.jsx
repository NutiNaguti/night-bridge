import logo from './bridge.svg';
import styles from './App.module.css';
import { isNearWalletSignedIn, signInNearWallet, signOutNearWaller } from './scripts/nearWallet';
import { createSignal, onMount } from 'solid-js';

function App() {
  onMount(async () => {
    if (!isNearWalletSignedIn()) {
      signInNearWallet();
    }
  });

  return (
    <>
      <div class={styles.App}>
        <header class={styles.header}>
          <div>
            <img src={logo} class={styles.logo} alt="logo" />
            <h1> Night Bridge </h1>
          </div>
        </header>
        <div class={styles.content}>
          <h3>Send 100 FunCoin from Ethereum to NEAR</h3>
          <div class={styles.input}>
            <label for='address'>NEAR address</label>
            <input id={styles.address}></input>
          </div>
          <button class={styles.button} >Send</button>
        </div>
        <footer class={styles.footer}>
          <a href='https://github.com/NutiNaguti'>&#9001;NutiNaguti&nbsp;</a>
          <a href=''>&#9001;Source Code</a>
        </footer>
      </div>

    </>
  );
}

export default App;
