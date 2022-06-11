import React from 'react'
import styled from "styled-components";
import { invoke } from '@tauri-apps/api/tauri'
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';

import { NavbarLayout } from './navbar.tsx';

function invoke_ping_rpc() {
    invoke('ping_rpc');
}

const AppContainer = styled.div
`
width: 100vw;
height: 100vh;
`;


export const BaseLayout = (props: any) => {
    const { Navbar, Toolbar, Dashboard } = props;
    
    return (
	<Box sx={{ flexGrow: 1, height: '100vh', width: '100vw' }}>
	  <Grid container
                spacing={1}
                direction="column"
                justifyContent="space-evenly"
                alignItems="stretch"
          >

            <Grid item xs={12}>
              <Box sx={{ height: '4vh' }}>
                { Navbar }
              </Box>
            </Grid>

            <Grid item xs={12}>
              <Box sx={{ height: '38vh' }}>
                { Toolbar }
              </Box>
            </Grid>

            <Grid item xs={12}>
              <Box sx={{ height: '60vh' }}>
                { Dashboard }
              </Box>
            </Grid>

	  </Grid>
	</Box>
    );
};

export const AppView = () => {

    const Navbar = <NavbarLayout
                     C1={"Logo"}
                     C2={"File"}
                     C3={"Edit"}
                     C4={"Tools"}
                     C5={"Search"}
                   />;
    
    return (
	<AppContainer>
          <BaseViewGrid
            Navbar={Navbar}
            Toolbar={(<h3>hi</h3>)}
            Dashboard={(<h2> hello </h2>)}
          />
	</AppContainer>
    );
}
