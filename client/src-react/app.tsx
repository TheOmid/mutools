import React from 'react'
import { invoke } from '@tauri-apps/api/tauri'

import { BaseViewGrid, TopbarViewGrid } from './layout';

function invoke_ping_rpc() {
    invoke('ping_rpc');
}

export const AppView = () => {

    const Topbar = <TopbarViewGrid
                     C1={"Logo"}
                     C2={"File"}
                     C3={"Edit"}
                     C4={"Help"}
                     C5={"Search Bar"}
                   />;
    
    return (
	<div>
          <BaseViewGrid
            Top={Topbar}
            Middle={(<h3>hi</h3>)}
            Bottom={(<h2> hello </h2>)}
          />
	</div>
    );
}
