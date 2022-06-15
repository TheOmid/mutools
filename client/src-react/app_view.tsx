import React from 'react';
import styled from 'styled-components';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';

import ColorScheme from './color_scheme';

import Navbar from './navbar';
import Toolbar from './toolbar';
import Dashboard from './dashboard';

function AppViewMain() {
  return (
      <Box sx={{ height: '100%', width: '100%' }}>
      <Grid
        container
        spacing={0}
        direction="row"
      >

        <Grid item xs={12}>
          <Box sx={{ height: '4%' }}>
            { Navbar }
          </Box>
        </Grid>

        <Grid item xs={12}>
          <Box sx={{ height: '2%' }}>
            <Grid
              container
              spacing={0}
              direction="row"
            >
              <Grid item xs={8}>
                <Box>
                  { Dashboard }
                </Box>
              </Grid>

              <Grid item xs={4}>
                <Box>
                  { Toolbar }
                </Box>
              </Grid>

            </Grid>
          </Box>
        </Grid>
      </Grid>
    </Box>
  );
}

export default function AppView() {
  return (
      <Box sx={{ height: "100vh",
                 width: "100vw",
                 margin: "0 0 0 0",
                 display: "flex",
                 top: 0,
                 bottom: 0,
                 left: 0,
                 //right: 0,
                 bgcolor: ColorScheme.base_background}}
          >
          <AppViewMain />
      </Box>
  );
}
