import React from 'react'
import { invoke } from '@tauri-apps/api/tauri'

import { BaseViewGrid, TopbarViewGrid } from './layout';

function invoke_ping_rpc() {
    invoke('ping_rpc');
}

export const StartupView = () => {
    return (
	<div>
          <BaseViewGrid Top={<h1>123</h1>} Middle={(1)} Bottom={(2)} />
	</div>
    );
}
