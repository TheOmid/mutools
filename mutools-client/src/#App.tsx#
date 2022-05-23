import React from 'react';
import logo from './logo.svg';
import './App.css';

import { invoke } from '@tauri-apps/api/tauri'

function DoPing() {
    console.log("Pinging...");
    invoke('DispatchPing')
	.then((res) => console.log("RESULT: " + res))
	.catch((err) => console.log(err));
}

function App() {
    return (
	<div className="App">
	    <header className="App-header">
	    <img src={logo} className="App-logo" alt="logo" />

	    <button onClick={DoPing}> Ping Mutools Server! </button>

	    </header>
	</div>
    );
}

export default App;
