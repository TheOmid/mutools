import React from 'react'
import { invoke } from '@tauri-apps/api/tauri'

function invoke_ping_rpc() {
    invoke('ping_rpc');
}

export const StartupView = () => {
    return (
	<div>
	    <button onClick={invoke_ping_rpc}>Send RPC!</button>
	</div>
    );
}
