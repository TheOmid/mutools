import React from 'react'
import Grid from '@mui/material/Grid';

export const BaseViewGrid = (props) => {
    const { Top, Middle, Bottom } = props;
    
    return (
	<Box sx={{ flexGrow: 1 }}>
	  <Grid container spacing={2}>

            <Grid item xs={8}>
              <Top />
            </Grid>

            <Grid item xs={4}>
              <Middle />
            </Grid>

            <Grid item xs={4}>
              <Bottom />
            </Grid>

	  </Grid>
	</Box>
    );
};

export const TopbarViewGrid = (props) => {
    const { C1, C2, C3, C4, C5 } = props;

    return ( 
        <Box sx={{ flexGrow: 1 }}>
          <Grid container>

            <Grid item xs={2}>
              <C1 />
            </Grid>

            <Grid item xs={2}>
              <C2 />
            </Grid>

            <Grid item xs={2}>
              <C3 />
            </Grid>

            <Grid item xs={2}>
              <C4 />
            </Grid>
            
            <Grid item xs={2}>
              <C5 />
            </Grid>            
            
          </Grid>
        </Box>
    );
}
